use std::{collections::BTreeMap, fmt::Debug, fs::File, path::PathBuf};

use anyhow::{Error, Result};
use vidmod_node::{Frame, Node, PullPort, PushPort, TickNode};
use vidmod_plugin::Plugin;

use self::manifest::ProjectManifest;

mod manifest;

lazy_static! {
    static ref PLUGINS: BTreeMap<String, Plugin> = {
        let mut res = BTreeMap::new();
        res.insert(
            "vidmod-plugins-cvbs::SyncExtractor".to_owned(),
            vidmod_plugins_cvbs::plugin::SYNC_EXTRACTOR,
        );
        res.insert(
            "vidmod-plugins-core::RawFileSource".to_owned(),
            vidmod_plugins_core::plugin::RAW_FILE_SOURCE,
        );
        res.insert(
            "vidmod-plugins-core::RawFileSink".to_owned(),
            vidmod_plugins_core::plugin::RAW_FILE_SINK,
        );
        res
    };
}

#[derive(Debug)]
pub struct Project {
    nodes: NodeGraph,
}

impl Project {
    pub fn load(f: File, path: PathBuf) -> Self {
        let manifest: manifest::ProjectManifest = serde_yaml::from_reader(f).unwrap();
        Project::from_manifest(manifest, path)
    }

    pub fn tick(&mut self) -> bool {
        self.nodes.tick()
    }

    fn from_manifest(manifest: ProjectManifest, path: PathBuf) -> Self {
        let mut graph = NodeGraph::new();

        let mut node_map = BTreeMap::new();

        for (name, mut node) in manifest.nodes {
            node.args.insert(
                "vidmod.path".to_string(),
                path.to_str().unwrap().to_string(),
            );

            let plugin = PLUGINS
                .get(&node.name)
                .unwrap_or_else(|| panic!("Unknown plugin {}", node.name));
            let mut node = (plugin.make_node)(node.args);
            node.init();
            let id = graph.insert(node);
            node_map.insert(name, id);
        }
        for link in manifest.links {
            let p1 = graph
                .get_pull_port(*node_map.get(&link.from.0).unwrap(), &link.from.1)
                .unwrap();
            let p2 = graph
                .get_push_port(*node_map.get(&link.to.0).unwrap(), &link.to.1)
                .unwrap();
            graph.add_link(p1, p2).unwrap();
        }

        Self { nodes: graph }
    }
}

#[derive(Debug)]
pub struct NodeGraph {
    nodes: Vec<Node>,
    links: Vec<(PullPort, PushPort)>,
}

impl NodeGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn insert(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn get_pull_port(&mut self, id: usize, name: &str) -> Result<PullPort> {
        match &self.nodes[id] {
            Node::Source(n) => n.get_pull_port(id, name),
            Node::Intermediate(n) => n.get_pull_port(id, name),
            Node::Sink(_) => Err(Error::msg("Can't get pull port of output node")),
            Node::Null => Err(Error::msg("Can't get pull port of null node")),
            Node::N2(n) => n.get_pull_port(id, name),
        }
    }

    pub fn get_push_port(&mut self, id: usize, name: &str) -> Result<PushPort> {
        match &self.nodes[id] {
            Node::Intermediate(n) => n.get_push_port(id, name),
            Node::Sink(n) => n.get_push_port(id, name),
            Node::Source(_) => Err(Error::msg("Can't get push port of input node")),
            Node::Null => Err(Error::msg("Can't get push port of null node")),
            Node::N2(n) => n.get_push_port(id, name),
        }
    }

    pub fn add_link(&mut self, p1: PullPort, p2: PushPort) -> Result<()> {
        let p1i = p1.id();
        let p1n = p1.name();
        let p2i = p2.id();
        let p2n = p2.name();
        match &self.nodes[p1i] {
            Node::Source(n) => n.attach_push_port(p1n, p2.clone())?,
            Node::Intermediate(n) => n.attach_push_port(p1n, p2.clone())?,
            Node::N2(n) => n.attach_push_port(p1n, p2.clone())?,
            _ => panic!(),
        }
        match &self.nodes[p2i] {
            Node::Sink(n) => n.attach_pull_port(p2n, p1.clone())?,
            Node::Intermediate(n) => n.attach_pull_port(p2n, p1.clone())?,
            Node::N2(n) => n.attach_pull_port(p2n, p1.clone())?,
            _ => panic!(),
        }

        self.links.push((p1, p2));
        Ok(())
    }

    pub fn tick(&mut self) -> bool {
        let mut res = false;
        for node in &mut self.nodes {
            res |= node.tick()
        }
        for (pull, push) in self.links.clone() {
            let pull_count = self.pull_ready(&pull);
            let push_count = self.push_ready(&push);
            let count = usize::min(pull_count, push_count);
            if count > 0 {
                let frame = self.pull_from(&pull, count);
                self.push_to(&push, frame);
                res = true;
            }
        }
        res
    }

    fn pull_ready(&self, p: &PullPort) -> usize {
        match &self.nodes[p.id()] {
            Node::Source(v) => v.ready_to_pull(p),
            Node::Intermediate(v) => v.ready_to_pull(p),
            Node::N2(v) => v.ready_to_pull(p),
            _ => unimplemented!(),
        }
    }
    fn push_ready(&self, p: &PushPort) -> usize {
        match &self.nodes[p.id()] {
            Node::Sink(v) => v.ready_to_push(p),
            Node::Intermediate(v) => v.ready_to_push(p),
            Node::N2(v) => v.ready_to_push(p),
            _ => unimplemented!(),
        }
    }

    fn pull_from(&mut self, port: &PullPort, count: usize) -> Frame {
        match &mut self.nodes[port.id()] {
            Node::Source(v) => v.pull_frame(port, count),
            Node::Intermediate(v) => v.pull_frame(port, count),
            Node::N2(v) => v.pull_frame(port, count),
            _ => unimplemented!(),
        }
    }

    fn push_to(&mut self, p: &PushPort, f: Frame) {
        match &mut self.nodes[p.id()] {
            Node::Sink(v) => v.push_frame(p, f),
            Node::Intermediate(v) => v.push_frame(p, f),
            Node::N2(v) => v.push_frame(p, f),
            _ => unimplemented!(),
        }
    }
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self::new()
    }
}

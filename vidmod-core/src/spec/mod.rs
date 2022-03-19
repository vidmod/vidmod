use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    fs::File,
    iter::FromIterator,
    path::PathBuf,
};

use anyhow::Result;
use vidmod_node::{frame::Frame, FinishNode, Node, PullPort, PushPort, TickNode};

use self::manifest::ProjectManifest;

mod manifest;

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

    pub fn run(&mut self) {
        self.nodes.run()
    }

    fn from_manifest(manifest: ProjectManifest, path: PathBuf) -> Self {
        let mut graph = NodeGraph::new();

        let mut node_map = BTreeMap::new();

        for (name, mut node) in manifest.nodes {
            node.args.insert(
                "vidmod.path".to_string(),
                path.to_str().unwrap().to_string(),
            );

            let plugin = vidmod_plugin::PLUGINS
                .get(&node.name)
                .unwrap_or_else(|| panic!("Unknown plugin {}", node.name));
            let mut node = (plugin.make_node)(node.args);
            node.init();
            let id = graph.insert(node, name.clone());
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
    nodes:      Vec<Node>,
    links:      Vec<(PullPort, PushPort)>,
    node_names: Vec<String>,
}

impl NodeGraph {
    pub fn new() -> Self {
        Self {
            nodes:      Vec::new(),
            links:      Vec::new(),
            node_names: Vec::new(),
        }
    }

    pub fn insert(&mut self, node: Node, name: String) -> usize {
        self.nodes.push(node);
        self.node_names.push(name);
        self.nodes.len() - 1
    }

    pub fn get_pull_port(&mut self, id: usize, name: &str) -> Result<PullPort> {
        self.nodes[id].0.get_pull_port(id, name)
    }

    pub fn get_push_port(&mut self, id: usize, name: &str) -> Result<PushPort> {
        self.nodes[id].0.get_push_port(id, name)
    }

    pub fn add_link(&mut self, p1: PullPort, p2: PushPort) -> Result<()> {
        let p1i = p1.id();
        let p1n = p1.name();
        let p2i = p2.id();
        let p2n = p2.name();
        self.nodes[p1i].0.attach_push_port(p1n, p2.clone())?;
        self.nodes[p2i].0.attach_pull_port(p2n, p1.clone())?;

        self.links.push((p1, p2));
        Ok(())
    }

    pub fn tick(&mut self) -> bool {
        self.tick_nodes(None) || self.tick_links()
    }

    pub fn tick_nodes(&mut self, nodes: Option<&BTreeSet<usize>>) -> bool {
        let mut res = false;
        for (idx, node) in self.nodes.iter_mut().enumerate() {
            if let Some(nodes) = &nodes {
                if !nodes.contains(&idx) {
                    continue;
                }
            }
            res |= node.tick();
        }
        res
    }

    pub fn tick_links(&mut self) -> bool {
        let mut res = false;
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

    pub fn run(&mut self) {
        let mut nodes = BTreeSet::from_iter(0..self.nodes.len());
        while {
            let mut progress = false;
            println!("Running nodes");
            while {
                let mut inner_progress = false;
                inner_progress |= self.tick_nodes(Some(&nodes));
                inner_progress |= self.tick_links();
                progress |= inner_progress;
                inner_progress
            } {
                //println!("Inner made progress!");
            }
            println!("Pruning nodes");
            let nodes_cur = nodes.clone();
            nodes = BTreeSet::new();
            for node in &nodes_cur {
                for (pull, push) in &self.links {
                    if &push.id() != node {
                        continue;
                    }
                    if !nodes_cur.contains(&pull.id()) {
                        continue;
                    }
                    nodes.insert(*node);
                    break;
                }
            }
            let to_prune = nodes_cur.difference(&nodes);
            println!(
                "Finishing nodes: {:?}",
                to_prune
                    .clone()
                    .map(|x| self.node_names.get(*x).unwrap())
                    .collect::<Vec<&String>>()
            );
            for node in to_prune {
                println!("Finishing node: {:?}", self.node_names.get(*node).unwrap());
                if !self.nodes[*node].finish() {
                    println!("  Running to allow finish");
                    while self.tick_nodes(Some(&nodes_cur)) || self.tick_links() {
                        println!("   Inner made progress!");
                    }
                } else {
                    println!("  Immediate finish allowed");
                }
                progress = true;
            }
            progress
        } {
            println!("Outer made progress!");
        }
        println!("Done!");
    }

    fn pull_ready(&self, p: &PullPort) -> usize {
        self.nodes[p.id()].0.ready_to_pull(p)
    }
    fn push_ready(&self, p: &PushPort) -> usize {
        self.nodes[p.id()].0.ready_to_push(p)
    }

    fn pull_from(&mut self, port: &PullPort, count: usize) -> Frame {
        self.nodes[port.id()].0.pull_frame(port, count)
    }

    fn push_to(&mut self, p: &PushPort, f: Frame) {
        self.nodes[p.id()].0.push_frame(p, f)
    }
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self::new()
    }
}

use vidmod_core::spec::NodeGraph;

mod common;

use common::{TestSink, TestSource};
use vidmod_node::Node;

#[test]
fn link_nodes() {
    let mut graph = NodeGraph::new();
    let n1 = graph.insert(Node::Source(Box::new(TestSource::new())));
    let n2 = graph.insert(Node::Sink(Box::new(TestSink::new())));

    let p1 = graph.get_pull_port(n1, "out").unwrap();
    let p2 = graph.get_push_port(n2, "in").unwrap();

    graph.add_link(p1, p2).unwrap();
}

#[test]
#[should_panic(expected = "No such port")]
fn pull_name() {
    let mut graph = NodeGraph::new();
    let n1 = graph.insert(Node::Source(Box::new(TestSource::new())));

    graph.get_pull_port(n1, "foo").unwrap();
}

#[test]
#[should_panic(expected = "No such port")]
fn push_name() {
    let mut graph = NodeGraph::new();
    let n1 = graph.insert(Node::Sink(Box::new(TestSink::new())));

    graph.get_push_port(n1, "foo").unwrap();
}

#[test]
#[should_panic(expected = "Can't get pull port of output node")]
fn pull_output() {
    let mut graph = NodeGraph::new();
    let n1 = graph.insert(Node::Sink(Box::new(TestSink::new())));

    graph.get_pull_port(n1, "out").unwrap();
}

#[test]
#[should_panic(expected = "Can't get push port of input node")]
fn push_input() {
    let mut graph = NodeGraph::new();
    let n1 = graph.insert(Node::Source(Box::new(TestSource::new())));

    graph.get_push_port(n1, "in").unwrap();
}

#[test]
fn network() {
    let mut graph = NodeGraph::new();
    let n1 = graph.insert(Node::Source(Box::new(TestSource::new())));
    let n2 = graph.insert(Node::Sink(Box::new(TestSink::new())));

    let p1 = graph.get_pull_port(n1, "out").unwrap();
    let p2 = graph.get_push_port(n2, "in").unwrap();

    graph.add_link(p1, p2).unwrap();
}

use std::collections::VecDeque;

use vidmod_node::{Frame, FrameKind, PullFrame, PullPort, PushFrame, PushPort, TickNode};

#[derive(Debug)]
pub struct TestSource {
    n: u16,
}

impl TestSource {
    pub fn new() -> Self {
        Self { n: 0 }
    }
}

impl PullFrame for TestSource {
    fn pull_frame(&mut self, port: &PullPort, count: usize) -> Frame {
        assert_eq!(count, 1);
        self.n += 1;
        match port.name() {
            "out" => Frame::U16(VecDeque::from(vec![self.n - 1])),
            _ => panic!("Unknown port {}", port.name()),
        }
    }

    fn test_pull_port(&self, name: &str) -> bool {
        name == "out"
    }

    fn pull_port_kind(&self, name: &str) -> FrameKind {
        match name {
            "out" => FrameKind::U16,
            _ => panic!("Unknown port {}", name),
        }
    }
    fn ready_to_pull(&self, port: &PullPort) -> usize {
        match port.name() {
            "out" => 1,
            _ => panic!("Unknown port {}", port.name()),
        }
    }
}

impl TickNode for TestSource {}

#[derive(Debug)]
pub struct TestSink {
    n: u16,
}

impl TestSink {
    pub fn new() -> Self {
        Self { n: 0 }
    }
}

impl PushFrame for TestSink {
    fn push_frame(&mut self, port: &PushPort, frame: Frame) {
        self.n += 1;
        match port.name() {
            "in" => {
                if let Frame::U16(a) = frame {
                    assert!(a == vec![self.n - 1]);
                } else {
                    panic!("Pushed frame wrong type");
                }
            }
            _ => panic!("Unknown port {}", port.name()),
        }
    }

    fn test_push_port(&self, name: &str) -> bool {
        name == "in"
    }

    fn push_port_kind(&self, name: &str) -> FrameKind {
        match name {
            "in" => FrameKind::U16,
            _ => panic!("Unknown port {}", name),
        }
    }
    fn ready_to_push(&self, port: &PushPort) -> usize {
        match port.name() {
            "in" => 1,
            _ => panic!("Unknown port {}", port.name()),
        }
    }
}

impl TickNode for TestSink {}

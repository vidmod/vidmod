use std::collections::VecDeque;

use common::{TestSink, TestSource};
use vidmod_node::{Frame, PullFrame, PushFrame};

mod common;

#[test]
fn test_pull() {
    let mut input = TestSource::new();

    let port = input.get_pull_port(0, "out").unwrap();

    for i in 0..8 {
        let frame = input.pull_frame(&port, 1);

        if let Frame::U16(a) = frame {
            assert_eq!(a, vec![i]);
        } else {
            panic!("Pulled frame wrong type");
        }
    }
}

#[test]
fn test_push() {
    let mut output = TestSink::new();

    let port = output.get_push_port(0, "in").unwrap();

    for i in 0..8 {
        output.push_frame(&port, Frame::U16(VecDeque::from(vec![i])));
    }
}

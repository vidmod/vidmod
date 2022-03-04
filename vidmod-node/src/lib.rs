#![warn(missing_docs)]

//! API for declaring vidmod  processing nodes

use std::fmt::Debug;

use anyhow::{Error, Result};
use ndarray::{ArcArray, Ix2};

/// A node's port to pull frames out
#[derive(Debug, Clone)]
pub struct PullPort {
    id:   usize,
    name: String,
    kind: FrameKind,
}

impl PullPort {
    /// Get the node's ID
    pub fn id(&self) -> usize {
        self.id
    }
    /// Get the port's name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A node's port to push frames in
#[derive(Debug, Clone)]
pub struct PushPort {
    id:   usize,
    name: String,
    kind: FrameKind,
}

impl PushPort {
    /// Get the node's ID
    pub fn id(&self) -> usize {
        self.id
    }
    /// Get the port's name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A node implementing PullPush can both pull and push
pub trait PullPush: PullFrame + PushFrame {}

impl<T> PullPush for T where T: PullFrame + PushFrame {}

/// A node implementing PushFrame can accept pushed frames
pub trait PushFrame: Debug + TickNode {
    /// Recieve a pushed frame and prepare to process
    fn push_frame(&mut self, port: &PushPort, frame: Frame);
    /// Test if we have a named port
    fn test_push_port(&self, name: &str) -> bool;
    /// Get the kind of a named port
    fn push_port_kind(&self, name: &str) -> FrameKind;
    /// Get the number of frames we're prepared to receive
    fn ready_to_push(&self, name: &PushPort) -> usize;

    /// Get the push port for a given name
    fn get_push_port(&self, id: usize, name: &str) -> Result<PushPort> {
        if self.test_push_port(name) {
            Ok(PushPort {
                id,
                name: name.to_string(),
                kind: self.push_port_kind(name),
            })
        } else {
            Err(Error::msg("No such port"))
        }
    }
    /// Attach another node's pull port to the push port for a given name
    fn attach_pull_port(&self, name: &str, port: PullPort) -> Result<()> {
        if !self.test_push_port(name) {
            return Err(Error::msg("No such port"));
        }
        if self.push_port_kind(name) != port.kind {
            return Err(Error::msg("Port kind mismatch"));
        }
        Ok(())
    }
}

/// A node implementing PullFrame can produce pulled frames
pub trait PullFrame: Debug + TickNode {
    /// Send a processed frame
    fn pull_frame(&mut self, port: &PullPort, count: usize) -> Frame;
    /// Test if we have a named port
    fn test_pull_port(&self, name: &str) -> bool;
    /// Get the kind of a named port
    fn pull_port_kind(&self, name: &str) -> FrameKind;
    /// Get the number of frames we're prepared to send
    fn ready_to_pull(&self, name: &PullPort) -> usize;

    /// Get the pull port for a given name
    fn get_pull_port(&self, id: usize, name: &str) -> Result<PullPort> {
        if self.test_pull_port(name) {
            Ok(PullPort {
                id,
                name: name.to_string(),
                kind: self.pull_port_kind(name),
            })
        } else {
            Err(Error::msg("No such port"))
        }
    }
    /// Attach another node's push port to the pull port for a given name
    fn attach_push_port(&self, name: &str, port: PushPort) -> Result<()> {
        if !self.test_pull_port(name) {
            return Err(Error::msg("No such port"));
        }
        if self.pull_port_kind(name) != port.kind {
            return Err(Error::msg("Port kind mismatch"));
        }
        Ok(())
    }
}

/// All nodes must be able to be ticked
pub trait TickNode {
    /// Signal to the node to process all available frames
    fn tick(&mut self) -> bool {
        false
    }
}

/// A frame is a single point of data to pass between nodes
#[derive(Debug, Clone)]
pub enum Frame {
    /// A buffer of single u8s
    U8(Vec<u8>),
    /// A buffer of single u16s
    U16(Vec<u16>),
    /// A 2D array of u8s
    U8x2(ArcArray<u8, Ix2>),
}

impl From<ArcArray<u8, Ix2>> for Frame {
    fn from(data: ArcArray<u8, Ix2>) -> Self {
        Frame::U8x2(data)
    }
}

/// Datatype enum for a frame
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameKind {
    /// A buffer of single u8s
    U8,
    /// A buffer of single u16s
    U16,
    /// A 2D array of u8s
    U8x2,
}

impl From<&Frame> for FrameKind {
    fn from(f: &Frame) -> Self {
        match f {
            Frame::U8(_) => FrameKind::U8,
            Frame::U16(_) => FrameKind::U16,
            Frame::U8x2(_) => FrameKind::U8x2,
        }
    }
}

impl From<&str> for FrameKind {
    fn from(f: &str) -> Self {
        match f {
            "U8" => FrameKind::U8,
            "U16" => FrameKind::U16,
            "U8x2" => FrameKind::U8x2,
            _ => unimplemented!("Frame kind {}", f),
        }
    }
}

/// A processing node
#[derive(Debug)]
pub enum Node {
    /// A source node provides frames
    Source(Box<dyn PullFrame>),
    /// An intermediate node transforms frames
    Intermediate(Box<dyn PullPush>),
    /// A sink node consumes frames
    Sink(Box<dyn PushFrame>),
    /// A null node does not process frames
    Null,
}

impl TickNode for Node {
    fn tick(&mut self) -> bool {
        match self {
            Node::Source(n) => n.tick(),
            Node::Intermediate(n) => n.tick(),
            Node::Sink(n) => n.tick(),
            Node::Null => false,
        }
    }
}

#![warn(missing_docs)]
#![allow(clippy::new_without_default)]

//! API for declaring vidmod  processing nodes

use std::{collections::BTreeMap, fmt::Debug};

use anyhow::{Error, Result};
use ndarray::{ArcArray, Ix2};
use queues::{Buffer, IsQueue};

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
    U8x2(Vec<ArcArray<u8, Ix2>>),
}

impl From<ArcArray<u8, Ix2>> for Frame {
    fn from(data: ArcArray<u8, Ix2>) -> Self {
        Frame::U8x2(vec![data])
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
    /// A generation-2 node can push or pull
    N2(Box<dyn Node2TA>),
}

impl Node {
    /// Initialize the node
    pub fn init(&mut self) {
        if let Node::N2(v) = self {
            v.init()
        }
    }
}

impl TickNode for Node {
    fn tick(&mut self) -> bool {
        match self {
            Node::Source(n) => n.tick(),
            Node::Intermediate(n) => n.tick(),
            Node::Sink(n) => n.tick(),
            Node::Null => false,
            Node::N2(n) => n.tick(),
        }
    }
}

/// Rev2 node- TODO rename
#[derive(Debug)]
pub struct Node2 {
    pullports: BTreeMap<String, (FrameKind, Buffer<Frame>)>,
    pushports: BTreeMap<String, (FrameKind, Buffer<Frame>)>,
}

#[allow(missing_docs)]
impl Node2 {
    pub fn new() -> Self {
        Self {
            pullports: BTreeMap::new(),
            pushports: BTreeMap::new(),
        }
    }

    pub fn register_pullport(&mut self, name: &str, kind: FrameKind, buf_size: usize) {
        self.pullports
            .insert(name.to_owned(), (kind, Buffer::new(buf_size)));
    }
    pub fn register_pushport(&mut self, name: &str, kind: FrameKind, buf_size: usize) {
        self.pushports
            .insert(name.to_owned(), (kind, Buffer::new(buf_size)));
    }

    pub fn get_pull_port(&self, id: usize, name: &str) -> anyhow::Result<PullPort> {
        if let Some((kind, _)) = self.pullports.get(name) {
            Ok(PullPort {
                id,
                name: name.to_owned(),
                kind: *kind,
            })
        } else {
            Err(Error::msg(format!("No pull port: {}", name)))
        }
    }
    pub fn get_push_port(&self, id: usize, name: &str) -> anyhow::Result<PushPort> {
        if let Some((kind, _)) = self.pushports.get(name) {
            Ok(PushPort {
                id,
                name: name.to_owned(),
                kind: *kind,
            })
        } else {
            Err(Error::msg(format!("No push port: {}", name)))
        }
    }

    pub fn attach_push_port(&self, name: &str, port: PushPort) -> Result<()> {
        if let Some((kind, _)) = self.pullports.get(name) {
            if port.kind == *kind {
                Ok(())
            } else {
                Err(Error::msg(format!(
                    "Port kind mismatch: {:?},{:?}",
                    port.kind, kind
                )))
            }
        } else {
            Err(Error::msg(format!("No push port: {}", name)))
        }
    }

    pub fn attach_pull_port(&self, name: &str, port: PullPort) -> Result<()> {
        if let Some((kind, _)) = self.pushports.get(name) {
            if port.kind == *kind {
                Ok(())
            } else {
                Err(Error::msg(format!(
                    "Port kind mismatch: {:?},{:?}",
                    port.kind, kind
                )))
            }
        } else {
            Err(Error::msg(format!("No pull port: {}", name)))
        }
    }

    pub fn outbuf_avail(&self, name: &str) -> usize {
        if let Some((_, buf)) = self.pullports.get(name) {
            buf.capacity() - buf.size()
        } else {
            panic!("No pull port: {}", name)
        }
    }
    pub fn inbuf_avail(&self, name: &str) -> usize {
        if let Some((_, buf)) = self.pushports.get(name) {
            buf.size()
        } else {
            panic!("No push port: {}", name)
        }
    }
    pub fn outbuf_put(&mut self, name: &str, frame: Frame) {
        if let Some((_, buf)) = self.pullports.get_mut(name) {
            buf.add(frame).unwrap();
        } else {
            panic!("No pull port: {}", name)
        }
    }
    pub fn inbuf_get(&mut self, name: &str) -> Frame {
        if let Some((_, buf)) = self.pushports.get_mut(name) {
            buf.remove().unwrap()
        } else {
            panic!("No pull port: {}", name)
        }
    }

    pub fn ready_to_pull(&self, port: &PullPort) -> usize {
        if let Some((_, buf)) = self.pullports.get(&port.name) {
            buf.size()
        } else {
            panic!("No pull port: {}", &port.name)
        }
    }
    pub fn ready_to_push(&self, port: &PushPort) -> usize {
        if let Some((_, buf)) = self.pushports.get(&port.name) {
            buf.capacity() - buf.size()
        } else {
            panic!("No push port: {}", &port.name)
        }
    }
    pub fn pull_frame(&mut self, port: &PullPort, count: usize) -> Frame {
        assert_eq!(count, 1);
        if let Some((_, buf)) = self.pullports.get_mut(&port.name) {
            buf.remove().unwrap()
        } else {
            panic!("No pull port: {}", port.name)
        }
    }
    pub fn push_frame(&mut self, port: &PushPort, frame: Frame) {
        if let Some((_, buf)) = self.pushports.get_mut(&port.name) {
            buf.add(frame).unwrap();
        } else {
            panic!("No pull port: {}", port.name)
        }
    }
}

/// All trait functions for a node
pub trait Node2TA: Node2T + Node2MT {}

impl<T> Node2TA for T where T: Node2T + Node2MT {}

/// User-implemented functions for a node
pub trait Node2T: Debug {
    /// Setup for the node - register all ports here
    fn init(&mut self);
    /// Tick function for the node - signals the node to process data
    fn tick(&mut self) -> bool;
}

/// Macro-generated functions for a node
pub trait Node2MT {
    /// Register a pull port
    fn register_pullport(&mut self, name: &str, kind: FrameKind, buf_size: usize);
    /// Register a push port
    fn register_pushport(&mut self, name: &str, kind: FrameKind, buf_size: usize);
    /// Get a named pull port
    fn get_pull_port(&self, id: usize, name: &str) -> Result<PullPort>;
    /// Get a named push port
    fn get_push_port(&self, id: usize, name: &str) -> Result<PushPort>;
    /// Attach a pull port to a named push port
    fn attach_pull_port(&self, name: &str, port: PullPort) -> Result<()>;
    /// Attach a push port to a named pull port
    fn attach_push_port(&self, name: &str, port: PushPort) -> Result<()>;
    /// Check how many frames can be pulled before the output buffer is empty
    fn ready_to_pull(&self, port: &PullPort) -> usize;
    /// Check how many frames can be pushed before the input buffer is full
    fn ready_to_push(&self, port: &PushPort) -> usize;
    /// Pull a frame from the output buffer
    fn pull_frame(&mut self, port: &PullPort, count: usize) -> Frame;
    /// Push a frame into the input buffer
    fn push_frame(&mut self, port: &PushPort, frame: Frame);

    /// Check how many frames are available in the input buffer
    fn inbuf_avail(&self, name: &str) -> usize;
    /// Check how many spaces are free in the output buffer
    fn outbuf_avail(&self, name: &str) -> usize;
    /// Put a frame into the output buffer
    fn outbuf_put(&mut self, name: &str, frame: Frame);
    /// Get a frame from the input buffer
    fn inbuf_get(&mut self, name: &str) -> Frame;
}

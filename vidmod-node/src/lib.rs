#![warn(missing_docs)]
#![allow(clippy::new_without_default)]

//! API for declaring vidmod  processing nodes

use std::{collections::BTreeMap, fmt::Debug};

use anyhow::{Error, Result};
use frame::{Frame, FrameKind, FrameSingle};

/// Types, traits, and methods for handling frames
pub mod frame;

/// A VecDeque with a maximum capacity limit
pub mod limvecdeque;

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

/// All nodes must be able to be ticked
pub trait TickNode {
    /// Signal to the node to process all available frames
    fn tick(&mut self) -> bool {
        false
    }
}

/// All nodes must be able to be finished
pub trait FinishNode {
    /// Signal to the node to finish processing frames
    fn finish(&mut self) -> bool {
        false
    }
}

/// A processing node
#[derive(Debug)]
pub struct Node(pub Box<dyn Node2TA>);

impl Node {
    /// Initialize the node
    pub fn init(&mut self) {
        self.0.init()
    }
}

impl TickNode for Node {
    fn tick(&mut self) -> bool {
        self.0.tick()
    }
}

impl FinishNode for Node {
    fn finish(&mut self) -> bool {
        self.0.finish()
    }
}

/// Rev2 node- TODO rename
#[derive(Debug)]
pub struct Node2 {
    pullports: BTreeMap<String, Frame>,
    pushports: BTreeMap<String, Frame>,
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
            .insert(name.to_owned(), Frame::with_capacity(kind, buf_size));
    }
    pub fn register_pushport(&mut self, name: &str, kind: FrameKind, buf_size: usize) {
        self.pushports
            .insert(name.to_owned(), Frame::with_capacity(kind, buf_size));
    }

    pub fn get_pull_port(&self, id: usize, name: &str) -> anyhow::Result<PullPort> {
        if let Some(frame) = self.pullports.get(name) {
            Ok(PullPort {
                id,
                name: name.to_owned(),
                kind: frame.into(),
            })
        } else {
            Err(Error::msg(format!("No pull port: {}", name)))
        }
    }
    pub fn get_push_port(&self, id: usize, name: &str) -> anyhow::Result<PushPort> {
        if let Some(frame) = self.pushports.get(name) {
            Ok(PushPort {
                id,
                name: name.to_owned(),
                kind: frame.into(),
            })
        } else {
            Err(Error::msg(format!("No push port: {}", name)))
        }
    }

    pub fn attach_push_port(&self, name: &str, port: PushPort) -> Result<()> {
        if let Some(frame) = self.pullports.get(name) {
            if port.kind == frame.into() {
                Ok(())
            } else {
                Err(Error::msg(format!(
                    "Port kind mismatch: {:?},{:?}",
                    port.kind,
                    FrameKind::from(frame)
                )))
            }
        } else {
            Err(Error::msg(format!("No push port: {}", name)))
        }
    }

    pub fn attach_pull_port(&self, name: &str, port: PullPort) -> Result<()> {
        if let Some(frame) = self.pushports.get(name) {
            if port.kind == frame.into() {
                Ok(())
            } else {
                Err(Error::msg(format!(
                    "Port kind mismatch: {:?},{:?}",
                    port.kind,
                    FrameKind::from(frame)
                )))
            }
        } else {
            Err(Error::msg(format!("No pull port: {}", name)))
        }
    }

    pub fn outbuf_avail(&self, name: &str) -> usize {
        if let Some(frame) = self.pullports.get(name) {
            frame.capacity() - frame.size()
        } else {
            panic!("No pull port: {}", name)
        }
    }
    pub fn inbuf_avail(&self, name: &str) -> usize {
        if let Some(frame) = self.pushports.get(name) {
            frame.size()
        } else {
            panic!("No push port: {}", name)
        }
    }
    pub fn outbuf_put(&mut self, name: &str, frame: Frame) {
        if let Some(f) = self.pullports.get_mut(name) {
            f.add(frame).unwrap();
        } else {
            panic!("No pull port: {}", name)
        }
    }
    pub fn outbuf_put_single(&mut self, name: &str, frame: FrameSingle) {
        if let Some(f) = self.pullports.get_mut(name) {
            f.add_single(frame).unwrap();
        } else {
            panic!("No pull port: {}", name)
        }
    }
    pub fn inbuf_peek(&mut self, name: &str, count: usize) -> Frame {
        if let Some(frame) = self.pushports.get_mut(name) {
            frame.peek(count).unwrap()
        } else {
            panic!("No pull port: {}", name)
        }
    }
    pub fn inbuf_get(&mut self, name: &str, count: usize) -> Frame {
        if let Some(frame) = self.pushports.get_mut(name) {
            frame.remove(count).unwrap()
        } else {
            panic!("No pull port: {}", name)
        }
    }
    pub fn inbuf_get_all(&mut self, name: &str) -> Frame {
        if let Some(frame) = self.pushports.get_mut(name) {
            frame.remove_all()
        } else {
            panic!("No pull port: {}", name)
        }
    }
    pub fn inbuf_get_single(&mut self, name: &str) -> FrameSingle {
        if let Some(frame) = self.pushports.get_mut(name) {
            frame.remove_single().unwrap()
        } else {
            panic!("No pull port: {}", name)
        }
    }

    pub fn ready_to_pull(&self, port: &PullPort) -> usize {
        if let Some(frame) = self.pullports.get(&port.name) {
            frame.size()
        } else {
            panic!("No pull port: {}", &port.name)
        }
    }
    pub fn ready_to_push(&self, port: &PushPort) -> usize {
        if let Some(frame) = self.pushports.get(&port.name) {
            frame.capacity() - frame.size()
        } else {
            panic!("No push port: {}", &port.name)
        }
    }
    pub fn pull_frame(&mut self, port: &PullPort, count: usize) -> Frame {
        if let Some(frame) = self.pullports.get_mut(&port.name) {
            frame.remove(count).unwrap()
        } else {
            panic!("No pull port: {}", port.name)
        }
    }
    pub fn push_frame(&mut self, port: &PushPort, frame: Frame) {
        if let Some(f) = self.pushports.get_mut(&port.name) {
            f.add(frame).unwrap();
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
    /// Returns true if we processed any data this tick
    fn tick(&mut self) -> bool;
    /// Finish function for the node- signals the node to wrap up
    /// Returns true if we cannot possibly ever have more work to do
    fn finish(&mut self) -> bool;
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
    /// Put a frame into the output buffer
    fn outbuf_put_single(&mut self, name: &str, frame: FrameSingle);
    /// Get frames from the input buffer
    fn inbuf_get(&mut self, name: &str, count: usize) -> Frame;
    /// Get frames from the input buffer without consuming
    fn inbuf_peek(&mut self, name: &str, count: usize) -> Frame;
    /// Get a frame from the input buffer
    fn inbuf_get_single(&mut self, name: &str) -> FrameSingle;
    /// Get a frame from the input buffer
    fn inbuf_get_all(&mut self, name: &str) -> Frame;
}

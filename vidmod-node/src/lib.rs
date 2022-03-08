#![warn(missing_docs)]
#![allow(clippy::new_without_default)]

//! API for declaring vidmod  processing nodes

use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Debug,
};

use anyhow::{Error, Result};
use ndarray::{ArcArray, ArcArray2, Ix2};

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

#[derive(Debug, Clone)]
#[repr(packed)]
#[allow(missing_docs)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// A frame is a single point of data to pass between nodes
#[derive(Debug, Clone)]
pub enum Frame {
    /// A buffer of single u8s
    U8(VecDeque<u8>),
    /// A buffer of single u16s
    U16(VecDeque<u16>),
    /// A 2D array of u8s
    U8x2(VecDeque<ArcArray<u8, Ix2>>),
    /// A 2D array of RGBA pixels
    RGBAx2(VecDeque<ArcArray<RGBA, Ix2>>),
}

/// A frame is a single point of data to pass between nodes
pub enum FrameSingle {
    /// A buffer of single u8s
    U8(u8),
    /// A buffer of single u16s
    U16(u16),
    /// A 2D array of u8s
    U8x2(ArcArray<u8, Ix2>),
    /// A 2D array of RGBA pixels
    RGBAx2(ArcArray<RGBA, Ix2>),
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
    /// A 2D array of RGBA pixels
    RGBAx2,
}

impl FrameSingle {
    /// Unwrap the frame into its contents
    pub fn unwrap_u8(self) -> u8 {
        match self {
            FrameSingle::U8(v) => v,
            _ => todo!("{:?}", FrameKind::from(&self)),
        }
    }
    /// Unwrap the frame into its contents
    pub fn unwrap_u16(self) -> u16 {
        match self {
            FrameSingle::U16(v) => v,
            _ => todo!("{:?}", FrameKind::from(&self)),
        }
    }
    /// Unwrap the frame into its contents
    pub fn unwrap_u8x2(self) -> ArcArray2<u8> {
        match self {
            FrameSingle::U8x2(v) => v,
            _ => todo!("{:?}", FrameKind::from(&self)),
        }
    }
    /// Unwrap the frame into its contents
    pub fn unwrap_rgbax2(self) -> ArcArray2<RGBA> {
        match self {
            FrameSingle::RGBAx2(v) => v,
            _ => todo!("{:?}", FrameKind::from(&self)),
        }
    }
}

impl Frame {
    /// Get the number of frames in the queue
    pub fn size(&self) -> usize {
        match self {
            Self::U8(v) => v.len(),
            Self::U16(v) => v.len(),
            Self::U8x2(v) => v.len(),
            Self::RGBAx2(v) => v.len(),
        }
    }
    /// Get the capacity of the queue
    pub fn capacity(&self) -> usize {
        match self {
            Self::U8(v) => v.capacity(),
            Self::U16(v) => v.capacity(),
            Self::U8x2(v) => v.capacity(),
            Self::RGBAx2(v) => v.capacity(),
        }
    }
    /// Add a number of frames to the queue
    pub fn add(&mut self, data: Frame) -> Option<()> {
        if self.capacity() >= self.size() + data.size() {
            match self {
                Self::U8(v) => v.append(&mut data.unwrap_u8()),
                Self::U16(v) => v.append(&mut data.unwrap_u16()),
                Self::U8x2(v) => v.append(&mut data.unwrap_u8x2()),
                Self::RGBAx2(v) => v.append(&mut data.unwrap_rgbax2()),
            }
            Some(())
        } else {
            None
        }
    }
    /// Add a single frame to the queue
    pub fn add_single(&mut self, data: FrameSingle) -> Option<()> {
        if self.capacity() > self.size() {
            match self {
                Self::U8(v) => v.push_back(data.unwrap_u8()),
                Self::U16(v) => v.push_back(data.unwrap_u16()),
                Self::U8x2(v) => v.push_back(data.unwrap_u8x2()),
                Self::RGBAx2(v) => v.push_back(data.unwrap_rgbax2()),
            }
            Some(())
        } else {
            None
        }
    }
    /// Remove a number of frames from the queue
    pub fn remove(&mut self, count: usize) -> Option<Frame> {
        if self.size() >= count {
            Some(match self {
                Self::U8(v) => Frame::U8(VecDeque::from_iter(v.drain(..count))),
                Self::U16(v) => Frame::U16(VecDeque::from_iter(v.drain(..count))),
                Self::U8x2(v) => Frame::U8x2(VecDeque::from_iter(v.drain(..count))),
                Self::RGBAx2(v) => Frame::RGBAx2(VecDeque::from_iter(v.drain(..count))),
            })
        } else {
            None
        }
    }
    /// Remove a single frame from the queue
    pub fn remove_single(&mut self) -> Option<FrameSingle> {
        match self {
            Self::U8(v) => v.pop_front().map(FrameSingle::U8),
            Self::U16(v) => v.pop_front().map(FrameSingle::U16),
            Self::U8x2(v) => v.pop_front().map(FrameSingle::U8x2),
            Self::RGBAx2(v) => v.pop_front().map(FrameSingle::RGBAx2),
        }
    }
    /// Create a new frame with a given capacity
    pub fn with_capacity(kind: FrameKind, capacity: usize) -> Self {
        match kind {
            FrameKind::U8 => Self::U8(VecDeque::with_capacity(capacity)),
            FrameKind::U16 => Self::U16(VecDeque::with_capacity(capacity)),
            FrameKind::U8x2 => Self::U8x2(VecDeque::with_capacity(capacity)),
            FrameKind::RGBAx2 => Self::RGBAx2(VecDeque::with_capacity(capacity)),
        }
    }
    /// Unwrap the frame into its contents
    pub fn unwrap_u8(self) -> VecDeque<u8> {
        match self {
            Frame::U8(v) => v,
            _ => panic!("{:?}", FrameKind::from(&self)),
        }
    }
    /// Unwrap the frame into its contents
    pub fn unwrap_u16(self) -> VecDeque<u16> {
        match self {
            Frame::U16(v) => v,
            _ => panic!("{:?}", FrameKind::from(&self)),
        }
    }
    /// Unwrap the frame into its contents
    pub fn unwrap_u8x2(self) -> VecDeque<ArcArray2<u8>> {
        match self {
            Frame::U8x2(v) => v,
            _ => panic!("{:?}", FrameKind::from(&self)),
        }
    }
    /// Unwrap the frame into its contents
    pub fn unwrap_rgbax2(self) -> VecDeque<ArcArray2<RGBA>> {
        match self {
            Frame::RGBAx2(v) => v,
            _ => panic!("{:?}", FrameKind::from(&self)),
        }
    }
}

impl From<ArcArray<u8, Ix2>> for Frame {
    fn from(data: ArcArray<u8, Ix2>) -> Self {
        Frame::U8x2(VecDeque::from(vec![data]))
    }
}

impl From<&Frame> for FrameKind {
    fn from(f: &Frame) -> Self {
        match f {
            Frame::U8(_) => FrameKind::U8,
            Frame::U16(_) => FrameKind::U16,
            Frame::U8x2(_) => FrameKind::U8x2,
            Frame::RGBAx2(_) => FrameKind::RGBAx2,
        }
    }
}

impl From<&FrameSingle> for FrameKind {
    fn from(f: &FrameSingle) -> Self {
        match f {
            FrameSingle::U8(_) => FrameKind::U8,
            FrameSingle::U16(_) => FrameKind::U16,
            FrameSingle::U8x2(_) => FrameKind::U8x2,
            FrameSingle::RGBAx2(_) => FrameKind::RGBAx2,
        }
    }
}

impl From<&str> for FrameKind {
    fn from(f: &str) -> Self {
        match f {
            "U8" => FrameKind::U8,
            "U16" => FrameKind::U16,
            "U8x2" => FrameKind::U8x2,
            "RGBAx2" => FrameKind::RGBAx2,
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
    pub fn inbuf_get(&mut self, name: &str, count: usize) -> Frame {
        if let Some(frame) = self.pushports.get_mut(name) {
            frame.remove(count).unwrap()
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
        assert_eq!(count, 1);
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
    /// Put a frame into the output buffer
    fn outbuf_put_single(&mut self, name: &str, frame: FrameSingle);
    /// Get a frame from the input buffer
    fn inbuf_get(&mut self, name: &str, count: usize) -> Frame;
    /// Get a frame from the input buffer
    fn inbuf_get_single(&mut self, name: &str) -> FrameSingle;
}

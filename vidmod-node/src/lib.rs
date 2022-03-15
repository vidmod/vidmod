#![warn(missing_docs)]
#![allow(clippy::new_without_default)]

//! API for declaring vidmod  processing nodes

use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Debug,
};

use anyhow::{Error, Result};
use ndarray::{ArcArray1, ArcArray2};
use vidmod_macros::{unwrap_impl_frame, unwrap_impl_frame_single};

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

#[derive(Debug, Clone)]
#[repr(packed)]
#[allow(missing_docs)]
pub struct RGBA8 {
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
    /// A 1D array of u8s
    U8x1(VecDeque<ArcArray1<u8>>),
    /// A 2D array of u8s
    U8x2(VecDeque<ArcArray2<u8>>),
    /// A buffer of single u16s
    U16(VecDeque<u16>),
    /// A 1D array of u16s
    U16x1(VecDeque<ArcArray1<u16>>),
    /// A 2D array of u16s
    U16x2(VecDeque<ArcArray2<u16>>),
    /// A buffer of single f32s
    F32(VecDeque<f32>),
    /// A 1D array of f32s
    F32x1(VecDeque<ArcArray1<f32>>),
    /// A 2D array of f32s
    F32x2(VecDeque<ArcArray2<f32>>),
    /// A 2D array of RGBA8 pixels
    RGBA8x2(VecDeque<ArcArray2<RGBA8>>),
}

/// A frame is a single point of data to pass between nodes
pub enum FrameSingle {
    /// A buffer of single u8s
    U8(u8),
    /// A 1D array of u8s
    U8x1(ArcArray1<u8>),
    /// A 2D array of u8s
    U8x2(ArcArray2<u8>),
    /// A buffer of single u16s
    U16(u16),
    /// A 1D array of u16s
    U16x1(ArcArray1<u16>),
    /// A 2D array of u16s
    U16x2(ArcArray2<u16>),
    /// A buffer of single f32s
    F32(f32),
    /// A 1D array of f32s
    F32x1(ArcArray1<f32>),
    /// A 2D array of f32s
    F32x2(ArcArray2<f32>),
    /// A 2D array of RGBA8 pixels
    RGBA8x2(ArcArray2<RGBA8>),
}

/// Datatype enum for a frame
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameKind {
    /// A buffer of single u8s
    U8,
    /// A 1D array of u8s
    U8x1,
    /// A 2D array of u8s
    U8x2,
    /// A buffer of single u16s
    U16,
    /// A 1D array of u16s
    U16x1,
    /// A 2D array of u16s
    U16x2,
    /// A buffer of single f32s
    F32,
    /// A 1D array of f32s
    F32x1,
    /// A 2D array of f32s
    F32x2,
    /// A 2D array of RGBA8 pixels
    RGBA8x2,
}

impl FrameSingle {
    unwrap_impl_frame_single!(u8, 0);
    unwrap_impl_frame_single!(u8, 1);
    unwrap_impl_frame_single!(u8, 2);
    unwrap_impl_frame_single!(u16, 0);
    unwrap_impl_frame_single!(u16, 1);
    unwrap_impl_frame_single!(u16, 2);
    unwrap_impl_frame_single!(f32, 0);
    unwrap_impl_frame_single!(f32, 1);
    unwrap_impl_frame_single!(f32, 2);
    unwrap_impl_frame_single!(RGBA8, 2);
}

impl Frame {
    /// Get the number of frames in the queue
    pub fn size(&self) -> usize {
        match self {
            Self::U8(v) => v.len(),
            Self::U8x1(v) => v.len(),
            Self::U8x2(v) => v.len(),
            Self::U16(v) => v.len(),
            Self::U16x1(v) => v.len(),
            Self::U16x2(v) => v.len(),
            Self::F32(v) => v.len(),
            Self::F32x1(v) => v.len(),
            Self::F32x2(v) => v.len(),
            Self::RGBA8x2(v) => v.len(),
        }
    }
    /// Get the capacity of the queue
    pub fn capacity(&self) -> usize {
        match self {
            Self::U8(v) => v.capacity(),
            Self::U8x1(v) => v.capacity(),
            Self::U8x2(v) => v.capacity(),
            Self::U16(v) => v.capacity(),
            Self::U16x1(v) => v.capacity(),
            Self::U16x2(v) => v.capacity(),
            Self::F32(v) => v.capacity(),
            Self::F32x1(v) => v.capacity(),
            Self::F32x2(v) => v.capacity(),
            Self::RGBA8x2(v) => v.capacity(),
        }
    }
    /// Add a number of frames to the queue
    pub fn add(&mut self, data: Frame) -> Option<()> {
        if self.capacity() >= self.size() + data.size() {
            match self {
                Self::U8(v) => v.append(&mut data.unwrap_u8()),
                Self::U8x1(v) => v.append(&mut data.unwrap_u8x1()),
                Self::U8x2(v) => v.append(&mut data.unwrap_u8x2()),
                Self::U16(v) => v.append(&mut data.unwrap_u16()),
                Self::U16x1(v) => v.append(&mut data.unwrap_u16x1()),
                Self::U16x2(v) => v.append(&mut data.unwrap_u16x2()),
                Self::F32(v) => v.append(&mut data.unwrap_f32()),
                Self::F32x1(v) => v.append(&mut data.unwrap_f32x1()),
                Self::F32x2(v) => v.append(&mut data.unwrap_f32x2()),
                Self::RGBA8x2(v) => v.append(&mut data.unwrap_rgba8x2()),
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
                Self::U8x1(v) => v.push_back(data.unwrap_u8x1()),
                Self::U8x2(v) => v.push_back(data.unwrap_u8x2()),
                Self::U16(v) => v.push_back(data.unwrap_u16()),
                Self::U16x1(v) => v.push_back(data.unwrap_u16x1()),
                Self::U16x2(v) => v.push_back(data.unwrap_u16x2()),
                Self::F32(v) => v.push_back(data.unwrap_f32()),
                Self::F32x1(v) => v.push_back(data.unwrap_f32x1()),
                Self::F32x2(v) => v.push_back(data.unwrap_f32x2()),
                Self::RGBA8x2(v) => v.push_back(data.unwrap_rgba8x2()),
            }
            Some(())
        } else {
            None
        }
    }
    /// Look a number of frames from the queue without removing
    pub fn peek(&mut self, count: usize) -> Option<Frame> {
        if self.size() >= count {
            Some(match self {
                Self::U8(v) => Frame::U8(VecDeque::from(v.make_contiguous()[..count].to_vec())),
                Self::U8x1(v) => Frame::U8x1(VecDeque::from(v.make_contiguous()[..count].to_vec())),
                Self::U8x2(v) => Frame::U8x2(VecDeque::from(v.make_contiguous()[..count].to_vec())),
                Self::U16(v) => Frame::U16(VecDeque::from(v.make_contiguous()[..count].to_vec())),
                Self::U16x1(v) => {Frame::U16x1(VecDeque::from(v.make_contiguous()[..count].to_vec()))}
                Self::U16x2(v) => {Frame::U16x2(VecDeque::from(v.make_contiguous()[..count].to_vec()))}
                Self::F32(v) => Frame::F32(VecDeque::from(v.make_contiguous()[..count].to_vec())),
                Self::F32x1(v) => {Frame::F32x1(VecDeque::from(v.make_contiguous()[..count].to_vec()))}
                Self::F32x2(v) => {Frame::F32x2(VecDeque::from(v.make_contiguous()[..count].to_vec()))}
                Self::RGBA8x2(v) => {
                    Frame::RGBA8x2(VecDeque::from(v.make_contiguous()[..count].to_vec()))
                }
            })
        } else {
            None
        }
    }
    /// Remove a number of frames from the queue
    pub fn remove(&mut self, count: usize) -> Option<Frame> {
        if self.size() >= count {
            Some(match self {
                Self::U8(v) => Frame::U8(VecDeque::from_iter(v.drain(..count))),
                Self::U8x1(v) => Frame::U8x1(VecDeque::from_iter(v.drain(..count))),
                Self::U8x2(v) => Frame::U8x2(VecDeque::from_iter(v.drain(..count))),
                Self::U16(v) => Frame::U16(VecDeque::from_iter(v.drain(..count))),
                Self::U16x1(v) => Frame::U16x1(VecDeque::from_iter(v.drain(..count))),
                Self::U16x2(v) => Frame::U16x2(VecDeque::from_iter(v.drain(..count))),
                Self::F32(v) => Frame::F32(VecDeque::from_iter(v.drain(..count))),
                Self::F32x1(v) => Frame::F32x1(VecDeque::from_iter(v.drain(..count))),
                Self::F32x2(v) => Frame::F32x2(VecDeque::from_iter(v.drain(..count))),
                Self::RGBA8x2(v) => Frame::RGBA8x2(VecDeque::from_iter(v.drain(..count))),
            })
        } else {
            None
        }
    }
    /// Remove a single frame from the queue
    pub fn remove_single(&mut self) -> Option<FrameSingle> {
        match self {
            Self::U8(v) => v.pop_front().map(FrameSingle::U8),
            Self::U8x1(v) => v.pop_front().map(FrameSingle::U8x1),
            Self::U8x2(v) => v.pop_front().map(FrameSingle::U8x2),
            Self::U16(v) => v.pop_front().map(FrameSingle::U16),
            Self::U16x1(v) => v.pop_front().map(FrameSingle::U16x1),
            Self::U16x2(v) => v.pop_front().map(FrameSingle::U16x2),
            Self::F32(v) => v.pop_front().map(FrameSingle::F32),
            Self::F32x1(v) => v.pop_front().map(FrameSingle::F32x1),
            Self::F32x2(v) => v.pop_front().map(FrameSingle::F32x2),
            Self::RGBA8x2(v) => v.pop_front().map(FrameSingle::RGBA8x2),
        }
    }
    /// Create a new frame with a given capacity
    pub fn with_capacity(kind: FrameKind, capacity: usize) -> Self {
        match kind {
            FrameKind::U8 => Self::U8(VecDeque::with_capacity(capacity)),
            FrameKind::U8x1 => Self::U8x2(VecDeque::with_capacity(capacity)),
            FrameKind::U8x2 => Self::U8x2(VecDeque::with_capacity(capacity)),
            FrameKind::U16 => Self::U16(VecDeque::with_capacity(capacity)),
            FrameKind::U16x1 => Self::U16x1(VecDeque::with_capacity(capacity)),
            FrameKind::U16x2 => Self::U16x2(VecDeque::with_capacity(capacity)),
            FrameKind::F32 => Self::U16(VecDeque::with_capacity(capacity)),
            FrameKind::F32x1 => Self::U16x1(VecDeque::with_capacity(capacity)),
            FrameKind::F32x2 => Self::U16x2(VecDeque::with_capacity(capacity)),
            FrameKind::RGBA8x2 => Self::RGBA8x2(VecDeque::with_capacity(capacity)),
        }
    }
    unwrap_impl_frame!(u8, 0);
    unwrap_impl_frame!(u8, 1);
    unwrap_impl_frame!(u8, 2);
    unwrap_impl_frame!(u16, 0);
    unwrap_impl_frame!(u16, 1);
    unwrap_impl_frame!(u16, 2);
    unwrap_impl_frame!(f32, 0);
    unwrap_impl_frame!(f32, 1);
    unwrap_impl_frame!(f32, 2);
    unwrap_impl_frame!(RGBA8, 2);
}

impl From<ArcArray2<u8>> for Frame {
    fn from(data: ArcArray2<u8>) -> Self {
        Frame::U8x2(VecDeque::from(vec![data]))
    }
}

impl From<&Frame> for FrameKind {
    fn from(f: &Frame) -> Self {
        match f {
            Frame::U8(_) => FrameKind::U8,
            Frame::U8x1(_) => FrameKind::U8x1,
            Frame::U8x2(_) => FrameKind::U8x2,
            Frame::U16(_) => FrameKind::U16,
            Frame::U16x1(_) => FrameKind::U16x1,
            Frame::U16x2(_) => FrameKind::U16x2,
            Frame::F32(_) => FrameKind::F32,
            Frame::F32x1(_) => FrameKind::F32x1,
            Frame::F32x2(_) => FrameKind::F32x2,
            Frame::RGBA8x2(_) => FrameKind::RGBA8x2,
        }
    }
}

impl From<&FrameSingle> for FrameKind {
    fn from(f: &FrameSingle) -> Self {
        match f {
            FrameSingle::U8(_) => FrameKind::U8,
            FrameSingle::U8x1(_) => FrameKind::U8x1,
            FrameSingle::U8x2(_) => FrameKind::U8x2,
            FrameSingle::U16(_) => FrameKind::U16,
            FrameSingle::U16x1(_) => FrameKind::U16x1,
            FrameSingle::U16x2(_) => FrameKind::U16x2,
            FrameSingle::F32(_) => FrameKind::F32,
            FrameSingle::F32x1(_) => FrameKind::F32x1,
            FrameSingle::F32x2(_) => FrameKind::F32x2,
            FrameSingle::RGBA8x2(_) => FrameKind::RGBA8x2,
        }
    }
}

impl From<&str> for FrameKind {
    fn from(f: &str) -> Self {
        match f {
            "U8" => FrameKind::U8,
            "U8x1" => FrameKind::U8x1,
            "U8x2" => FrameKind::U8x2,
            "U16" => FrameKind::U16,
            "U16x1" => FrameKind::U16x1,
            "U16x2" => FrameKind::U16x2,
            "F32" => FrameKind::F32,
            "F32x1" => FrameKind::F32x1,
            "F32x2" => FrameKind::F32x2,
            "RGBA8x2" => FrameKind::RGBA8x2,
            _ => unimplemented!("Frame kind {}", f),
        }
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
}

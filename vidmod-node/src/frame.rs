use std::iter::FromIterator;

use ndarray::{ArcArray1, ArcArray2};
use vidmod_macros::{unwrap_impl_frame, unwrap_impl_frame_single};

use crate::limvecdeque::LimVecDeque;

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
    U8(LimVecDeque<u8>),
    /// A 1D array of u8s
    U8x1(LimVecDeque<ArcArray1<u8>>),
    /// A 2D array of u8s
    U8x2(LimVecDeque<ArcArray2<u8>>),
    /// A buffer of single u16s
    U16(LimVecDeque<u16>),
    /// A 1D array of u16s
    U16x1(LimVecDeque<ArcArray1<u16>>),
    /// A 2D array of u16s
    U16x2(LimVecDeque<ArcArray2<u16>>),
    /// A buffer of single f32s
    F32(LimVecDeque<f32>),
    /// A 1D array of f32s
    F32x1(LimVecDeque<ArcArray1<f32>>),
    /// A 2D array of f32s
    F32x2(LimVecDeque<ArcArray2<f32>>),
    /// A 2D array of RGBA8 pixels
    RGBA8x2(LimVecDeque<ArcArray2<RGBA8>>),
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
                Self::U8(v) => Frame::U8(LimVecDeque::from(v.make_contiguous()[..count].to_vec())),
                Self::U8x1(v) => {
                    Frame::U8x1(LimVecDeque::from(v.make_contiguous()[..count].to_vec()))
                }
                Self::U8x2(v) => {
                    Frame::U8x2(LimVecDeque::from(v.make_contiguous()[..count].to_vec()))
                }
                Self::U16(v) => {
                    Frame::U16(LimVecDeque::from(v.make_contiguous()[..count].to_vec()))
                }
                Self::U16x1(v) => {
                    Frame::U16x1(LimVecDeque::from(v.make_contiguous()[..count].to_vec()))
                }
                Self::U16x2(v) => {
                    Frame::U16x2(LimVecDeque::from(v.make_contiguous()[..count].to_vec()))
                }
                Self::F32(v) => {
                    Frame::F32(LimVecDeque::from(v.make_contiguous()[..count].to_vec()))
                }
                Self::F32x1(v) => {
                    Frame::F32x1(LimVecDeque::from(v.make_contiguous()[..count].to_vec()))
                }
                Self::F32x2(v) => {
                    Frame::F32x2(LimVecDeque::from(v.make_contiguous()[..count].to_vec()))
                }
                Self::RGBA8x2(v) => {
                    Frame::RGBA8x2(LimVecDeque::from(v.make_contiguous()[..count].to_vec()))
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
                Self::U8(v) => Frame::U8(LimVecDeque::from_iter(v.drain(..count))),
                Self::U8x1(v) => Frame::U8x1(LimVecDeque::from_iter(v.drain(..count))),
                Self::U8x2(v) => Frame::U8x2(LimVecDeque::from_iter(v.drain(..count))),
                Self::U16(v) => Frame::U16(LimVecDeque::from_iter(v.drain(..count))),
                Self::U16x1(v) => Frame::U16x1(LimVecDeque::from_iter(v.drain(..count))),
                Self::U16x2(v) => Frame::U16x2(LimVecDeque::from_iter(v.drain(..count))),
                Self::F32(v) => Frame::F32(LimVecDeque::from_iter(v.drain(..count))),
                Self::F32x1(v) => Frame::F32x1(LimVecDeque::from_iter(v.drain(..count))),
                Self::F32x2(v) => Frame::F32x2(LimVecDeque::from_iter(v.drain(..count))),
                Self::RGBA8x2(v) => Frame::RGBA8x2(LimVecDeque::from_iter(v.drain(..count))),
            })
        } else {
            None
        }
    }
    /// Remove all frames from the queue
    pub fn remove_all(&mut self) -> Frame {
        let mut new = Frame::with_capacity(FrameKind::from(self as &Frame), self.capacity());
        std::mem::swap(&mut new, self);
        new
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
            FrameKind::U8 => Self::U8(LimVecDeque::with_capacity(capacity)),
            FrameKind::U8x1 => Self::U8x2(LimVecDeque::with_capacity(capacity)),
            FrameKind::U8x2 => Self::U8x2(LimVecDeque::with_capacity(capacity)),
            FrameKind::U16 => Self::U16(LimVecDeque::with_capacity(capacity)),
            FrameKind::U16x1 => Self::U16x1(LimVecDeque::with_capacity(capacity)),
            FrameKind::U16x2 => Self::U16x2(LimVecDeque::with_capacity(capacity)),
            FrameKind::F32 => Self::F32(LimVecDeque::with_capacity(capacity)),
            FrameKind::F32x1 => Self::F32x1(LimVecDeque::with_capacity(capacity)),
            FrameKind::F32x2 => Self::F32x2(LimVecDeque::with_capacity(capacity)),
            FrameKind::RGBA8x2 => Self::RGBA8x2(LimVecDeque::with_capacity(capacity)),
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
        Frame::U8x2(LimVecDeque::from(vec![data]))
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

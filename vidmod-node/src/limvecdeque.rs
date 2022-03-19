use std::{collections::VecDeque, ops::RangeBounds};

use all_asserts::assert_le;

/// A VecDeque wrapper that enforces a limited capacity
#[derive(Debug, Clone)]
pub struct LimVecDeque<T> {
    queue:    VecDeque<T>,
    capacity: usize,
}

impl<T> LimVecDeque<T> {
    /// Creates an empty LimVecDeque with capacity for up to `capacity` elements.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    /// Removed the first element and returns it, or `None` if empty.
    pub fn pop_front(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
    /// Appends an element to the back of the deque.
    pub fn push_back(&mut self, val: T) {
        assert_le!(self.queue.len() + 1, self.capacity);
        self.queue.push_back(val)
    }
    /// Moves all elements of `other` into `self`, leaving `other` empty.
    pub fn append(&mut self, other: &mut LimVecDeque<T>) {
        assert_le!(self.queue.len() + other.len(), self.capacity);
        self.queue.append(&mut other.queue)
    }
    /// Returns the number of elements in the deque.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    /// Returns true if the deque is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    /// Removes the specified range from the deque in bulk, returning all removed elements as an iterator.
    pub fn drain<R>(&mut self, range: R) -> std::collections::vec_deque::Drain<T>
    where
        R: RangeBounds<usize>,
    {
        self.queue.drain(range)
    }
    /// Rearranges the internal storage of this deque so it is one contiguous slice, which is then returned.
    pub fn make_contiguous(&mut self) -> &mut [T] {
        self.queue.make_contiguous()
    }
    /// Gets the maximum capacity of the deque.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    /// Returns a pair of slices which contain, in order, the contents of the deque.
    pub fn as_slices(&self) -> (&[T], &[T]) {
        self.queue.as_slices()
    }
    /// Returns a front-to-back iterator.
    pub fn iter(&self) -> std::collections::vec_deque::Iter<T> {
        self.queue.iter()
    }
}

impl<T> From<Vec<T>> for LimVecDeque<T> {
    fn from(v: Vec<T>) -> Self {
        Self {
            capacity: v.len(),
            queue:    VecDeque::from(v),
        }
    }
}

impl<T> std::iter::FromIterator<T> for LimVecDeque<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let queue = VecDeque::from_iter(iter);
        let capacity = queue.len();
        Self { queue, capacity }
    }
}

impl<'a, T> IntoIterator for &'a LimVecDeque<T>
where
    T: Clone,
{
    type Item = T;

    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.queue.clone().into_iter()
    }
}

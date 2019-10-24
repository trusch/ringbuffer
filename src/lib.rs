use std::collections::VecDeque;

mod tests;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// RingBuffer implements a ring buffer with a fixed capacity
///
/// # Example
/// ```
/// let mut buf = ringbuffer::RingBuffer::new(32);
/// buf.push(1);
/// assert_eq!(buf.check_if_any(|x| *x == 1), true);
/// assert_eq!(buf.contains(&1), true);
/// assert_eq!(buf.for_each( |_| Ok(()) ).is_ok(), true);
/// ```
pub struct RingBuffer<T> {
    _data: VecDeque<T>,
    _cap: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(cap: usize) -> RingBuffer<T> {
        RingBuffer {
            _data: VecDeque::new(),
            _cap: cap,
        }
    }

    pub fn push(&mut self, data: T) {
        if self._data.len() == self._cap {
            self._data.pop_front();
        }
        self._data.push_back(data);
    }

    pub fn check_if_any<F>(&self, f: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        for item in self._data.iter() {
            if f(item) == true {
                return true;
            }
        }
        return false;
    }

    pub fn for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&T) -> Result<()>,
    {
        for item in self._data.iter() {
            f(item)?;
        }
        Ok(())
    }

    pub fn contains(&self, val: &T) -> bool
    where
        T: std::cmp::PartialEq,
    {
        for item in self._data.iter() {
            if *item == *val {
                return true;
            }
        }
        return false;
    }
}

use std::{cell::UnsafeCell, sync::atomic::AtomicU8};

use crate::aligned_buffer::*;

#[derive(Debug)]
pub(crate) struct State<T> {
    pub(crate) bufs: UnsafeCell<[Buffer<T>; 3]>,
    pub(crate) mid: AtomicU8,
}

impl<T> Default for State<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            bufs: UnsafeCell::default(),
            mid: 1.into(),
        }
    }
}

impl<T> State<T>
where T: Clone {
    pub(crate) fn new(initial: &T) -> Self {
        Self {
            bufs: UnsafeCell::new([
                Buffer(initial.clone()),
                Buffer(initial.clone()),
                Buffer(initial.clone()),
            ]),
            mid: 1.into()
        }
    }
}

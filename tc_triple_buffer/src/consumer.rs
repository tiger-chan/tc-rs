use std::sync::{atomic::Ordering, Arc};

use crate::{Buffer, State, DIRTY};

pub struct TripleBufferConsumer<T> {
    src: Arc<State<T>>,
    fwd: u8,
}

impl<T> TripleBufferConsumer<T> {
    pub(crate) fn new(src: Arc<State<T>>) -> Self {
        Self { src, fwd: 2 }
    }

    pub fn data(&mut self) -> &T {
        let mid = self.src.mid.load(Ordering::Relaxed);
        if mid & DIRTY == DIRTY {
            let fwd = self.fwd;
            self.fwd = self.src.mid.swap(fwd, Ordering::AcqRel) & !DIRTY;
        }

        let fwd = self.fwd;
        unsafe {
            let bufs: &[Buffer<T>; 3] = &*self.src.bufs.get();
            &bufs[fwd as usize]
        }
    }
}

unsafe impl<T> Send for TripleBufferConsumer<T> where T: Send {}
unsafe impl<T> Sync for TripleBufferConsumer<T> where T: Sync {}

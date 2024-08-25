use std::sync::{atomic::Ordering, Arc};

use crate::{Buffer, State, DIRTY};

pub struct TripleBufferProducer<T> {
    src: Arc<State<T>>,
    bck: u8,
}

impl<T> TripleBufferProducer<T> {
    pub(crate) fn new(src: Arc<State<T>>) -> Self {
        Self { src, bck: 0 }
    }

    pub fn commit(&mut self) {
        let bck = self.bck | DIRTY;
        self.bck = self.src.mid.swap(bck, Ordering::AcqRel) & !DIRTY;
    }

    pub fn data(&mut self) -> &mut T {
        let bck = self.bck;
        unsafe {
            let bufs: &mut [Buffer<T>; 3] = &mut *self.src.bufs.get();
            &mut bufs[bck as usize]
        }
    }
}

unsafe impl<T> Send for TripleBufferProducer<T> where T: Send {}
unsafe impl<T> Sync for TripleBufferProducer<T> where T: Sync {}

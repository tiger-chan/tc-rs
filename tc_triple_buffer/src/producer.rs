use std::sync::{atomic::Ordering, Arc};

use crate::{Buffer, State, DIRTY};

/// The `TripleBufferProducer` is responsible for writing data into one of the
/// three buffers in the triple-buffered system. The producer can access and
/// modify data, then commit it to signal that the data is ready for
/// consumption.
///
/// # Type Parameters
/// - `T`: The type of data stored in the triple buffer.
#[derive(Debug)]
pub struct TripleBufferProducer<T> {
    // Shared state that holds the three buffers and their current state.
    src: Arc<State<T>>,
    /// The index of the backup buffer, which is currently being modified.
    bck: u8,
}

impl<T> TripleBufferProducer<T> {
    pub(crate) fn new(src: Arc<State<T>>) -> Self {
        Self { src, bck: 0 }
    }

    /// Commits the data in the current backup buffer, marking it as dirty and
    /// ready for consumption by a consumer. The `DIRTY` flag is used to signal
    /// that the buffer has been updated.
    pub fn commit(&mut self) {
        let bck = self.bck | DIRTY;
        self.bck = self.src.mid.swap(bck, Ordering::AcqRel) & !DIRTY;
    }

    /// Provides mutable access to the data in the current backup buffer.
    /// # Returns
    /// A mutable reference to the data in the current backup buffer.
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

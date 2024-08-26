use std::sync::{atomic::Ordering, Arc};

use crate::{Buffer, State, DIRTY};

/// The `TripleBufferConsumer` is responsible for reading data from one of the
/// three buffers in the triple-buffered system. The consumer retrieves data
/// that was written by the producer and marked as committed.
///
/// # Type Parameters
/// - `T`: The type of data stored in the triple buffer.
pub struct TripleBufferConsumer<T> {
    /// Shared state that holds the three buffers and their current state.
    src: Arc<State<T>>,
    /// The index of the forward buffer, which contains the data that is being
    /// read.
    fwd: u8,
}

impl<T> TripleBufferConsumer<T> {
    pub(crate) fn new(src: Arc<State<T>>) -> Self {
        Self { src, fwd: 2 }
    }

    /// Provides immutable access to the data in the current forward buffer.
    ///
    /// This function checks if the middle buffer has been marked as dirty, and
    /// if so, swaps the buffer indices, retrieving the most up-to-date data
    /// for the consumer.
    ///
    /// # Returns
    /// A reference to the data in the current forward buffer.
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

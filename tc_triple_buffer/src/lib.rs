mod aligned_buffer;
mod buffer_state;
mod consumer;
mod producer;

use std::sync::Arc;

use crate::{aligned_buffer::*, buffer_state::*};

pub use crate::{consumer::*, producer::*};

pub(crate) const DIRTY: u8 = 0x8;

/// A triple-buffer pattern implementation for safely sharing data between producers and consumers.
///
/// The `TripleBuffer` provides a way for multiple threads to communicate through a set of buffers.
/// This pattern allows one thread to write data while another reads from it, with an additional
/// buffer to help with synchronization.
///
/// # Examples
///
/// ```rust
/// use tc_triple_buffer::*; 
/// use std::thread;
///
/// let TripleBuffer::<u64>(mut publisher, mut subscriber) = TripleBuffer::default();
///
/// let producer = thread::spawn(move || {
///     for i in 1..1000 {
///         *publisher.data() = i;
///         publisher.commit();
///     }
/// });
///
/// let mut prev = 0;
/// while prev != 999 {
///     let next = *subscriber.data();
///     assert!(prev <= next);
///     prev = next;
/// }
///
/// let _ = producer.join();
/// ```
pub struct TripleBuffer<T>(pub TripleBufferProducer<T>, pub TripleBufferConsumer<T>);

impl<T> Default for TripleBuffer<T>
where
    T: Default,
{
    fn default() -> Self {
        let state: Arc<State<T>> = Arc::default();
        Self(TripleBufferProducer::new(state.clone()), TripleBufferConsumer::new(state))
    }
}

impl<T> TripleBuffer<T>
where
    T: Clone,
{
    pub fn new(initial: &T) -> Self {
        let state = Arc::new(State::new(initial));
        Self(TripleBufferProducer::new(state.clone()), TripleBufferConsumer::new(state))
    }
}

#[cfg(test)]
mod test {
    use std::thread;

    use super::*;

    #[test]
    fn single_thread() {
        let TripleBuffer::<u64>(mut publisher, mut sub) = TripleBuffer::default();

        *publisher.data() = 42;

        assert_eq!(*sub.data(), 0);
        publisher.commit();

        assert_eq!(*sub.data(), 42);
    }

    #[test]
    fn multi_threaded() {
        let TripleBuffer::<u64>(mut publisher, mut sub) = TripleBuffer::default();

        let producer = thread::spawn(move || {
            for i in 1..1000 {
                *publisher.data() = i;
                publisher.commit();
            }
        });

        let mut prev = 0;
        while prev != 999 {
            let next = *sub.data();
            assert!(prev <= next);
            prev = next;
        }

        let _ = producer.join();
    }
}

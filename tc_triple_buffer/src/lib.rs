use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicUsize, Ordering},
};

const DIRTY: usize = 0x8000_0000_0000_0000;

#[repr(align(64))]
struct Buffer<T>(T);

#[repr(align(64))]
#[derive(Default)]
struct BufferIdx {
    val: AtomicUsize,
}

impl<T> Default for Buffer<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(T::default())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct TripleBuffer<T> {
    bufs: UnsafeCell<[Buffer<T>; 3]>,
    mid: BufferIdx,
}

impl<T> Default for TripleBuffer<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            bufs: UnsafeCell::new([Buffer::default(), Buffer::default(), Buffer::default()]),
            mid: BufferIdx::default(),
        }
    }
}

impl<'a, T> TripleBuffer<T> {
    pub fn pub_sub(&'a self) -> (TripleBufferProducer<'a, T>, TripleBufferConsumer<'a, T>) {
        (
            TripleBufferProducer::new(self),
            TripleBufferConsumer::new(self),
        )
    }
}

pub struct TripleBufferProducer<'a, T> {
    src: &'a TripleBuffer<T>,
    bck: usize,
}

impl<'a, T> TripleBufferProducer<'a, T> {
    fn new(src: &'a TripleBuffer<T>) -> Self {
        Self { src, bck: 0 }
    }

    pub fn commit(&mut self) {
        let bck = self.bck | DIRTY;
        self.bck = self.src.mid.val.swap(bck, Ordering::AcqRel) & !DIRTY;
    }

    pub fn data(&mut self) -> &mut T {
        let bck = self.bck;
        unsafe {
            let bufs: &mut [Buffer<T>; 3] = &mut *self.src.bufs.get();
            &mut bufs[bck]
        }
    }
}

unsafe impl<'a, T> Send for TripleBufferProducer<'a, T> where T: Send {}
unsafe impl<'a, T> Sync for TripleBufferProducer<'a, T> where T: Sync {}

pub struct TripleBufferConsumer<'a, T> {
    src: &'a TripleBuffer<T>,
    fwd: usize,
}

impl<'a, T> TripleBufferConsumer<'a, T> {
    fn new(src: &'a TripleBuffer<T>) -> Self {
        Self { src, fwd: 2 }
    }

    pub fn data(&mut self) -> &T {
        let mid = self.src.mid.val.load(Ordering::Relaxed);
        if mid & DIRTY == DIRTY {
            let fwd = self.fwd;
            self.fwd = self.src.mid.val.swap(fwd, Ordering::AcqRel) & !DIRTY;
        }

        let fwd = self.fwd;
        unsafe {
            let bufs: &[Buffer<T>; 3] = &*self.src.bufs.get();
            &bufs[fwd]
        }
    }
}

unsafe impl<'a, T> Send for TripleBufferConsumer<'a, T> where T: Send {}
unsafe impl<'a, T> Sync for TripleBufferConsumer<'a, T> where T: Sync {}

#[cfg(test)]
mod test {
    use std::thread;

    use super::*;

    #[test]
    fn single_thread() {
        let buffer = TripleBuffer::<u64>::default();
        let (mut publisher, mut sub) = buffer.pub_sub();

        *publisher.data() = 42;

        assert_eq!(*sub.data(), 0);
        publisher.commit();

        assert_eq!(*sub.data(), 42);
    }

    #[test]
    fn multi_threaded() {
        let buffer = TripleBuffer::<u64>::default();
        {
            let (mut publisher, mut sub) = buffer.pub_sub();

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
}

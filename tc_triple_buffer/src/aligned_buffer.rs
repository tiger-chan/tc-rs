use std::ops::{Deref, DerefMut};

#[repr(align(64))]
pub(crate) struct Buffer<T>(pub(crate) T);

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

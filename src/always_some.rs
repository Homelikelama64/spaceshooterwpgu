use std::ops::{Deref, DerefMut};

pub struct AlwaysSome<T> {
    value: Option<T>,
}

impl<T> AlwaysSome<T> {
    /// After calling this you should not use the AlwaysSome<T> anymore
    pub fn take(&mut self) -> T {
        self.value.take().unwrap()
    }
}

impl<T> From<T> for AlwaysSome<T> {
    fn from(value: T) -> Self {
        Self { value: Some(value) }
    }
}

impl<T> Deref for AlwaysSome<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

impl<T> DerefMut for AlwaysSome<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap()
    }
}

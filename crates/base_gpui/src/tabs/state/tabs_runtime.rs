use std::marker::PhantomData;

pub struct TabsRuntime<T: Clone + Eq + 'static> {
    value: PhantomData<T>,
}

impl<T: Clone + Eq + 'static> Clone for TabsRuntime<T> {
    fn clone(&self) -> Self {
        Self {
            value: PhantomData,
        }
    }
}

impl<T: Clone + Eq + 'static> Default for TabsRuntime<T> {
    fn default() -> Self {
        Self {
            value: PhantomData,
        }
    }
}

impl<T: Clone + Eq + 'static> TabsRuntime<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

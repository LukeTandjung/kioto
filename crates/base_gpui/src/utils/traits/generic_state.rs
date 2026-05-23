pub trait GenericState {
    fn new<T>(value: Option<T>) -> Self
    where
        T: Clone + Eq + 'static,
        Self: Sized
    {
        Self { value }
    }
}

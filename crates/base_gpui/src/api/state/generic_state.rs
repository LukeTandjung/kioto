pub trait GenericState {
    type Value: Clone + Eq + 'static;
    
    fn new(value: Option<Self::Value>) -> Self;

    fn get_value(&self) -> Option<&Self::Value>;

    fn set_value(&mut self, value: Option<Self::Value>);
}

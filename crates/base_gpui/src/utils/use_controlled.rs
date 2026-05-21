enum ValueOrFn<T> {
    Value(T),
    Fn(Box<dyn FnOnce(T) -> T>)
}

struct UseControlledProps<T> {
    controlled: Option<T>,
    default: Option<T>,
    name: String,
    state: Option<String>
}

impl<T> Default for UseControlledProps<T> {
    fn default() -> Self {
        Self {
            controlled: None,
            default: None,
            name: String::new(),
            state: Some(String::from("value"))
        }
    }
}

impl<T> UseControlledProps<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn use_controlled(&self) -> (T, ValueOrFn<T>) {
        let is_controlled: bool = self.controlled.is_some();
        let (value, set_value) = 
        
        
    } 
}

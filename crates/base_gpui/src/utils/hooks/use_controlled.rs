pub fn use_controlled<T: Clone + Equal + 'static>(
    controlled: Option<Option<T>>,
    default: Option<Option<T>>,
    id: SharedString
) -> (Option<Option<T>>, Box<dyn Fn(Option<Option<T>>)>) {
    let 
    
    let value = match controlled {
        Some(non_null_vaue) => Some(non_null_value),
        None => 
    }
        let (value_state, set_value_state) = self.use_state(|| default);

        let is_controlled = *is_controlled_ref.borrow();
        let value = if is_controlled {
            controlled
        } else {
            value_state()
        };

        let set_value_state: Rc<dyn Fn(Option<T>)> = Rc::from(set_value_state);
        let get_set_value = self.use_callback(
            {
                let set_value_state = set_value_state.clone();

                move || {
                    Box::new(move || {
                        let set_value_state = set_value_state.clone();
                        Rc::new(move |next: Option<T>| {
                            if !is_controlled {
                                set_value_state(next);
                            }
                        }) as Rc<dyn Fn(Option<T>)>
                    }) as Box<dyn Fn() -> Rc<dyn Fn(Option<T>)>>
                }
            },
            [is_controlled],
        );

        let set_value = get_set_value();
        (value, Box::new(move |next| set_value(next)))
    }
}

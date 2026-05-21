use std::rc::Rc;

use gpui_hooks::hooks::{UseCallbackHook, UseRefHook, UseStateHook};

pub trait UseControlledHook {
    fn use_controlled<T: Clone + 'static>(
        &self,
        controlled: Option<T>,
        default: Option<T>,
    ) -> (Option<T>, Box<dyn Fn(Option<T>)>);
}

impl<H> UseControlledHook for H
where
    H: UseCallbackHook + UseRefHook + UseStateHook,
{
    fn use_controlled<T: Clone + 'static>(
        &self,
        controlled: Option<T>,
        default: Option<T>,
    ) -> (Option<T>, Box<dyn Fn(Option<T>)>) {
        let is_controlled_ref = self.use_ref(|| controlled.is_some());
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

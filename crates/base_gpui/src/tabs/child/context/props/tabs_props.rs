use std::rc::Rc;

use gpui::{App, Window};

use super::TabsOrientation;

pub struct TabsProps<T: Clone + Eq + 'static> {
    orientation: TabsOrientation,
    on_value_change: Option<Rc<dyn Fn(Option<&T>, &mut Window, &mut App) + 'static>>,
}

impl<T: Clone + Eq + 'static> Clone for TabsProps<T> {
    fn clone(&self) -> Self {
        Self {
            orientation: self.orientation,
            on_value_change: self.on_value_change.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> TabsProps<T> {
    pub fn new(
        orientation: TabsOrientation,
        on_value_change: Option<Rc<dyn Fn(Option<&T>, &mut Window, &mut App) + 'static>>,
    ) -> Self {
        Self {
            orientation,
            on_value_change,
        }
    }

    pub fn orientation(&self) -> TabsOrientation {
        self.orientation
    }

    pub fn on_value_change(
        &self,
    ) -> Option<&Rc<dyn Fn(Option<&T>, &mut Window, &mut App) + 'static>> {
        self.on_value_change.as_ref()
    }
}

use std::collections::BTreeMap;

use gpui::SharedString;

use crate::field::FieldValue;

#[derive(Clone, Debug, PartialEq)]
pub enum FormValue {
    Empty,
    Present,
    Bool(bool),
    Text(SharedString),
    Number(f64),
}

impl From<FieldValue> for FormValue {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::Empty => Self::Empty,
            FieldValue::Present => Self::Present,
            FieldValue::Bool(value) => Self::Bool(value),
            FieldValue::Text(value) => Self::Text(value),
        }
    }
}

pub type FormValues = BTreeMap<SharedString, FormValue>;

use std::ops::{Deref,DerefMut};
use serde_json::Value;

#[derive(Clone)]
pub struct JSValue(Value);


impl druid::Data for JSValue {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Deref for JSValue {
    type Target=Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JSValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

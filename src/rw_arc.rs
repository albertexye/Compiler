use serde::Serialize;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

#[derive(Debug, Serialize)]
pub(crate) struct RwArc<T>(Arc<RwLock<T>>)
where
    T: PartialEq + Serialize;

impl<T: PartialEq + Serialize> PartialEq for RwArc<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_data = self.0.read().unwrap();
        let other_data = other.0.read().unwrap();
        *self_data == *other_data
    }
}

impl<T: PartialEq + Serialize> Deref for RwArc<T> {
    type Target = Arc<RwLock<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: PartialEq + Serialize> Clone for RwArc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: PartialEq + Serialize> RwArc<T> {
    pub(crate) fn new(data: T) -> Self {
        Self(Arc::new(RwLock::new(data)))
    }
}

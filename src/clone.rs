
use crate::Map;

impl<V: Clone> Clone for Map<V> {
    fn clone(&self) -> Self {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't clone() non-initialized Map");
        let mut m = Self::with_capacity_none(self.capacity());
        for (k, v) in self.iter() {
            m.insert(k, v.clone());
        }
        m
    }
}

#[cfg(test)]
#[path = "clone_tests.rs"]
mod tests;

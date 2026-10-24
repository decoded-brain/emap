
use crate::Map;

impl<V> Map<V> {
    #[inline]
    #[must_use]
    pub fn next_key(&self) -> usize {
        if self.first_free.is_def() {
            self.first_free.get()
        } else {
            panic!("No more keys available left");
        }
    }
}

#[cfg(test)]
#[path = "next_key_tests.rs"]
mod tests;

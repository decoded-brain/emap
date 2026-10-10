// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;
use crate::{IntoValues, Values};
use std::marker::PhantomData;

impl<'a, V: 'a> Iterator for Values<'a, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_def() {
            let node = unsafe { &*self.head.add(self.current.get()) };
            self.current = node.get_next();
            node.get()
        } else {
            None
        }
    }
}

impl<'a, V: Clone + 'a> Iterator for IntoValues<'a, V> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_def() {
            // This iterator shares the map with get()/values() references;
            // creating a mutable node reference would invalidate those borrows.
            let node = unsafe { &*self.head.add(self.current.get()) };
            self.current = node.get_next();
            Some(node.get().unwrap().clone())
        } else {
            None
        }
    }
}

impl<V> Map<V> {
    /// Make an iterator over all values.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn values(&self) -> Values<'_, V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't values() non-initialized Map");
        Values {
            current: self.first_used,
            head: self.head,
            _marker: PhantomData,
        }
    }

    /// Make an into-iterator over all items.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn into_values(&self) -> IntoValues<'_, V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't into_values() non-initialized Map");
        IntoValues {
            current: self.first_used,
            head: self.head,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
#[path = "values_tests.rs"]
mod tests;

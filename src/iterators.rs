// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{IntoIter, Iter, IterMut, Map};
use std::marker::PhantomData;

impl<'a, V: Clone + 'a> Iterator for Iter<'a, V> {
    type Item = (usize, &'a V);

    /// This is an implementation of the `next` function that returns the next item in an iterator if it
    /// exists, or `None` if the end of the iterator has been reached.
    ///
    /// Returns:
    ///
    /// The `next` function returns an `Option` that contains a tuple of `(i, p)` where `i` is the index of
    /// the item and `p` is a reference to the item. If there are no more items to iterate over, it returns
    /// `None`.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_def() {
            let node = unsafe { &*self.head.add(self.current.get()) };
            let mut next = node.get_next();
            std::mem::swap(&mut self.current, &mut next);
            Some((next.get(), node.get().unwrap()))
        } else {
            None
        }
    }
}

impl<'a, V: Clone + 'a> Iterator for IterMut<'a, V> {
    type Item = (usize, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_def() {
            // The map is borrowed exclusively and each used slot is visited once;
            // advancing to disjoint nodes keeps earlier mutable values valid.
            let node = unsafe { &mut *self.head.add(self.current.get()) };
            let mut next = node.get_next();
            std::mem::swap(&mut self.current, &mut next);
            Some((next.get(), node.get_mut().unwrap()))
        } else {
            None
        }
    }
}

impl<'a, V: Clone + 'a> Iterator for IntoIter<'a, V> {
    type Item = (usize, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_def() {
            // Cloning through a shared map must preserve existing value borrows;
            // a mutable node reference would invalidate them without any write.
            let node = unsafe { &*self.head.add(self.current.get()) };
            let mut next = node.get_next();
            std::mem::swap(&mut self.current, &mut next);
            Some((next.get(), node.get().unwrap().clone()))
        } else {
            None
        }
    }
}

impl<'a, V: Clone> IntoIterator for &'a Map<V> {
    type Item = (usize, V);
    type IntoIter = IntoIter<'a, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            current: self.first_used,
            head: self.head,
            _marker: PhantomData,
        }
    }
}

impl<V: Clone> Map<V> {
    /// Make an iterator over all items.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> Iter<'_, V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't iter() non-initialized Map");
        Iter {
            current: self.first_used,
            head: self.head,
            _marker: PhantomData,
        }
    }
    /// Make a mutable iterator over all items.
    ///
    /// For example:
    ///
    /// ```
    /// use emap::Map;
    /// let mut m: Map<String> = Map::with_capacity_none(16);
    /// m.insert(0, "Jeff".to_string());
    /// m.insert(1, "Lebowski".to_string());
    /// for (_, v) in m.iter_mut() {
    ///   *v = v.to_lowercase();
    /// }
    /// ```
    ///
    /// The keys are not mutable, only the values, for obvious reasons.
    /// The map is borrowed exclusively while the iterator or its values are in use.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    // Keep mutation explicit: an IntoIterator impl for &mut Map could change
    // borrowed .into_iter() dispatch and its existing cloned-value contract.
    #[allow(clippy::iter_without_into_iter)]
    // A shared receiver permits aliases with map reads or another mutable iterator;
    // exclusive borrowing prevents those references from invalidating each other.
    pub const fn iter_mut(&mut self) -> IterMut<'_, V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't iter_mut() non-initialized Map");
        IterMut {
            current: self.first_used,
            head: self.head,
            _marker: PhantomData,
        }
    }

    /// Make an iterator over all items.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn into_iter(&self) -> IntoIter<'_, V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't into_iter() non-initialized Map");
        IntoIter {
            current: self.first_used,
            head: self.head,
            _marker: PhantomData,
        }
    }
}

#[test]
fn insert_and_iterate() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.insert(1, "two");
    m.insert(2, "three");
    let mut sum = 0;
    let mut count = 0;
    for (k, _v) in m.iter() {
        sum += k;
        count += 1;
    }
    assert_eq!(3, count);
    assert_eq!(3, sum);
}

#[cfg(test)]
#[path = "iterators_tests.rs"]
mod tests;

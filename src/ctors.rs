// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{Map, Node, NodeId};
use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};
use std::mem;
use std::ptr::{self, NonNull};

impl<V> Drop for Map<V> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        if self.initialized {
            unsafe { self.drop_all_initialized_nodes() };
        } else {
            unsafe { self.drop_used_nodes() };
        }

        #[cfg(not(debug_assertions))]
        unsafe {
            self.drop_used_nodes();
        }

        // Zero-capacity maps use an aligned dangling pointer, not an allocation;
        // only pointers returned by alloc with this nonzero layout may be freed.
        if self.layout.size() != 0 {
            unsafe {
                dealloc(self.head.cast(), self.layout);
            }
        }
    }
}

impl<V> Map<V> {
    /// Create a map with the given capacity without initializing values.
    ///
    /// # Panics
    ///
    /// Panics on allocation error.
    #[inline]
    #[must_use]
    fn with_capacity(cap: usize) -> Self {
        let layout = Layout::array::<Node<V>>(cap).expect("invalid layout");
        // GlobalAlloc rejects zero-sized layouts; an empty map never dereferences
        // its aligned dangling pointer, and Drop must not pass it to dealloc.
        let head = if layout.size() == 0 {
            NonNull::<Node<V>>::dangling().as_ptr()
        } else {
            let allocation = unsafe { alloc(layout) };
            if allocation.is_null() {
                handle_alloc_error(layout);
            }
            allocation.cast()
        };
        Self {
            first_free: NodeId::new(NodeId::UNDEF),
            first_used: NodeId::new(NodeId::UNDEF),
            layout,
            head,
            len: 0,
            #[cfg(debug_assertions)]
            initialized: false,
        }
    }

    /// Create a map and initialize all slots with `None`.
    ///
    /// More expensive than `with_capacity` since it initializes every slot.
    ///
    /// # Panics
    ///
    /// Panics on allocation error.
    #[inline]
    #[must_use]
    pub fn with_capacity_none(cap: usize) -> Self {
        let mut m = Self::with_capacity(cap);
        m.init_with_none();
        #[cfg(debug_assertions)]
        {
            m.initialized = true;
        }
        m
    }

    /// Initialize all slots as free and link the free-list.
    #[inline]
    fn init_with_none(&mut self) {
        let cap = self.capacity();
        // An empty allocation has no valid key; advertising slot zero would let
        // next_key() return an index outside the map's capacity.
        self.first_free = NodeId::new(if cap == 0 { NodeId::UNDEF } else { 0 });
        let mut p = self.head;
        for i in 0..cap {
            let free_next = if i + 1 == cap { NodeId::UNDEF } else { i + 1 };
            let free_prev = if i == 0 { NodeId::UNDEF } else { i - 1 };
            let node = Node::new(free_next, free_prev, None);
            unsafe {
                ptr::write(p, node);
                p = p.add(1);
            }
        }
    }

    /// Return the map capacity.
    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.layout.size() / mem::size_of::<Node<V>>()
    }
}

impl<V: Clone> Map<V> {
    /// Create a map and fill all slots with the provided value.
    ///
    /// # Panics
    ///
    /// Panics on allocation error or if `Clone` panics during initialization.
    #[inline]
    #[must_use]
    pub fn with_capacity_some(cap: usize, v: V) -> Self {
        let mut m = Self::with_capacity(cap);
        m.fill_with_some(v);
        #[cfg(debug_assertions)]
        {
            m.initialized = true;
        }
        m
    }

    /// Replace all slots with `Some(v.clone())` and build the used-list.
    ///
    /// Existing values are retained if cloning the replacement panics.
    ///
    /// # Panics
    ///
    /// Panics if `cap` differs from the map capacity, on allocation error,
    /// or if `Clone` panics during initialization.
    #[inline]
    pub fn init_with_some(&mut self, cap: usize, v: V) {
        // This map owns exactly capacity() slots; a mismatched count can write
        // beyond the allocation or leave existing slots outside the rebuilt lists.
        assert_eq!(
            cap,
            self.capacity(),
            "Initialization capacity must match the map"
        );
        // The replacement owns every completed clone, so unwinding drops its
        // partial used-list and preserves the old map; exchange ownership before
        // dropping the old values so their destruction cannot invalidate self.
        let replacement = Self::with_capacity_some(cap, v);
        let previous = mem::replace(self, replacement);
        drop(previous);
    }

    /// Initialize a fresh allocation while recording ownership of every clone.
    #[inline]
    fn fill_with_some(&mut self, v: V) {
        let cap = self.capacity();
        let mut previous_used = NodeId::new(NodeId::UNDEF);
        self.first_free = NodeId::new(NodeId::UNDEF);
        self.first_used = NodeId::new(NodeId::UNDEF);
        self.len = 0;

        for index in 0..cap {
            let cloned = v.clone();
            let node = Node::new(NodeId::UNDEF, previous_used.get(), Some(cloned));
            // Only the fresh constructor calls this helper: index is in bounds
            // and the slot owns no value yet, so writing cannot leak an old value.
            unsafe {
                ptr::write(self.head.add(index), node);
            }
            if previous_used.is_def() {
                // The preceding slot was initialized before it became reachable;
                // linking each completed clone lets Drop reclaim a prefix on panic.
                unsafe {
                    let previous_node = &mut *self.head.add(previous_used.get());
                    previous_node.update_next(NodeId::new(index));
                }
            } else {
                self.first_used = NodeId::new(index);
            }
            previous_used = NodeId::new(index);
            self.len = index + 1;
        }
    }
}

impl<V> Map<V> {
    /// Drop values reachable through the used-list only.
    ///
    /// # Safety
    ///
    /// `self.head` must point to a valid array of `Node<V>` of size `capacity()`.
    #[inline]
    unsafe fn drop_used_nodes(&mut self) {
        let mut current = self.first_used;
        while current.is_def() {
            let node = unsafe { &mut *self.head.add(current.get()) };
            if let Some(value) = node.take_value() {
                drop(value);
            }
            current = node.get_next();
        }
    }

    /// Drop all values in initialized slots regardless of list integrity.
    ///
    /// Enabled only in debug builds to detect leaks when invariants are violated.
    ///
    /// # Safety
    ///
    /// `self.head` must point to a valid array of `Node<V>` of size `capacity()`.
    #[cfg(debug_assertions)]
    #[inline]
    unsafe fn drop_all_initialized_nodes(&mut self) {
        for index in 0..self.capacity() {
            let node = unsafe { &mut *self.head.add(index) };
            if let Some(value) = node.take_value() {
                drop(value);
            }
        }
    }
}

#[cfg(test)]
#[path = "ctors_tests.rs"]
mod tests;

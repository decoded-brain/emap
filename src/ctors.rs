
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

        if self.layout.size() != 0 {
            unsafe {
                dealloc(self.head.cast(), self.layout);
            }
        }
    }
}

impl<V> Map<V> {
    #[inline]
    #[must_use]
    fn with_capacity(cap: usize) -> Self {
        let layout = Layout::array::<Node<V>>(cap).expect("invalid layout");
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

    #[inline]
    fn init_with_none(&mut self) {
        let cap = self.capacity();
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

    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.layout.size() / mem::size_of::<Node<V>>()
    }
}

impl<V: Clone> Map<V> {
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

    #[inline]
    pub fn init_with_some(&mut self, cap: usize, v: V) {
        assert_eq!(
            cap,
            self.capacity(),
            "Initialization capacity must match the map"
        );
        let replacement = Self::with_capacity_some(cap, v);
        let previous = mem::replace(self, replacement);
        drop(previous);
    }

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
            unsafe {
                ptr::write(self.head.add(index), node);
            }
            if previous_used.is_def() {
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

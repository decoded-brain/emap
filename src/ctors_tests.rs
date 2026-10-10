// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::*;
use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;

/// Out-of-bounds insert must panic in debug builds.
#[test]
#[should_panic(expected = "The key 5 is over the boundary 1")]
#[cfg(debug_assertions)]
fn insert_out_of_boundary() {
    let mut m: Map<&str> = Map::with_capacity(1);
    m.insert(5, "one");
}

/// Out-of-bounds get must panic in debug builds.
#[test]
#[should_panic(expected = "The key 5 is over the boundary 1")]
#[cfg(debug_assertions)]
fn get_out_of_boundary() {
    let m: Map<&str> = Map::with_capacity(1);
    m.get(5).unwrap();
}

/// Out-of-bounds remove must panic in debug builds.
#[test]
#[should_panic(expected = "The key 5 is over the boundary 1")]
#[cfg(debug_assertions)]
fn remove_out_of_boundary() {
    let mut m: Map<&str> = Map::with_capacity(1);
    m.remove(5);
}

/// Memory layout size must reflect Node<V> array size.
#[test]
fn calculates_size_of_memory() {
    let m1: Map<u8> = Map::with_capacity_none(8);
    assert_eq!(24 * 8, m1.layout.size());
    let m2: Map<bool> = Map::with_capacity_none(8);
    assert_eq!(24 * 8, m2.layout.size());
}

/// New map must start empty.
#[test]
fn makes_new_map() {
    let m: Map<&str> = Map::with_capacity_none(16);
    assert_eq!(0, m.len());
}

/// Capacity must match constructor argument.
#[test]
fn returns_capacity() {
    let m: Map<&str> = Map::with_capacity_none(16);
    assert_eq!(16, m.capacity());
}

/// Contains check must return false for uninitialized slot.
#[test]
fn with_init() {
    let m: Map<&str> = Map::with_capacity_none(16);
    assert!(!m.contains_key(8));
}

/// Dropping an empty map should be safe.
#[test]
fn drops_correctly() {
    let m: Map<Vec<u8>> = Map::with_capacity_none(16);
    assert_eq!(0, m.len());
    drop(m);
}

/// Values must be dropped on map drop.
#[test]
fn drops_values() {
    let mut m: Map<Rc<()>> = Map::with_capacity_none(1);
    let v = Rc::new(());
    m.insert(0, Rc::clone(&v));
    drop(m);
    assert_eq!(Rc::strong_count(&v), 1);
}

/// Multiple values must be dropped on map drop.
#[test]
fn drops_multiple_values() {
    let mut m: Map<Rc<()>> = Map::with_capacity_none(3);
    let a = Rc::new(());
    let b = Rc::new(());
    let c = Rc::new(());

    m.insert(0, Rc::clone(&a));
    m.insert(1, Rc::clone(&b));
    m.insert(2, Rc::clone(&c));

    drop(m);

    assert_eq!(Rc::strong_count(&a), 1);
    assert_eq!(Rc::strong_count(&b), 1);
    assert_eq!(Rc::strong_count(&c), 1);
}

/// Values must not leak after repeated insert/remove cycles.
#[test]
fn drops_values_after_remove_cycles() {
    let mut m: Map<Rc<()>> = Map::with_capacity_none(2);
    let value = Rc::new(());

    for _ in 0..3 {
        m.insert(0, Rc::clone(&value));
        m.remove(0);
        assert_eq!(Rc::strong_count(&value), 1);
    }

    m.insert(0, Rc::clone(&value));
    drop(m);
    assert_eq!(Rc::strong_count(&value), 1);
}

#[derive(Clone, PartialEq, Eq)]
struct Foo {
    pub t: i32,
}

/// Struct values must initialize as expected.
#[test]
fn init_with_structs() {
    let m: Map<Foo> = Map::with_capacity_none(16);
    assert_eq!(16, m.capacity());
}

/// `with_capacity_some` must fill all slots and set len accordingly.
#[test]
fn init_with_some() {
    let m: Map<Foo> = Map::with_capacity_some(16, Foo { t: 42 });
    assert_eq!(16, m.capacity());
    assert_eq!(16, m.len());
}

/// Zero capacity must be supported.
#[test]
fn init_with_empty() {
    let m: Map<Foo> = Map::with_capacity_some(0, Foo { t: 42 });
    assert_eq!(0, m.capacity());
    assert_eq!(0, m.len());
}

/// Partial initialization that panics must still be drop-safe.
#[test]
fn drop_after_boundary_panic_without_initialization() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let mut map: Map<&str> = Map::with_capacity(1);
        map.insert(1, "boom");
    }));
    assert!(result.is_err());
}

#[derive(Debug)]
struct PanicOnClone {
    clones: Rc<Cell<usize>>,
    active: Rc<Cell<usize>>,
    panic_after: usize,
}

impl PanicOnClone {
    fn new(panic_after: usize, clones: Rc<Cell<usize>>, active: Rc<Cell<usize>>) -> Self {
        active.set(active.get() + 1);
        Self {
            clones,
            active,
            panic_after,
        }
    }
}

impl Clone for PanicOnClone {
    fn clone(&self) -> Self {
        assert!(self.clones.get() < self.panic_after, "clone limit reached");
        self.clones.set(self.clones.get() + 1);
        self.active.set(self.active.get() + 1);
        Self {
            clones: Rc::clone(&self.clones),
            active: Rc::clone(&self.active),
            panic_after: self.panic_after,
        }
    }
}

impl Drop for PanicOnClone {
    fn drop(&mut self) {
        let current = self.active.get();
        self.active.set(current.saturating_sub(1));
    }
}

/// If cloning panics during `with_capacity_some`, no values must leak.
#[test]
fn drop_after_partial_with_capacity_some_panics() {
    let clones = Rc::new(Cell::new(0));
    let active = Rc::new(Cell::new(0));

    let result = catch_unwind(AssertUnwindSafe(|| {
        let value = PanicOnClone::new(1, Rc::clone(&clones), Rc::clone(&active));
        let _ = Map::with_capacity_some(3, value);
    }));

    assert!(result.is_err());
    assert_eq!(clones.get(), 1);
    assert_eq!(active.get(), 0);
}

#[derive(Clone)]
struct CountedValue {
    value: usize,
    drops: Rc<Cell<usize>>,
}

impl Drop for CountedValue {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

#[test]
fn reinitialization_drops_previous_values_once() {
    let previous_drops = Rc::new(Cell::new(0));
    let replacement_drops = Rc::new(Cell::new(0));
    let mut map = Map::with_capacity_none(4);
    for key in [0, 3] {
        map.insert(
            key,
            CountedValue {
                value: key,
                drops: Rc::clone(&previous_drops),
            },
        );
    }

    map.init_with_some(
        4,
        CountedValue {
            value: 7,
            drops: Rc::clone(&replacement_drops),
        },
    );

    assert_eq!(previous_drops.get(), 2);
    assert_eq!(replacement_drops.get(), 1);
    assert_eq!(map.len(), 4);
    for key in 0..4 {
        assert_eq!(map.get(key).unwrap().value, 7);
    }
    map.remove(2);
    assert_eq!(replacement_drops.get(), 2);
    drop(map);
    assert_eq!(previous_drops.get(), 2);
    assert_eq!(replacement_drops.get(), 5);
}

#[test]
fn smaller_initialization_capacity_is_rejected_without_changes() {
    let mut map = Map::with_capacity_none(3);
    map.insert(0, String::from("first"));
    map.insert(2, String::from("last"));

    let result = catch_unwind(AssertUnwindSafe(|| {
        map.init_with_some(2, String::from("replacement"));
    }));

    assert!(result.is_err());
    assert_eq!(map.capacity(), 3);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(0).unwrap(), "first");
    assert_eq!(map.get(2).unwrap(), "last");
    assert_eq!(map.next_key(), 1);
}

#[test]
fn panicking_reinitialization_preserves_old_values_and_drops_new_clones() {
    let previous_clones = Rc::new(Cell::new(0));
    let previous_active = Rc::new(Cell::new(0));
    let replacement_clones = Rc::new(Cell::new(0));
    let replacement_active = Rc::new(Cell::new(0));
    let mut map = Map::with_capacity_none(3);
    for key in [0, 2] {
        map.insert(
            key,
            PanicOnClone::new(
                usize::MAX,
                Rc::clone(&previous_clones),
                Rc::clone(&previous_active),
            ),
        );
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        map.init_with_some(
            3,
            PanicOnClone::new(
                1,
                Rc::clone(&replacement_clones),
                Rc::clone(&replacement_active),
            ),
        );
    }));

    assert!(result.is_err());
    assert_eq!(replacement_clones.get(), 1);
    assert_eq!(replacement_active.get(), 0);
    assert_eq!(previous_active.get(), 2);
    assert_eq!(map.len(), 2);
    assert!(Rc::ptr_eq(&map.get(0).unwrap().active, &previous_active));
    assert!(Rc::ptr_eq(&map.get(2).unwrap().active, &previous_active));
    assert_eq!(map.next_key(), 1);
    drop(map);
    assert_eq!(previous_active.get(), 0);
}

#[test]
fn larger_initialization_capacity_is_rejected_without_changes() {
    let mut map = Map::with_capacity_none(1);
    map.insert(0, String::from("original"));

    let result = catch_unwind(AssertUnwindSafe(|| {
        map.init_with_some(2, String::from("replacement"));
    }));

    assert!(result.is_err());
    assert_eq!(map.capacity(), 1);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(0).unwrap(), "original");
}

#[test]
fn zero_capacity_has_no_usable_keys() {
    let mut map = Map::<()>::with_capacity_none(0);

    assert_eq!(map.capacity(), 0);
    assert!(map.is_empty());
    assert_eq!(map.keys().count(), 0);
    assert_eq!(map.values().count(), 0);
    assert_eq!(map.into_values().count(), 0);
    assert!(catch_unwind(AssertUnwindSafe(|| map.next_key())).is_err());
    assert!(catch_unwind(AssertUnwindSafe(|| map.insert(0, ()))).is_err());

    map.init_with_some(0, ());
    assert!(map.is_empty());
    assert!(catch_unwind(AssertUnwindSafe(|| map.next_key())).is_err());
    drop(map);
}

#[test]
fn zero_capacity_drops_input_without_cloning() {
    let drops = Rc::new(Cell::new(0));
    let map = Map::with_capacity_some(
        0,
        CountedValue {
            value: 7,
            drops: Rc::clone(&drops),
        },
    );

    assert_eq!(drops.get(), 1);
    assert_eq!(map.capacity(), 0);
    assert!(map.is_empty());
    drop(map);
    assert_eq!(drops.get(), 1);
}

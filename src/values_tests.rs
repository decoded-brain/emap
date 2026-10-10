// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::*;
#[test]
fn empty_values() {
    let m: Map<u32> = Map::with_capacity_none(16);
    assert!(m.values().next().is_none());
}

#[test]
fn simple_values() {
    let mut m: Map<u32> = Map::with_capacity_none(3);
    m.insert(0, 2);
    m.insert(1, 1);
    m.insert(2, 0);
    let items: Vec<_> = m.values().collect();
    assert_eq!(items, vec![&0, &1, &2]);
}

#[test]
fn values_insert() {
    let mut m: Map<u32> = Map::with_capacity_none(6);
    assert!(m.values().next().is_none());
    m.insert(0, 4);
    assert_eq!(*m.values().next().unwrap(), 4);
    m.insert(2, 2);
    m.insert(4, 1);

    let items: Vec<_> = m.values().collect();
    assert_eq!(items, vec![&1, &2, &4]);
}

#[test]
fn values_remove() {
    let mut m: Map<u32> = Map::with_capacity_none(6);
    assert!(m.values().next().is_none());
    m.insert(0, 4);
    assert_eq!(*m.values().next().unwrap(), 4);
    m.insert(2, 2);
    m.insert(4, 1);
    m.remove(2);
    m.insert(4, 5);
    m.remove(0);
    let items: Vec<_> = m.values().collect();
    assert_eq!(items, vec![&5]);
    m.remove(4);
    let items: Vec<_> = m.values().collect();
    assert!(items.is_empty());
}

#[test]
fn values_clear() {
    let mut m: Map<u32> = Map::with_capacity_none(6);
    assert!(m.values().next().is_none());
    m.insert(0, 4);
    assert_eq!(*m.values().next().unwrap(), 4);
    m.insert(2, 2);
    m.insert(4, 1);
    m.remove(2);
    m.insert(4, 5);
    m.clear();
    let items: Vec<_> = m.values().collect();
    assert!(items.is_empty());
}

#[test]
fn insert_and_jump_over_next() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "foo");
    let mut values = m.into_values();
    assert_eq!("foo", values.next().unwrap());
    assert!(values.next().is_none());
}

#[test]
fn count_them_all() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.insert(1, "two");
    m.insert(2, "three");
    assert_eq!(3, m.into_values().count());
}

#[test]
fn into_values_empty() {
    let map: Map<u32> = Map::with_capacity_none(10);
    let mut iter = map.into_values();
    assert_eq!(iter.next(), None);
}

#[test]
fn into_values_basic() {
    let mut map = Map::with_capacity_none(5);
    map.insert(1, 30);
    map.insert(3, 10);

    let items: Vec<_> = map.into_values().collect();
    assert_eq!(items, vec![10, 30]);
}

#[test]
fn into_values_full() {
    let mut map = Map::with_capacity_some(3, 0);
    map.insert(0, 100);
    map.insert(1, 200);
    map.insert(2, 300);

    let mut iter = map.into_values();
    assert_eq!(iter.next(), Some(100));
    assert_eq!(iter.next(), Some(200));
    assert_eq!(iter.next(), Some(300));
    assert_eq!(iter.next(), None);
}

#[test]
fn into_values_mixed() {
    let mut map = Map::with_capacity_none(4);
    map.insert(1, 'a');
    map.insert(3, 'b');

    let items: Vec<_> = map.into_values().collect();
    assert_eq!(items, vec!['b', 'a']);
}

#[test]
fn into_values_consumes() {
    let mut map = Map::with_capacity_none(2);
    map.insert(0, String::from("test"));

    let items: Vec<_> = map.into_values().collect();
    assert_eq!(items, vec!["test"]);
}

#[test]
fn into_values_with_gaps() {
    let mut map = Map::with_capacity_none(5);
    map.insert(4, 400);
    map.insert(1, 100);

    let items: Vec<_> = map.into_values().collect();
    assert_eq!(items.len(), 2);
    assert!(items.contains(&100));
    assert!(items.contains(&400));
}

#[test]
fn cloned_values_preserve_shared_references_and_originals() {
    let mut map = Map::with_capacity_none(3);
    map.insert(0, String::from("first"));
    map.insert(2, String::from("last"));
    let first = map.get(0).unwrap();
    let borrowed: Vec<_> = map.values().collect();
    let mut cloned: Vec<_> = map.into_values().collect();

    assert_eq!(cloned, ["last", "first"]);
    cloned[1].push_str(" changed");
    assert_eq!(first, "first");
    assert_eq!(borrowed[0], "last");
    assert_eq!(borrowed[1], "first");
    assert_eq!(map.len(), 2);
}

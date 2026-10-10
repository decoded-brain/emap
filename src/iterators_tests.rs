// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use super::*;

#[test]
fn empty_iterator() {
    let m: Map<u32> = Map::with_capacity_none(16);
    assert!(m.into_iter().next().is_none());
}

#[test]
fn insert_and_jump_over_next() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "foo");
    let mut iter = m.into_iter();
    assert_eq!("foo", iter.next().unwrap().1);
    assert!(iter.next().is_none());
}

#[test]
fn insert_and_into_iterate() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.insert(1, "two");
    m.insert(2, "three");
    let mut sum = 0;
    let mut count = 0;
    for (k, _v) in m.into_iter() {
        sum += k;
        count += 1;
    }
    assert_eq!(3, count);
    assert_eq!(3, sum);
}

#[test]
fn iterate_without_function() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "test");
    let mut count = 0;
    for (_, _) in &m {
        count += 1;
    }
    assert_eq!(1, count);
}

#[test]
fn cloned_entries_preserve_shared_references_and_originals() {
    let mut map = Map::with_capacity_none(3);
    map.insert(0, String::from("first"));
    map.insert(2, String::from("last"));
    let first = map.get(0).unwrap();
    let borrowed: Vec<_> = map.values().collect();
    let cloned: Vec<_> = map.into_iter().collect();
    let cloned_from_trait: Vec<_> = IntoIterator::into_iter(&map).collect();

    assert_eq!(
        cloned,
        [(2, String::from("last")), (0, String::from("first"))]
    );
    assert_eq!(cloned_from_trait, cloned);
    assert_eq!(first, "first");
    assert_eq!(borrowed[0], "last");
    assert_eq!(borrowed[1], "first");
    assert_eq!(map.len(), 2);
}

#[test]
fn mutable_iteration_keeps_distinct_values_live_without_losing_ownership() {
    let mut map = Map::with_capacity_none(5);
    map.insert(1, String::from("one"));
    map.insert(3, String::from("three"));
    map.insert(4, String::from("four"));
    let mut iterator = map.iter_mut();
    let (first_key, first) = iterator.next().unwrap();
    let (second_key, second) = iterator.next().unwrap();
    let (third_key, third) = iterator.next().unwrap();

    assert_eq!((first_key, second_key, third_key), (4, 3, 1));
    first.push_str(" changed");
    second.push_str(" changed");
    third.push_str(" changed");
    assert!(iterator.next().is_none());
    assert_eq!(first, "four changed");
    assert_eq!(second, "three changed");
    assert_eq!(third, "one changed");

    assert_eq!(map.len(), 3);
    assert_eq!(map.get(1).unwrap(), "one changed");
    assert_eq!(map.get(3).unwrap(), "three changed");
    assert_eq!(map.get(4).unwrap(), "four changed");
}

#[test]
fn iterate_and_mutate() {
    let mut m: Map<u64> = Map::with_capacity_none(16);
    m.insert(0, 16);
    m.insert(1, 32);
    m.insert(2, 64);
    for (_, v) in m.iter_mut() {
        *v += 1;
    }
    let mut sum = 0;
    for v in m.values() {
        sum += v;
    }
    assert_eq!(115, sum);
}

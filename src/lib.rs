

#![doc(html_root_url = "https://docs.rs/emap/0.0.0")]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_inherent_impl)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::missing_panics_doc)]

mod clone;
mod ctors;
mod debug;
mod index;
mod iterators;
mod keys;
mod map;
mod next_key;
pub mod node;
#[cfg(feature = "serde")]
mod serialization;
mod values;

use crate::node::{Node, NodeId};

use std::alloc::Layout;
use std::marker::PhantomData;
pub struct Map<V> {
    first_free: NodeId,
    first_used: NodeId,
    head: *mut Node<V>,
    layout: Layout,
    len: usize,
    #[cfg(debug_assertions)]
    initialized: bool,
}

pub struct Iter<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a V>,
}

pub struct IterMut<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a mut V>,
}

pub struct IntoIter<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a Map<V>>,
}

pub struct Values<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a V>,
}

pub struct IntoValues<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a Map<V>>,
}

pub struct Keys<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a Map<V>>,
}

#[cfg(test)]
use std::time::Instant;

#[test]
fn perf() {
    let cap = 256;
    let mut m: Map<&str> = Map::with_capacity_none(cap);
    let start = Instant::now();
    for _ in 0..1000 {
        m.clear();
        for _ in 0..cap {
            m.push("Hello, world!");
        }
        for i in 0..cap {
            m.remove(i);
        }
        let keys: Vec<usize> = m.into_iter().map(|(k, _)| k).collect();
        for k in keys {
            m.remove(k);
        }
        for i in 0..cap {
            assert!(!m.contains_key(i));
        }
    }
    let d = start.elapsed();
    println!("Total time: {}", d.as_millis());
}

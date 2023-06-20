//! This file contains source code that is licensed under the MIT License. A copy of this license
//! is provided below:
//!
//! MIT License
//!
//! Copyright (c) 2019 The Algorithms
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use std::hash::Hash;
use std::ops::Mul;
use std::{collections::HashMap, fmt::Debug};

use num_traits::One;

// https://github.com/TheAlgorithms/Rust/blob/master/src/graph/floyd_warshall.rs
pub type Graph<V, E> = HashMap<V, HashMap<V, E>>;

pub fn floyd_warshall_multiplicative<
    V: Eq + Hash + Copy + Debug,
    E: PartialOrd + Copy + Mul<Output = E> + One + Debug,
>(
    graph: &Graph<V, E>,
) -> HashMap<V, HashMap<V, E>> {
    let mut map: HashMap<V, HashMap<V, E>> = HashMap::new();
    for (u, edges) in graph.iter() {
        if !map.contains_key(u) {
            map.insert(*u, HashMap::new());
        }
        map.entry(*u).or_default().insert(*u, One::one());
        for (v, weight) in edges.iter() {
            if !map.contains_key(v) {
                map.insert(*v, HashMap::new());
            }
            map.entry(*v).or_default().insert(*v, One::one());
            map.entry(*u).and_modify(|mp| {
                mp.insert(*v, *weight);
            });
        }
    }
    let keys = map.keys().copied().collect::<Vec<_>>();
    for &k in &keys {
        for &i in &keys {
            if map[&i].get(&k).is_none() {
                continue;
            }
            for &j in &keys {
                if !map[&k].contains_key(&j) {
                    continue;
                }
                let entry_i_j = map[&i].get(&j);
                let entry_i_k = map[&i][&k];
                let entry_k_j = map[&k][&j];
                match entry_i_j {
                    Some(&e) => {
                        if e > entry_i_k * entry_k_j {
                            map.entry(i).or_default().insert(j, entry_i_k * entry_k_j);
                        }
                    }
                    None => {
                        map.entry(i).or_default().insert(j, entry_i_k * entry_k_j);
                    }
                };
            }
        }
    }
    map
}

pub fn add_edge<V: Eq + Hash + Copy, E: PartialOrd + Copy>(
    graph: &mut Graph<V, E>,
    v1: V,
    v2: V,
    c: E,
) {
    graph.entry(v1).or_insert_with(HashMap::new).insert(v2, c);
}

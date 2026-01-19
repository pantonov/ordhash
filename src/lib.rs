//! Ordered hash map with queue-like semantics.
//!
//! `OrdHash` combines a `HashMap` for fast key lookups with a `VecDeque` that
//! records insertion or refresh order. Each update advances a generation counter
//! so stale order entries can be skipped without scanning or reordering the map.
//!
//! Key behaviors:
//! - `push_back()` inserts or updates a key and moves it to the back of the order.
//! - `pop_front()` and `peek_front()` operate on the oldest *live* entry.
//! - `mark_unused()` temporarily disables a key without removing its stored value.
//! - `refresh()` re-enables a key and moves it to the back.
//!
//! The `len()` count reflects live entries. `used_entries()` reports the raw
//! number of order entries (including stale ones), which can be higher when
//! keys are refreshed or overwritten.
//!
//! # Examples
//! ```
//! use ordhash::OrdHash;
//!
//! let mut q = OrdHash::new();
//! q.push_back("a", 1);
//! q.push_back("b", 2);
//! assert_eq!(q.peek_front(), Some((&"a", &1)));
//! assert_eq!(q.pop_front(), Some(("a", 1)));
//! assert_eq!(q.get(&"b"), Some(&2));
//! ```
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

// use generational counter to check whether entry in deque correspods to entry in hashmap.
// generation value 0 is used to mark unused entries.
struct GenHolder<V> {
    generation: usize, 
    value: V,
}

/// Ordered hash map with queue-like ordering semantics.
///
/// This structure preserves a logical order of entries while providing
/// $O(1)$ average-time lookups by key.
pub struct OrdHash<K, V> {
    map: HashMap<K, GenHolder<V>>,
    order: VecDeque<GenHolder<K>>,
    generation: usize,
    length: usize,
}

impl<K: Eq + Hash + Clone, V> OrdHash<K, V> {
    /// Creates a new empty `OrdHash`.
    ///
    /// The map and order queue start with zero capacity.
    pub fn new() -> Self {
        OrdHash {
            map: HashMap::new(),
            order: VecDeque::new(),
            generation: 0,
            length: 0,
        }
    }
    /// Creates a new empty `OrdHash` with the specified capacity.
    pub fn with_capacity(cap: usize) -> Self {
        OrdHash {
            map: HashMap::with_capacity(cap),
            order: VecDeque::with_capacity(cap),
            generation: 0,
            length: 0,
        }
    }
    /// Reserves capacity for at least `additional` more entries.
    pub fn reserve(&mut self, additional: usize) {
        self.map.reserve(additional);
        self.order.reserve(additional);
    }
    /// Inserts a key-value pair and moves the key to the back of the order.
    ///
    /// If the key already exists, the value is replaced and the key is
    /// refreshed to the back. The live length only increases when a key
    /// is inserted for the first time or re-enabled via `refresh()`.
    pub fn push_back(&mut self, key: K, value: V) {
        let cloned_key = key.clone();
        self.generation += 1;
        if self.map.insert(key, GenHolder { value, generation: self.generation }).is_none() {
            self.length += 1;
        }
        self.order.push_back(GenHolder{ value: cloned_key, generation: self.generation });
    }
    /// Returns a reference to the value for `key`, if it is live.
    ///
    /// Keys marked unused via `mark_unused()` are treated as missing.
    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(v) = self.map.get(key) && 0 != v.generation {
            return Some(&v.value)
        }
        None
    }
    /// Removes and returns the oldest live entry, if any.
    ///
    /// Stale or unused entries are skipped automatically.
    pub fn pop_front(&mut self) -> Option<(K, V)> {
        while let Some(key) = self.order.pop_front() {
            if let Some(vh) = self.map.get(&key.value) && vh.generation == key.generation {
                let vh = self.map.remove(&key.value).unwrap(); // can't fail
                self.length -= 1;
                return Some((key.value, vh.value));
            }
        }
        None
    }
    /// Returns the oldest live entry without removing it.
    ///
    /// Stale or unused entries are skipped automatically.
    pub fn peek_front(&self) -> Option<(&K, &V)> {
        for key in &self.order {
            if let Some(vh) = self.map.get(&key.value) && vh.generation == key.generation {
                return Some((&key.value, &vh.value));
            }
        }
        None
    }
    /// Returns `true` if there are no live entries.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the number of live entries.
    pub fn len(&self) -> usize {
        self.length
    }
    /// Returns the number of order entries, including stale or unused ones.
    ///
    /// This can be larger than `len()` when keys are overwritten or refreshed.
    pub fn used_entries(&self) -> usize {
        self.order.len()
    }
    /// Marks `key` as unused and returns a reference to its value.
    ///
    /// After this call, `get()` returns `None` for `key`, and the entry is
    /// ignored by `peek_front()` and dropped by `pop_front()`.
    pub fn mark_unused(&mut self, key: &K) -> Option<&V> {
        if let Some(vh) = self.map.get_mut(key) && vh.generation != 0 {
            vh.generation = 0;
            self.length -= 1;
            return Some(&vh.value)
        }
        None
    }
    /// Refreshes an entry by moving it to the back of the order.
    ///
    /// If the entry was previously marked unused, it becomes live again and
    /// contributes to `len()`. Returns a reference to the value, if it exists.
    pub fn refresh(&mut self, key: &K) -> Option<&V> {
        if let Some(gh) = self.map.get_mut(key) {
            if 0 == gh.generation {
                self.length += 1;
            }
            self.generation += 1;
            gh.generation = self.generation;
            self.order.push_back(GenHolder{ value: key.clone(), generation: self.generation });
            return Some(&gh.value);
        }
        None
    }
    
}

impl<K: Eq + Hash + Clone, V> Default for OrdHash<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

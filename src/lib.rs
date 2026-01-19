use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

struct ValueHolder<V> {
    cnt: usize, // counter is used to track how many duplicate keys are in the order queue
    value: V,
}

pub struct OrdHash<K, V> {
    map: HashMap<K, ValueHolder<V>>,
    order: VecDeque<K>,
}

impl<K: Eq + Hash + Clone, V> OrdHash<K, V> {
    // Create a new empty OrdHash
    pub fn new() -> Self {
        OrdHash {
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }
    // Create a new OrdHash with capacity
    pub fn with_capacity(cap: usize) -> Self {
        OrdHash {
            map: HashMap::with_capacity(cap),
            order: VecDeque::with_capacity(cap),
        }
    }
    // reserve space for 'additional' entries
    pub fn reserve(&mut self, additional: usize) {
        self.map.reserve(additional);
        self.order.reserve(additional);
    }
    // Push key and value pair to the back of order. If key already exists, re-set value and put key at back of order.
    pub fn push_back(&mut self, key: K, value: V) {
        let cloned_key = key.clone();
        self.map
            .entry(key)
            .and_modify(|ev| ev.cnt += 1)
            .or_insert(ValueHolder { value, cnt: 0 });
        self.order.push_back(cloned_key);
    }
    // Get reference to value by key
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key).map(|v| &v.value)
    }
    // Fetch and remove the front entry in order
    pub fn pop_front(&mut self) -> Option<(K, V)> {
        while let Some(key) = self.order.pop_front() {
            if let Some(vh) = self.map.remove(&key) {
                if vh.cnt == 0 {
                    return Some((key, vh.value));
                } else {
                    self.map.entry(key).and_modify(|e| e.cnt -= 1);
                }
            }
        }
        None
    }
    // Peek at the front entry without removing it
    pub fn peek_front(&self) -> Option<(&K, &V)> {
        for key in &self.order {
            if let Some(vh) = self.map.get(key) && vh.cnt == 0 {
                return Some((key, &vh.value));
            }
        }
        None
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|e| e.value)
    }
}

impl<K: Eq + Hash + Clone, V> Default for OrdHash<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

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

pub struct OrdHash<K, V> {
    map: HashMap<K, GenHolder<V>>,
    order: VecDeque<GenHolder<K>>,
    generation: usize,
    length: usize,
}

impl<K: Eq + Hash + Clone, V> OrdHash<K, V> {
    // Create a new empty OrdHash
    pub fn new() -> Self {
        OrdHash {
            map: HashMap::new(),
            order: VecDeque::new(),
            generation: 0,
            length: 0,
        }
    }
    // Create a new OrdHash with capacity
    pub fn with_capacity(cap: usize) -> Self {
        OrdHash {
            map: HashMap::with_capacity(cap),
            order: VecDeque::with_capacity(cap),
            generation: 0,
            length: 0,
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
        self.generation += 1;
        if self.map.insert(key, GenHolder { value, generation: self.generation }).is_none() {
            self.length += 1;
        }
        self.order.push_back(GenHolder{ value: cloned_key, generation: self.generation });
    }
    // Get reference to stored value by key
    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(v) = self.map.get(key) && 0 != v.generation {
            return Some(&v.value)
        }
        None
    }
    // Pop the front entry in order
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
    // Peek at the front entry without removing it
    pub fn peek_front(&self) -> Option<(&K, &V)> {
        for key in &self.order {
            if let Some(vh) = self.map.get(&key.value) && vh.generation == key.generation {
                return Some((&key.value, &vh.value));
            }
        }
        None
    }
    // Check if queue is empty (has no entries)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    // returns length of the queue
    pub fn len(&self) -> usize {
        self.length
    }
    // return number of used entries. This is not the same as queue lengh, as some of these entries
    // may be marked as 'unused'.
    pub fn used_entries(&self) -> usize {
        self.order.len()
    }
    // Mark entry with given key as unused, so get() for the given key will return None,
    // it will be ignored by peek_front() and silently dropped by pop_front().
    pub fn mark_unused(&mut self, key: &K) -> Option<&V> {
        if let Some(vh) = self.map.get_mut(key) && vh.generation != 0 {
            vh.generation = 0;
            self.length -= 1;
            return Some(&vh.value)
        }
        None
    }
    // Refresh entry (put to the end of the queue); both key and value are unchanged.
    // Returns reference to the value, if it exists.
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

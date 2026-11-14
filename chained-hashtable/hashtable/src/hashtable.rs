use std::{
    hash::{DefaultHasher, Hash, Hasher},
    ops::Index,
};

use linked_list::linked_list::List;

pub struct HashTable<K, V> {
    table: Vec<List<(K, V)>>,
    size: usize,
}

impl<K: Hash + Eq, V> HashTable<K, V> {
    fn new() -> Self {
        let size = 16;
        let table = (0..size).map(|_| List::default()).collect();

        Self { table, size }
    }

    pub fn add(&mut self, key: K, value: V) {
        let index = self.clac_index(&key);
        let list = &mut self.table[index as usize];

        for (k, v) in list.iter_mut() {
            if *k == key {
                *v = value;
                return;
            }
        }
        list.push((key, value));
    }

    pub fn search(&self, key: &K) -> Option<&V> {
        let index = self.clac_index(key);
        let list = &self.table[index as usize];

        for (k, v) in list.iter() {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn re_search(&self, key: &K) -> Option<&V> {
        let index = self.clac_index(key);
        let list = &self.table[index as usize];
        let mut iter = list.iter();

        loop {
            match iter.next() {
                Some((k, v)) if k == key => return Some(v),
                Some(&(_, _)) => continue,
                None => return None,
            }
        }
    }

    fn clac_index(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() % (self.size as u64)
    }
}

impl<K: Hash + Eq, V> Default for HashTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

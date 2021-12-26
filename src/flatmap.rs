use std::fmt::Debug;
use std::slice::Iter;
use std::slice::IterMut;

pub struct FlatMap<K: Clone + PartialEq, V: Clone> {
    data: Vec<(K, V)>,
}

impl<K: Clone + PartialEq, V: Clone> FlatMap<K, V> {
    pub fn new() -> Self {
        let data = Vec::new();
        Self { data }
    }

    fn index(&self, key: &K) -> Option<usize> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|v| if v.1 .0 == *key { Some(v.0) } else { None })
            .next()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.index(key).map(|idx| &self.data[idx].1)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.index(key).map(|idx| &mut self.data[idx].1)
    }

    pub fn set(&mut self, key: &K, value: V) {
        match self.index(key) {
            None => self.data.push((key.clone(), value)),
            Some(idx) => self.data[idx].1 = value,
        }
    }

    pub fn at(&mut self, key: &K, default: &V) -> &mut V {
        let idx = self.index(key);
        let idx = idx.unwrap_or_else(|| {
            self.data.push((key.clone(), default.clone()));
            self.data.len() - 1
        });
        &mut self.data[idx].1
    }

    pub fn iter(&self) -> Iter<'_, (K, V)> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, (K, V)> {
        self.data.iter_mut()
    }

    pub fn data(&self) -> &Vec<(K, V)> {
        &self.data
    }

    pub fn take_data(self) -> Vec<(K, V)> {
        self.data
    }
}

impl<K: Clone + PartialEq + Debug, V: Clone + Debug> Debug for FlatMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.data).finish()
    }
}

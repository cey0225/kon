#![allow(dead_code)]

//! Component storage using SparseSet.

use std::any::Any;
use std::fmt::Debug;

/// Fast component storage with O(1) operations
pub struct SparseSet<T> {
    sparse: Vec<Option<usize>>,
    dense: Vec<T>,
    entities: Vec<u32>,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity_id: u32, value: T) {
        let id = entity_id as usize;

        if id >= self.sparse.len() {
            self.sparse.resize(id + 1, None);
        }

        if let Some(dense_idx) = self.sparse[id] {
            self.dense[dense_idx] = value;
        } else {
            let dense_idx = self.dense.len();
            self.sparse[id] = Some(dense_idx);
            self.dense.push(value);
            self.entities.push(entity_id);
        }
    }

    pub fn get(&self, entity_id: u32) -> Option<&T> {
        let id = entity_id as usize;
        let idx = (*self.sparse.get(id)?)?;
        Some(&self.dense[idx])
    }

    pub fn get_mut(&mut self, entity_id: u32) -> Option<&mut T> {
        let id = entity_id as usize;
        let idx = (*self.sparse.get(id)?)?;
        Some(&mut self.dense[idx])
    }

    pub fn remove(&mut self, entity_id: u32) -> Option<T> {
        let id = entity_id as usize;
        let dense_idx = self.sparse.get_mut(id)?.take()?;

        if self.dense.is_empty() {
            return None;
        }

        let last_entity = *self.entities.last()?;

        if dense_idx < self.dense.len() - 1 {
            self.entities[dense_idx] = last_entity;
            self.sparse[last_entity as usize] = Some(dense_idx);
        }

        self.entities.pop();
        Some(self.dense.swap_remove(dense_idx))
    }

    pub fn contains(&self, entity_id: u32) -> bool {
        let id = entity_id as usize;
        self.sparse.get(id).is_some_and(|s| s.is_some())
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &T)> {
        self.entities.iter().copied().zip(self.dense.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut T)> {
        self.entities.iter().copied().zip(self.dense.iter_mut())
    }

    pub fn entities(&self) -> &[u32] {
        &self.entities
    }

    pub fn len(&self) -> usize {
        self.dense.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Type-erased storage trait
pub trait Storage: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove(&mut self, entity_id: u32) -> bool;
    fn contains(&self, entity_id: u32) -> bool;
    fn entity_ids(&self) -> Vec<u32>;
    fn type_name(&self) -> &'static str;
    fn debug_entry(&self, entity_id: u32) -> Option<String>;
}

impl<T: Any + Send + Sync + Debug + 'static> Storage for SparseSet<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove(&mut self, entity_id: u32) -> bool {
        SparseSet::remove(self, entity_id).is_some()
    }

    fn contains(&self, entity_id: u32) -> bool {
        SparseSet::contains(self, entity_id)
    }

    fn entity_ids(&self) -> Vec<u32> {
        self.entities().to_vec()
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn debug_entry(&self, entity_id: u32) -> Option<String> {
        self.get(entity_id).map(|v| format!("{:?}", v))
    }
}

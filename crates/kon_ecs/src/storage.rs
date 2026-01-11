//! Component storage using SparseSet data structure
//!
//! SparseSet provides O(1) insert, remove, and lookup with dense memory layout
//! for cache-efficient iteration.

use std::any::Any;
use crate::Component;

/// Fast component storage with O(1) operations
///
/// Uses a SparseSet data structure
/// - `sparse`: Maps entity ID -> dense array index
/// - `dense`: Contiguous component data
/// - `entities`: Entity IDs in dense array order
///
/// Benefits:
/// - O(1) insert, remove, get
/// - Cache-friendly iteration (dense array)
/// - Stable pointers within a frame
pub struct SparseSet<T> {
    sparse: Vec<usize>,
    dense: Vec<T>,
    entities: Vec<u32>,
}

/// Sentinel value indicating "no entry" in sparse array
const NONE: usize = usize::MAX;

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            entities: Vec::new(),
        }
    }

    /// Inserts or updates a component for an entity
    ///
    /// If the entity already has this component, it will be replaced.
    pub fn insert(&mut self, entity_id: u32, value: T) {
        let id = entity_id as usize;

        if id >= self.sparse.len() {
            self.sparse.resize(id + 1, NONE);
        }

        if self.sparse[id] != NONE {
            let dense_idx = self.sparse[id];
            self.dense[dense_idx] = value;
        } else {
            let dense_idx = self.dense.len();
            self.sparse[id] = dense_idx;
            self.dense.push(value);
            self.entities.push(entity_id);
        }
    }

    #[inline(always)]
    pub fn get(&self, entity_id: u32) -> Option<&T> {
        let id = entity_id as usize;
        if id >= self.sparse.len() {
            return None;
        }

        let dense_idx = self.sparse[id];
        if dense_idx == NONE {
            return None;
        }

        Some(&self.dense[dense_idx])
    }

    #[inline(always)]
    pub fn get_mut(&mut self, entity_id: u32) -> Option<&mut T> {
        let id = entity_id as usize;
        if id >= self.sparse.len() {
            return None;
        }

        let dense_idx = self.sparse[id];
        if dense_idx == NONE {
            return None;
        }

        Some(&mut self.dense[dense_idx])
    }

    /// Removes a component and returns it
    ///
    /// Uses swap-remove for O(1) deletion. The last element is moved
    /// to fill the gap, so iteration order is not preserved.
    pub fn remove(&mut self, entity_id: u32) -> Option<T> {
        let id = entity_id as usize;
        if id >= self.sparse.len() {
            return None;
        }

        let dense_idx = self.sparse[id];
        if dense_idx == NONE {
            return None;
        }
        self.sparse[id] = NONE;

        if self.dense.is_empty() {
            return None;
        }

        let last_entity = *self.entities.last()?;

        if dense_idx < self.dense.len() - 1 {
            self.entities[dense_idx] = last_entity;
            self.sparse[last_entity as usize] = dense_idx;
        }

        self.entities.pop();
        Some(self.dense.swap_remove(dense_idx))
    }

    pub fn contains(&self, entity_id: u32) -> bool {
        let id = entity_id as usize;
        if id >= self.sparse.len() {
            return false;
        }

        let dense_idx = self.sparse[id];
        dense_idx != NONE
    }

    /// Iterates over all (entity_id, component) pairs
    pub fn iter(&self) -> impl Iterator<Item = (u32, &T)> {
        self.entities.iter().copied().zip(self.dense.iter())
    }

    /// Iterates over all (entity_id, component) pairs mutably
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut T)> {
        self.entities.iter().copied().zip(self.dense.iter_mut())
    }

    /// Returns a slice of all entity IDs in this storage
    ///
    /// Order matches the dense component array.
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

/// Type-erased trait for generic storage access
///
/// Allows World to store different component types in a HashMap
/// without knowing their concrete types at compile time.
pub trait Storage: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove(&mut self, entity_id: u32) -> bool;
    fn contains(&self, entity_id: u32) -> bool;
    fn entity_ids(&self) -> &[u32];

    /// Returns type name (debug builds only)
    #[cfg(debug_assertions)]
    fn type_name(&self) -> &'static str;

    /// Returns formatted debug string for a component (debug builds only)
    #[cfg(debug_assertions)]
    fn debug_entry(&self, entity_id: u32) -> Option<String>;

    /// Prints memory layout of stored components (debug builds only)
    ///
    /// Shows physical memory addresses and offsets to verify contiguity.
    #[cfg(debug_assertions)]
    fn dump_memory_layout(&self);
}

impl<T: Component> Storage for SparseSet<T> {
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

    fn entity_ids(&self) -> &[u32] {
        self.entities()
    }

    #[cfg(debug_assertions)]
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    #[cfg(debug_assertions)]
    fn debug_entry(&self, entity_id: u32) -> Option<String> {
        self.get(entity_id).map(|v| format!("{:?}", v))
    }

    /// Prints a formatted table of memory addresses for stored components
    ///
    /// Used for verifying cache-friendly memory layout and debugging
    /// performance issues. Only available in debug builds.
    #[cfg(debug_assertions)]
    fn dump_memory_layout(&self) {
        let type_name = std::any::type_name::<T>()
            .split("::")
            .last()
            .unwrap_or("Unknown");
        let size = std::mem::size_of::<T>();

        println!(
            "\n● STORAGE: {} ({} bytes per element)",
            type_name.to_uppercase(),
            size
        );
        println!("┌─────────┬───────────┬────────────────────┬───────────────┐");
        println!("│  Index  │ Entity ID │   Memory Address   │    Offset     │");
        println!("├─────────┼───────────┼────────────────────┼───────────────┤");

        let mut prev_addr = 0usize;

        for (i, component) in self.dense.iter().enumerate() {
            let addr = component as *const T as usize;
            let offset = if i > 0 { addr - prev_addr } else { 0 };

            println!(
                "│ {:^7} │ {:^9} │ 0x{:012x}     │ {:>10} B  │",
                i, self.entities[i], addr, offset
            );

            prev_addr = addr;
        }
        println!("└─────────┴───────────┴────────────────────┴───────────────┘");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut set = SparseSet::new();
        set.insert(1, "a");
        assert_eq!(set.get(1), Some(&"a"));
    }

    #[test]
    fn get_nonexistent() {
        let set = SparseSet::<i32>::new();
        assert_eq!(set.get(1), None);
    }

    #[test]
    fn remove_existing() {
        let mut set = SparseSet::new();
        set.insert(1, "a");
        assert_eq!(set.remove(1), Some("a"));
        assert_eq!(set.get(1), None);
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn remove_nonexistent() {
        let mut set = SparseSet::<i32>::new();
        assert_eq!(set.remove(1), None);
    }

    #[test]
    fn insert_overwrites() {
        let mut set = SparseSet::new();
        set.insert(1, "a");
        set.insert(1, "b");
        assert_eq!(set.get(1), Some(&"b"));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn remove_middle_swaps_last() {
        let mut set = SparseSet::new();
        set.insert(1, "a");
        set.insert(2, "b");
        set.insert(3, "c");

        set.remove(2);

        assert_eq!(set.len(), 2);
        assert_eq!(set.get(1), Some(&"a"));
        assert_eq!(set.get(3), Some(&"c"));
        assert_eq!(set.get(2), None);
    }

    #[test]
    fn contains_check() {
        let mut set = SparseSet::new();
        set.insert(1, "a");
        assert!(set.contains(1));
        assert!(!set.contains(2));
    }

    #[test]
    fn iter_all_entries() {
        let mut set = SparseSet::new();
        set.insert(1, "a");
        set.insert(2, "b");
        set.insert(3, "c");

        let items: Vec<_> = set.iter().collect();
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn iter_mut_modifies() {
        let mut set = SparseSet::new();
        set.insert(1, 50);
        set.insert(2, 100);

        for (_, value) in set.iter_mut() {
            *value *= 3;
        }

        assert_eq!(set.get(1), Some(&150));
        assert_eq!(set.get(2), Some(&300));
    }

    #[test]
    fn empty_set() {
        let set = SparseSet::<i32>::new();
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }
}

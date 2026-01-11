//! ECS World - entity and component management
//!
//! The world is the core ECS container that stores:
//! - Entities with generational indices
//! - Component storage (SparseSet per type)
//! - Tag system (128 bitmask-based labels per entity)
//! - Deferred operations queue

use crate::Component;
use crate::entity::{Entity, EntityBuilder};
use crate::query::{Query, QueryMut, QueryTuple, QueryTupleMut};
use crate::storage::{SparseSet, Storage};
use std::any::TypeId;
use std::collections::{HashMap, HashSet};

/// Boxed closure for deferred World operations
///
/// Used to queue operations that modify the World during queries.
/// Applied via `world.apply_deferred()` at safe points.
type DeferredOp = Box<dyn FnOnce(&mut World) + Send + Sync>;

/// ECS World containing all entities, components and tags
///
/// # Tag System
/// Tags are lightweight string labels stored as bitmasks (u128).
/// - Up to 128 unique tags globally
/// - O(1) tag filtering in queries
/// - Tags are not components (no storage overhead per entity)
///
/// # Deferred Operations
/// Use `world.defer()` to queue operations that modify the World during iteration.
/// Applied via `apply_deferred()` at frame end.
///
/// # Example
/// ```ignore
/// let mut world = World::new();
///
/// // Create entity with components and tags
/// let player = world.spawn()
///     .insert(Position { x: 0.0, y: 0.0 })
///     .insert(Velocity { x: 1.0, y: 0.0 })
///     .tag("player")
///     .tag("friendly")
///     .id();
///
/// // Immutable query
/// world.select::<(Position, Velocity)>()
///     .tagged("player")
///     .each(|entity, (pos, vel)| {
///         println!("{}: {:?}", entity, pos);
///     });
///
/// // Mutable query
/// world.select_mut::<(Position, Velocity)>()
///     .not_tagged("frozen")
///     .each(|entity, (pos, vel)| {
///         pos.x += vel.x;
///         pos.y += vel.y;
///     });
/// ```
pub struct World {
    next_id: u32,
    generations: Vec<u32>,
    alive: HashSet<u32>,
    free_ids: Vec<u32>,
    components: HashMap<TypeId, Box<dyn Storage>>,

    /// Mapping of tag names to their respective bit indices (0-127).
    tag_registry: HashMap<String, usize>,

    /// List of tag names indexed by their bit position. Used for debugging and inspection.
    tag_names: Vec<String>,

    /// Bitmask for each entity storing active tags. Indexed by Entity ID.
    entity_tags: Vec<u128>,

    deferred: Vec<DeferredOp>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    /// Creates a new empty World
    pub fn new() -> Self {
        Self {
            next_id: 0,
            generations: Vec::new(),
            alive: HashSet::new(),
            free_ids: Vec::new(),
            components: HashMap::new(),
            tag_registry: HashMap::new(),
            tag_names: Vec::new(),
            entity_tags: Vec::new(),
            deferred: Vec::new(),
        }
    }

    /// Starts building a new entity
    ///
    /// Returns an EntityBuilder for fluent API chaining.
    /// Entity IDs are reused from a free list after destruction.
    pub fn spawn(&mut self) -> EntityBuilder<'_> {
        let id = self.free_ids.pop().unwrap_or_else(|| {
            let id = self.next_id;
            self.next_id += 1;
            id
        });

        if id as usize >= self.generations.len() {
            self.generations.resize(id as usize + 1, 0);
        }

        let generation = self.generations[id as usize];
        let entity = Entity::new(id, generation);
        self.alive.insert(id);

        EntityBuilder::new(self, entity)
    }

    /// Checks if an entity is alive and matches the given generation
    ///
    /// Returns false if:
    /// - Entity ID not in alive set
    /// - Generation mismatch (entity was destroyed and ID reused)
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.alive.contains(&entity.id())
            && self.generations.get(entity.id() as usize) == Some(&entity.generation())
    }

    /// Returns internal component storage map (used by query system)
    pub(crate) fn components(&self) -> &HashMap<TypeId, Box<dyn Storage>> {
        &self.components
    }

    /// Returns mutable internal component storage map (used by query system)
    pub(crate) fn components_mut(&mut self) -> &mut HashMap<TypeId, Box<dyn Storage>> {
        &mut self.components
    }

    /// Checks if entity has a component by TypeId (used by query filters)
    pub(crate) fn has_by_type_id(&self, entity: Entity, type_id: &TypeId) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        self.components
            .get(type_id)
            .is_some_and(|s| s.contains(entity.id()))
    }

    /// Returns the current generation for an entity ID (used by query system)
    pub(crate) fn generation(&self, id: u32) -> u32 {
        self.generations.get(id as usize).copied().unwrap_or(0)
    }

    /// Destroys an entity and removes all its components and tags
    ///
    /// The entity ID is added to a free list for reuse.
    /// The generation counter is increased to invalidate old references.
    ///
    /// Returns true if the entity was alive, false otherwise.
    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        let id = entity.id();

        for storage in self.components.values_mut() {
            storage.remove(id);
        }

        if (id as usize) < self.entity_tags.len() {
            self.entity_tags[id as usize] = 0;
        }

        self.alive.remove(&id);
        self.generations[id as usize] = self.generations[id as usize].wrapping_add(1);
        self.free_ids.push(id);

        true
    }

    /// Inserts a component into an entity
    ///
    /// If the entity already has this component type, it will be replaced.
    /// Does nothing if the entity is not alive.
    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) {
        if !self.is_alive(entity) {
            return;
        }

        let storage = self
            .components
            .entry(TypeId::of::<C>())
            .or_insert_with(|| Box::new(SparseSet::<C>::new()));

        if let Some(set) = storage.as_any_mut().downcast_mut::<SparseSet<C>>() {
            set.insert(entity.id(), component);
        }
    }

    /// Removes a component from an entity
    ///
    /// Returns true if the component was removed, false if not found.
    pub fn remove<C: Component>(&mut self, entity: Entity) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        self.components
            .get_mut(&TypeId::of::<C>())
            .and_then(|s| s.as_any_mut().downcast_mut::<SparseSet<C>>())
            .is_some_and(|s| s.remove(entity.id()).is_some())
    }

    /// Gets an immutable reference to a component
    ///
    /// Returns None if:
    /// - Entity is not alive
    /// - Entity doesn't have this component type
    pub fn get<C: Component>(&self, entity: Entity) -> Option<&C> {
        if !self.is_alive(entity) {
            return None;
        }

        self.components
            .get(&TypeId::of::<C>())
            .and_then(|s| s.as_any().downcast_ref::<SparseSet<C>>())
            .and_then(|s| s.get(entity.id()))
    }

    /// Gets a mutable reference to a component
    ///
    /// Returns None if:
    /// - Entity is not alive
    /// - Entity doesn't have this component type
    pub fn get_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        if !self.is_alive(entity) {
            return None;
        }

        self.components
            .get_mut(&TypeId::of::<C>())
            .and_then(|s| s.as_any_mut().downcast_mut::<SparseSet<C>>())
            .and_then(|s| s.get_mut(entity.id()))
    }

    /// Checks if an entity has a component of the given type
    pub fn has<C: Component>(&self, entity: Entity) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        self.components
            .get(&TypeId::of::<C>())
            .is_some_and(|s| s.contains(entity.id()))
    }

    /// Maps a tag name to its bit index (0-127) in the bitmask
    ///
    /// Creates a new index if the tag hasn't been registered yet.
    ///
    /// # Panics
    /// Panics if more than 128 unique tags are registered globally.
    #[track_caller]
    fn get_or_create_tag_id(&mut self, tag: &str) -> usize {
        if let Some(&id) = self.tag_registry.get(tag) {
            id
        } else {
            let id = self.tag_registry.len();
            assert!(id < 128, "ECS only supports up to 128 unique tags");
            self.tag_registry.insert(tag.to_string(), id);
            self.tag_names.push(tag.to_string());
            id
        }
    }

    /// Returns the bit index of a tag if it exists (used by query system)
    pub(crate) fn get_tag_id(&self, tag: &str) -> Option<usize> {
        self.tag_registry.get(tag).copied()
    }

    /// Checks if an entity has a specific tag
    ///
    /// This is O(1) after the tag name lookup.
    #[inline(always)]
    pub fn has_tag(&self, entity: Entity, tag: &str) -> bool {
        if let Some(&tag_id) = self.tag_registry.get(tag) {
            let mask = self.get_tag_mask(entity.id());
            (mask & (1 << tag_id)) != 0
        } else {
            false
        }
    }

    /// Attaches a tag to an entity
    ///
    /// Tags are stored as bitmasks for fast filtering in queries.
    /// Does nothing if the entity is not alive.
    ///
    /// # Panics
    /// Panics if more than 128 unique tags are created globally.
    #[track_caller]
    pub fn tag(&mut self, entity: Entity, tag: &str) {
        if !self.is_alive(entity) {
            return;
        }

        let tag_id = self.get_or_create_tag_id(tag);
        let id = entity.id() as usize;

        if id >= self.entity_tags.len() {
            self.entity_tags.resize(id + 1, 0);
        }

        self.entity_tags[id] |= 1 << tag_id;
    }

    /// Removes a tag from an entity by clearing its bit in the bitmask
    pub fn untag(&mut self, entity: Entity, tag: &str) {
        if let Some(&tag_id) = self.tag_registry.get(tag) {
            let id = entity.id() as usize;
            if id < self.entity_tags.len() {
                self.entity_tags[id] &= !(1 << tag_id);
            }
        }
    }

    /// Returns all tag names associated with an entity
    ///
    /// Primarily used for debugging and inspection.
    pub fn get_entity_tags(&self, entity_id: u32) -> Vec<String> {
        let mut names = Vec::new();

        if let Some(&mask) = self.entity_tags.get(entity_id as usize) {
            for i in 0..self.tag_names.len() {
                if (mask & (1 << i)) != 0 {
                    names.push(self.tag_names[i].clone());
                }
            }
        }

        names
    }

    /// Returns the bitmask of all tags for an entity (used by query system)
    ///
    /// Each bit represents one tag. Used for O(1) tag filtering.
    #[inline(always)]
    pub(crate) fn get_tag_mask(&self, entity_id: u32) -> u128 {
        self.entity_tags
            .get(entity_id as usize)
            .copied()
            .unwrap_or(0)
    }

    /// Starts an immutable query
    ///
    /// # Example
    /// ```ignore
    /// world.select::<(Position, Velocity)>()
    ///     .tagged("player")
    ///     .each(|entity, (pos, vel)| {
    ///         println!("{:?}", pos);
    ///     });
    /// ```
    #[track_caller]
    pub fn select<T: for<'w> QueryTuple<'w>>(&self) -> Query<'_, T> {
        Query::new(self)
    }

    /// Starts a mutable query
    ///
    /// # Example
    /// ```ignore
    /// world.select_mut::<(Position, Velocity)>()
    ///     .not_tagged("frozen")
    ///     .each(|entity, (pos, vel)| {
    ///         pos.x += vel.x;
    ///     });
    /// ```
    #[track_caller]
    pub fn select_mut<T: for<'w> QueryTupleMut<'w>>(&mut self) -> QueryMut<'_, T> {
        QueryMut::new(self)
    }

    /// Queues an operation to be executed later via `apply_deferred()`
    ///
    /// Useful for spawning/destroying entities during query iteration.
    ///
    /// # Example
    /// ```ignore
    /// world.select::<(Health,)>().each(|entity, (health,)| {
    ///     if health.0 <= 0 {
    ///         world.defer(move |w| {
    ///             w.destroy(entity);
    ///         });
    ///     }
    /// });
    /// world.apply_deferred();
    /// ```
    pub fn defer<F: FnOnce(&mut World) + Send + Sync + 'static>(&mut self, f: F) {
        self.deferred.push(Box::new(f));
    }

    /// Executes all queued deferred operations
    ///
    /// Called automatically each frame by `apply_deferred_system`.
    pub fn apply_deferred(&mut self) {
        let deferred = std::mem::take(&mut self.deferred);
        for f in deferred {
            f(self);
        }
    }

    /// Returns the number of alive entities
    pub fn entity_count(&self) -> usize {
        self.alive.len()
    }

    /// No-op in release builds (prints warning)
    ///
    /// Use debug builds to access memory dump functionality.
    #[cfg(not(debug_assertions))]
    pub fn dump_all_memory(&self) {
        log::warn!("Memory dump is disabled in release mode.");
    }

    /// Prints physical memory layout of all component storages (debug only)
    ///
    /// Shows memory addresses and byte offsets to verify:
    /// - Components are stored contiguously
    /// - Cache-friendly memory access patterns
    ///
    /// Only enabled in debug builds to avoid I/O overhead.
    #[cfg(debug_assertions)]
    pub fn dump_all_memory(&self) {
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║                   WORLD MEMORY DUMP                      ║");
        println!("╠══════════════════════════════════════════════════════════╣");
        println!(
            "║ Entities: {:<5}          Component Types: {:<5}          ║",
            self.entity_count(),
            self.components.len()
        );
        println!("╚══════════════════════════════════════════════════════════╝");

        if self.components.is_empty() {
            println!("(No components registered)");
        } else {
            for storage in self.components.values() {
                storage.dump_memory_layout();
            }
        }
    }

    /// No-op in release builds (prints warning)
    ///
    /// Use debug builds to access inspection functionality.
    #[cfg(not(debug_assertions))]
    pub fn inspect(&self) {
        log::warn!("Inspect is disabled in release mode.");
    }

    /// Prints a formatted table of all entities and their components (debug only)
    ///
    /// Shows:
    /// - Entity IDs and generations
    /// - Attached tags
    /// - Component values
    ///
    /// Useful for debugging game state and verifying entity configurations.
    /// Only enabled in debug builds to avoid string formatting overhead.
    #[cfg(debug_assertions)]
    pub fn inspect(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════════════════╗");
        println!("║                            WORLD INSPECTOR                               ║");
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        println!(
            "║  Entities: {:<5}  Component Types: {:<5}                                 ║",
            self.alive.len(),
            self.components.len()
        );
        println!("╚══════════════════════════════════════════════════════════════════════════╝\n");

        if self.alive.is_empty() {
            println!("  (no entities)");
            return;
        }

        // Collect data first to calculate widths
        let mut sorted_ids: Vec<_> = self.alive.iter().copied().collect();
        sorted_ids.sort();

        let type_names: Vec<String> = self
            .components
            .values()
            .map(|s| {
                let full = s.type_name();
                full.rsplit("::").next().unwrap_or(full).to_string()
            })
            .collect();

        // Calculate column widths
        let entity_width = 14;
        let tags_width = 20;

        let mut col_widths: Vec<usize> = type_names.iter().map(|n| n.len()).collect();

        for id in &sorted_ids {
            for (i, storage) in self.components.values().enumerate() {
                let value = storage.debug_entry(*id).unwrap_or("-".to_string());
                if value.len() > col_widths[i] {
                    col_widths[i] = value.len();
                }
            }
        }

        // Add padding
        for w in &mut col_widths {
            *w = (*w).max(10) + 2;
        }

        // Header
        print!("┌{:─<entity_width$}┬{:─<tags_width$}", "", "");
        for w in &col_widths {
            print!("┬{:─<w$}", "");
        }
        println!("┐");

        print!("│{:<entity_width$}│{:<tags_width$}", " Entity", " Tags");
        for (name, w) in type_names.iter().zip(&col_widths) {
            print!("│ {:<width$}", name, width = w - 1);
        }
        println!("│");

        print!("├{:─<entity_width$}┼{:─<tags_width$}", "", "");
        for w in &col_widths {
            print!("┼{:─<w$}", "");
        }
        println!("┤");

        // Rows
        for id in &sorted_ids {
            let generation = self.generations[*id as usize];
            let entity = Entity::new(*id, generation);

            let tags = self.get_entity_tags(*id);
            let tags_str = if tags.is_empty() {
                "-".to_string()
            } else {
                tags.join(", ")
            };

            print!(
                "│ {:<width$}│ {:<tags_w$}",
                format!("{}", entity),
                tags_str,
                width = entity_width - 1,
                tags_w = tags_width - 1
            );

            for (storage, w) in self.components.values().zip(&col_widths) {
                let value = storage.debug_entry(*id).unwrap_or("-".to_string());
                print!("│ {:<width$}", value, width = w - 1);
            }
            println!("│");
        }

        // Footer
        print!("└{:─<entity_width$}┴{:─<tags_width$}", "", "");
        for w in &col_widths {
            print!("┴{:─<w$}", "");
        }
        println!("┘");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Health(i32);

    #[derive(Debug, Clone, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[test]
    fn spawn_entity() {
        let mut world = World::new();
        let entity = world.spawn().id();
        assert!(world.is_alive(entity));
        assert_eq!(world.entity_count(), 1);
    }

    #[test]
    fn destroy_entity() {
        let mut world = World::new();
        let entity = world.spawn().id();
        world.destroy(entity);
        assert!(!world.is_alive(entity));
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn destroy_dead_entity_returns_false() {
        let mut world = World::new();
        let entity = world.spawn().id();
        world.destroy(entity);
        assert!(!world.destroy(entity));
    }

    #[test]
    fn entity_count() {
        let mut world = World::new();
        world.spawn().id();
        world.spawn().id();
        world.spawn().id();
        assert_eq!(world.entity_count(), 3);
    }

    #[test]
    fn entity_reuse_after_destroy() {
        let mut world = World::new();
        let entity = world.spawn().id();
        world.destroy(entity);
        let new_entity = world.spawn().id();
        assert_eq!(entity.id(), new_entity.id());
        assert_ne!(entity.generation(), new_entity.generation());
    }

    #[test]
    fn generation_tracking() {
        let mut world = World::new();
        let entity = world.spawn().id();
        world.destroy(entity);
        let entity2 = world.spawn().id();
        world.destroy(entity2);
        let entity3 = world.spawn().id();

        assert_eq!(entity.generation(), 0);
        assert_eq!(entity2.generation(), 1);
        assert_eq!(entity3.generation(), 2);
    }

    #[test]
    fn insert_and_get_component() {
        let mut world = World::new();
        let entity = world.spawn().id();

        world.insert(entity, Health(100));

        let health = world.get::<Health>(entity).unwrap();
        assert_eq!(health.0, 100);
    }

    #[test]
    fn get_nonexistent_component() {
        let mut world = World::new();
        let entity = world.spawn().id();
        assert_eq!(world.get::<Health>(entity), None);
    }

    #[test]
    fn get_mut_component() {
        let mut world = World::new();
        let entity = world.spawn().id();

        world.insert(entity, Health(100));

        {
            let health = world.get_mut::<Health>(entity).unwrap();
            health.0 -= 30;
        }

        assert_eq!(world.get::<Health>(entity).unwrap().0, 70);
    }

    #[test]
    fn has_component() {
        let mut world = World::new();
        let entity = world.spawn().id();

        world.insert(entity, Health(100));

        assert!(world.has::<Health>(entity));
        assert!(!world.has::<Position>(entity));
    }

    #[test]
    fn remove_component() {
        let mut world = World::new();
        let entity = world.spawn().id();

        world.insert(entity, Health(100));
        world.remove::<Health>(entity);

        assert!(!world.has::<Health>(entity));
        assert_eq!(world.get::<Health>(entity), None);
    }

    #[test]
    fn remove_nonexistent_component() {
        let mut world = World::new();
        let entity = world.spawn().id();
        assert!(!world.remove::<Health>(entity));
    }

    #[test]
    fn insert_overwrites_component() {
        let mut world = World::new();
        let entity = world.spawn().id();
        world.insert(entity, Health(100));
        world.insert(entity, Health(50));
        assert_eq!(world.get::<Health>(entity).unwrap().0, 50);
    }

    #[test]
    fn destroy_clears_all_components() {
        let mut world = World::new();
        let entity = world.spawn().id();
        world.insert(entity, Health(100));
        world.insert(entity, Position { x: 10.0, y: 60.0 });
        world.destroy(entity);
        assert!(!world.has::<Health>(entity));
        assert!(!world.has::<Position>(entity));
    }

    #[test]
    fn tag_and_has_tag() {
        let mut world = World::new();
        let entity = world.spawn().id();
        world.tag(entity, "player");
        assert!(world.has_tag(entity, "player"));
    }

    #[test]
    fn untag_removes_tag() {
        let mut world = World::new();
        let entity = world.spawn().id();
        world.tag(entity, "player");
        world.untag(entity, "player");
        assert!(!world.has_tag(entity, "player"));
    }

    #[test]
    fn multiple_tags_on_entity() {
        let mut world = World::new();
        let entity = world.spawn().id();
        world.tag(entity, "npc");
        world.tag(entity, "friendly");
        world.tag(entity, "tradeable");
        assert!(world.has_tag(entity, "npc"));
        assert!(world.has_tag(entity, "friendly"));
        assert!(world.has_tag(entity, "tradeable"));
    }

    #[test]
    fn destroy_clears_tags() {
        let mut world = World::new();
        let entity = world.spawn().id();
        world.tag(entity, "npc");
        world.tag(entity, "friendly");
        world.tag(entity, "tradeable");
        world.destroy(entity);
        assert_eq!(world.get_tag_mask(entity.id()), 0);
    }

    #[test]
    fn insert_on_dead_entity_ignored() {
        let mut world = World::new();
        let entity = world.spawn().id();

        world.destroy(entity);
        world.insert(entity, Health(100));

        assert!(!world.has::<Health>(entity));
    }

    #[test]
    fn get_from_dead_entity_returns_none() {
        let mut world = World::new();
        let entity = world.spawn().id();

        world.insert(entity, Health(100));
        world.destroy(entity);

        assert_eq!(world.get::<Health>(entity), None);
    }

    #[test]
    fn tag_on_dead_entity_ignored() {
        let mut world = World::new();
        let entity = world.spawn().id();

        world.destroy(entity);
        world.tag(entity, "player");

        assert!(!world.has_tag(entity, "player"));
    }

    #[test]
    fn entity_builder_chain() {
        let mut world = World::new();
        let entity = world
            .spawn()
            .insert(Health(100))
            .insert(Position { x: 5.0, y: 7.0 })
            .tag("player")
            .id();
        assert!(world.has::<Health>(entity));
        assert!(world.has::<Position>(entity));
        assert!(world.has_tag(entity, "player"));
    }

    #[test]
    fn defer_spawns_entity() {
        let mut world = World::new();

        world.defer(|w| {
            w.spawn().id();
        });

        assert_eq!(world.entity_count(), 0);

        world.apply_deferred();

        assert_eq!(world.entity_count(), 1);
    }

    #[test]
    fn multiple_deferred_operations() {
        let mut world = World::new();

        for i in 0..10 {
            world.defer(move |w| {
                w.spawn().insert(Health(i));
            });
        }

        world.apply_deferred();
        assert_eq!(world.entity_count(), 10);
    }

    #[test]
    fn deferred_cleared_after_apply() {
        let mut world = World::new();
        world.defer(|w| {
            w.spawn().id();
        });
        world.apply_deferred();
        assert_eq!(world.deferred.len(), 0);
    }

    #[test]
    #[should_panic]
    fn world_tag_limit_panic() {
        let mut world = World::new();
        let entity = world.spawn().id();

        for i in 0..129 {
            let tag_name = format!("tag_{}", i);
            world.tag(entity, &tag_name);
        }
    }
}

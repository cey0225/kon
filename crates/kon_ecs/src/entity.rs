//! Entity identifier with generational indices for safe references
//!
//! Entities are lightweight IDs that reference component data in the World.
//! The generation counter prevents use-after-free bugs when entities are destroyed and IDs are reused.

use std::fmt::Debug;
use crate::Component;

/// Unique entity identifier with generation tracking
///
/// Each entity has:
/// - `id`: Numeric identifier (can be reused after destruction)
/// - `generation`: Counter that increases when ID is reused
///
/// This prevents stale references: if you hold an old Entity handle,
/// operations will fail safely because the generation won't match.
///
/// # Example
/// ```ignore
/// let entity = world.spawn()
///     .insert(Position { x: 0.0, y: 0.0 })
///     .id();
///
/// println!("{}", entity); // "Entity(0v0)"
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u32,
    generation: u32,
}

impl Entity {
    pub(crate) fn new(id: u32, generation: u32) -> Self {
        Self { id, generation }
    }

    /// Creates an entity from raw parts (internal use only)
    ///
    /// Used by the query system to reconstruct Entity handles from stored IDs.
    pub(crate) fn from_raw(id: u32, generation: u32) -> Self {
        Self { id, generation }
    }

    /// Returns the entity ID
    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the generation counter
    #[inline]
    pub fn generation(&self) -> u32 {
        self.generation
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity({}v{})", self.id, self.generation)
    }
}

/// Fluent builder for constructing entities with components and tags
///
/// Obtained via `world.spawn()`. Allows method chaining.
///
/// # Example
/// ```ignore
/// let player = world.spawn()
///     .insert(Health(100))
///     .insert(Position { x: 0.0, y: 0.0 })
///     .tag("player")
///     .tag("friendly")
///     .id();
/// ```
pub struct EntityBuilder<'w> {
    world: &'w mut crate::World,
    entity: Entity,
}

impl<'w> EntityBuilder<'w> {
    pub(crate) fn new(world: &'w mut crate::World, entity: Entity) -> Self {
        Self { world, entity }
    }

    /// Attaches a component to the entity
    ///
    /// Components can be any type implementing `Component` (Debug + Send + Sync + 'static)
    pub fn insert<C: Component>(self, component: C) -> Self {
        self.world.insert(self.entity, component);
        self
    }

    /// Attaches a tag to the entity
    ///
    /// Tags are lightweight string labels for filtering queries.
    /// Uses a bitmask system internally for O(1) filtering.
    pub fn tag(self, tag: &str) -> Self {
        self.world.tag(self.entity, tag);
        self
    }

    /// Completes the builder and returns the entity handle
    pub fn id(self) -> Entity {
        self.entity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_creation() {
        let entity = Entity::new(1, 0);
        assert_eq!(entity.id(), 1);
        assert_eq!(entity.generation(), 0);
    }

    #[test]
    fn entity_equality() {
        let e1 = Entity::new(1, 0);
        let e2 = Entity::new(1, 0);
        let e3 = Entity::new(1, 1);

        assert_eq!(e1, e2);
        assert_ne!(e1, e3);
    }
    #[test]
    fn entity_display() {
        let entity = Entity::new(1, 0);
        let display = format!("{}", entity);
        assert_eq!(display, "Entity(1v0)");
    }

    #[test]
    fn generation_increments() {
        let e1 = Entity::new(1, 0);
        let e2 = Entity::new(1, 1);

        assert_eq!(e1.id(), e2.id());
        assert_ne!(e1.generation(), e2.generation());
    }
}

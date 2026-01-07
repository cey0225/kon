//! Entity identifier with generation for safe references.

use std::any::Any;
use std::fmt::Debug;

/// Unique entity identifier
///
/// # Example
/// ```ignore
/// let entity = world.spawn().insert(Position { x: 0.0, y: 0.0 }).id();
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

    /// Create entity from raw parts (for query system)
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

/// Builder for creating entities
pub struct EntityBuilder<'w> {
    world: &'w mut crate::World,
    entity: Entity,
}

impl<'w> EntityBuilder<'w> {
    pub(crate) fn new(world: &'w mut crate::World, entity: Entity) -> Self {
        Self { world, entity }
    }

    /// Inserts a component
    pub fn insert<C: Any + Send + Sync + Debug + 'static>(self, component: C) -> Self {
        self.world.insert(self.entity, component);
        self
    }

    /// Adds a tag
    pub fn tag(self, tag: &str) -> Self {
        self.world.tag(self.entity, tag);
        self
    }

    /// Finishes building and returns the entity
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

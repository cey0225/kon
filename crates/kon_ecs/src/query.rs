//! ECS Query System - Tuple-based fluent API
//!
//! # Example
//! ```ignore
//! // Select entities with Position and Velocity, exclude "frozen" tag
//! world.select::<(Position, Velocity)>()
//!     .tagged("player")
//!     .not_tagged("frozen")
//!     .each(|entity, (pos, vel)| {
//!         println!("{}: {:?}", entity, pos);
//!     });
//!
//! // Mutable query
//! world.select_mut::<(Position, Velocity)>()
//!     .not_tagged("frozen")
//!     .each(|entity, (pos, vel)| {
//!         pos.x += vel.x;
//!     });
//!
//! // Single component works too
//! world.select::<(Health,)>()
//!     .tagged("enemy")
//!     .each(|entity, (hp,)| {
//!         println!("Enemy HP: {}", hp.0);
//!     });
//! ```

use crate::entity::Entity;
use crate::storage::SparseSet;
use crate::World;
use std::any::{Any, TypeId};
use std::marker::PhantomData;

// ============================================================================
// Query Filter
// ============================================================================

/// Filter configuration for tag-based filtering
#[derive(Default, Clone)]
pub struct QueryFilter {
    required_tags: Vec<String>,
    excluded_tags: Vec<String>,
}

impl QueryFilter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if entity passes all tag filters
    pub fn matches(&self, world: &World, entity: Entity) -> bool {
        for tag in &self.required_tags {
            if !world.has_tag(entity, tag) {
                return false;
            }
        }

        for tag in &self.excluded_tags {
            if world.has_tag(entity, tag) {
                return false;
            }
        }

        true
    }
}

// ============================================================================
// Fetch Trait - How to fetch a single component
// ============================================================================

/// Fetch immutable reference
pub trait Fetch<'w> {
    type Output;
    fn fetch(world: &'w World, entity_id: u32) -> Option<Self::Output>;
    fn type_id() -> TypeId;
}

/// Fetch mutable reference
pub trait FetchMut<'w> {
    type Output;
    fn fetch(world: &'w mut World, entity_id: u32) -> Option<Self::Output>;
    fn type_id() -> TypeId;
}

impl<'w, T: Any + Send + Sync + 'static> Fetch<'w> for T {
    type Output = &'w T;

    fn fetch(world: &'w World, entity_id: u32) -> Option<Self::Output> {
        world
            .components()
            .get(&TypeId::of::<T>())
            .and_then(|s| s.as_any().downcast_ref::<SparseSet<T>>())
            .and_then(|s| s.get(entity_id))
    }

    fn type_id() -> TypeId {
        TypeId::of::<T>()
    }
}

impl<'w, T: Any + Send + Sync + 'static> FetchMut<'w> for T {
    type Output = &'w mut T;

    fn fetch(world: &'w mut World, entity_id: u32) -> Option<Self::Output> {
        world
            .components_mut()
            .get_mut(&TypeId::of::<T>())
            .and_then(|s| s.as_any_mut().downcast_mut::<SparseSet<T>>())
            .and_then(|s| s.get_mut(entity_id))
    }

    fn type_id() -> TypeId {
        TypeId::of::<T>()
    }
}

// ============================================================================
// QueryTuple - Trait for tuple of components (immutable)
// ============================================================================

pub trait QueryTuple<'w> {
    type Output;
    fn fetch_all(world: &'w World, entity_id: u32) -> Option<Self::Output>;
    fn first_type_id() -> TypeId;
}

// ============================================================================
// QueryTupleMut - Trait for tuple of components (mutable)
// ============================================================================

pub trait QueryTupleMut<'w> {
    type Output;
    fn fetch_all(world: &'w mut World, entity_id: u32) -> Option<Self::Output>;
    fn first_type_id() -> TypeId;
}

// ============================================================================
// Macro to implement QueryTuple and QueryTupleMut for tuples 1-12
// ============================================================================

macro_rules! impl_query_tuple {
    // Match: first component + rest
    ($first:ident $(, $rest:ident)*) => {
        // Immutable version
        impl<'w, $first: Fetch<'w>, $($rest: Fetch<'w>),*> QueryTuple<'w> for ($first, $($rest),*) {
            type Output = ($first::Output, $($rest::Output),*);

            fn fetch_all(world: &'w World, entity_id: u32) -> Option<Self::Output> {
                Some((
                    $first::fetch(world, entity_id)?,
                    $($rest::fetch(world, entity_id)?),*
                ))
            }

            fn first_type_id() -> TypeId {
                $first::type_id()
            }
        }

        // Mutable version
        impl<'w, $first: FetchMut<'w>, $($rest: FetchMut<'w>),*> QueryTupleMut<'w> for ($first, $($rest),*) {
            type Output = ($first::Output, $($rest::Output),*);

            fn fetch_all(world: &'w mut World, entity_id: u32) -> Option<Self::Output> {
                // SAFETY: We fetch different component types, no aliasing
                let world_ptr = world as *mut World;
                unsafe {
                    Some((
                        $first::fetch(&mut *world_ptr, entity_id)?,
                        $($rest::fetch(&mut *world_ptr, entity_id)?),*
                    ))
                }
            }

            fn first_type_id() -> TypeId {
                $first::type_id()
            }
        }
    };
}

// Generate implementations for 1-12 component tuples
impl_query_tuple!(A);
impl_query_tuple!(A, B);
impl_query_tuple!(A, B, C);
impl_query_tuple!(A, B, C, D);
impl_query_tuple!(A, B, C, D, E);
impl_query_tuple!(A, B, C, D, E, F);
impl_query_tuple!(A, B, C, D, E, F, G);
impl_query_tuple!(A, B, C, D, E, F, G, H);
impl_query_tuple!(A, B, C, D, E, F, G, H, I);
impl_query_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_query_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_query_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

// ============================================================================
// Query - Immutable query builder
// ============================================================================

/// Immutable query builder
///
/// # Example
/// ```ignore
/// world.select::<(Position, Velocity)>()
///     .tagged("player")
///     .not_tagged("frozen")
///     .each(|entity, (pos, vel)| {
///         println!("{:?}", pos);
///     });
/// ```
pub struct Query<'w, T> {
    world: &'w World,
    filter: QueryFilter,
    _marker: PhantomData<T>,
}

impl<'w, T: QueryTuple<'w>> Query<'w, T> {
    pub(crate) fn new(world: &'w World) -> Self {
        Self {
            world,
            filter: QueryFilter::new(),
            _marker: PhantomData,
        }
    }

    /// Require entities to have this tag
    pub fn tagged(mut self, tag: &str) -> Self {
        self.filter.required_tags.push(tag.to_string());
        self
    }

    /// Exclude entities with this tag
    pub fn not_tagged(mut self, tag: &str) -> Self {
        self.filter.excluded_tags.push(tag.to_string());
        self
    }

    /// Iterate over all matching entities
    pub fn each<F>(self, mut f: F)
    where
        F: FnMut(Entity, T::Output),
    {
        let first_type_id = T::first_type_id();

        let entity_ids: Vec<u32> = match self.world.components().get(&first_type_id) {
            Some(storage) => storage.entity_ids(),
            None => return,
        };

        for id in entity_ids {
            let entity = Entity::from_raw(id, self.world.generation(id));

            if !self.filter.matches(self.world, entity) {
                continue;
            }

            if let Some(components) = T::fetch_all(self.world, id) {
                f(entity, components);
            }
        }
    }
}

// ============================================================================
// QueryMut - Mutable query builder
// ============================================================================

/// Mutable query builder
///
/// # Example
/// ```ignore
/// world.select_mut::<(Position, Velocity)>()
///     .not_tagged("frozen")
///     .each(|entity, (pos, vel)| {
///         pos.x += vel.x;
///     });
/// ```
pub struct QueryMut<'w, T> {
    world: &'w mut World,
    filter: QueryFilter,
    _marker: PhantomData<T>,
}

impl<'w, T: QueryTupleMut<'w>> QueryMut<'w, T> {
    pub(crate) fn new(world: &'w mut World) -> Self {
        Self {
            world,
            filter: QueryFilter::new(),
            _marker: PhantomData,
        }
    }

    /// Require entities to have this tag
    pub fn tagged(mut self, tag: &str) -> Self {
        self.filter.required_tags.push(tag.to_string());
        self
    }

    /// Exclude entities with this tag
    pub fn not_tagged(mut self, tag: &str) -> Self {
        self.filter.excluded_tags.push(tag.to_string());
        self
    }

    /// Iterate over all matching entities
    pub fn each<F>(self, mut f: F)
    where
        F: FnMut(Entity, T::Output),
    {
        let first_type_id = T::first_type_id();

        let entity_ids: Vec<u32> = match self.world.components().get(&first_type_id) {
            Some(storage) => storage.entity_ids(),
            None => return,
        };

        for id in entity_ids {
            let entity = Entity::from_raw(id, self.world.generation(id));

            if !self.filter.matches(self.world, entity) {
                continue;
            }

            // SAFETY: We collect entity_ids first, then fetch components
            let world_ptr = self.world as *mut World;
            if let Some(components) = T::fetch_all(unsafe { &mut *world_ptr }, id) {
                f(entity, components);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::World;

    #[derive(Debug, Clone, PartialEq)]
    struct Health(i32);

    #[derive(Debug, Clone, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    #[test]
    fn query_single_component() {
        let mut world = World::new();
        world.spawn().insert(Health(100));
        world.spawn().insert(Health(50));

        let mut entity_count = 0;
        world.select::<(Health,)>().each(|_, _| {
            entity_count += 1;
        });

        assert_eq!(entity_count, 2);
    }

    #[test]
    fn query_multiple_components() {
        let mut world = World::new();
        world.spawn().insert(Health(100));
        world
            .spawn()
            .insert(Health(50))
            .insert(Velocity { x: 5.0, y: 5.0 });

        let mut entity_count = 0;
        world.select::<(Health, Velocity)>().each(|_, _| {
            entity_count += 1;
        });

        assert_eq!(entity_count, 1);
    }

    #[test]
    fn query_three_components() {
        let mut world = World::new();
        world.spawn().insert(Health(100));
        world
            .spawn()
            .insert(Health(50))
            .insert(Velocity { x: 5.0, y: 5.0 });
        world
            .spawn()
            .insert(Health(20))
            .insert(Velocity { x: 10.0, y: 0.0 })
            .insert(Position { x: 100.0, y: 30.0 });

        let mut entity_count = 0;
        world.select::<(Health, Velocity, Position)>().each(|_, _| {
            entity_count += 1;
        });

        assert_eq!(entity_count, 1);
    }

    #[test]
    fn query_empty_world() {
        let world = World::new();

        let mut entity_count = 0;
        world.select::<(Health,)>().each(|_, _| {
            entity_count += 1;
        });

        assert_eq!(entity_count, 0);
    }

    #[test]
    fn query_with_tagged() {
        let mut world = World::new();
        world.spawn().insert(Health(100)).tag("player");
        world.spawn().insert(Health(50));

        let mut entity_count = 0;
        world.select::<(Health,)>().tagged("player").each(|_, _| {
            entity_count += 1;
        });

        assert_eq!(entity_count, 1);
    }

    #[test]
    fn query_with_not_tagged() {
        let mut world = World::new();
        world.spawn().insert(Health(100)).tag("npc");
        world.spawn().insert(Health(50)).tag("npc");
        world.spawn().insert(Health(80));

        let mut entity_count = 0;
        world.select::<(Health,)>().not_tagged("npc").each(|_, _| {
            entity_count += 1;
        });

        assert_eq!(entity_count, 1);
    }

    #[test]
    fn query_multiple_tag_filters() {
        let mut world = World::new();
        world.spawn().insert(Health(100)).tag("npc");
        world.spawn().insert(Health(50)).tag("npc").tag("friendly");
        world.spawn().insert(Health(80)).tag("npc").tag("friendly");

        let mut entity_count = 0;
        world
            .select::<(Health,)>()
            .tagged("npc")
            .not_tagged("friendly")
            .each(|_, _| {
                entity_count += 1;
            });

        assert_eq!(entity_count, 1);
    }

    #[test]
    fn query_no_matching_tags() {
        let mut world = World::new();
        world.spawn().insert(Health(100)).tag("npc");
        world.spawn().insert(Health(50)).tag("friendly");

        let mut entity_count = 0;
        world.select::<(Health,)>().tagged("player").each(|_, _| {
            entity_count += 1;
        });

        assert_eq!(entity_count, 0);
    }

    #[test]
    fn query_mut_modifies_components() {
        let mut world = World::new();
        let entity = world.spawn().insert(Health(100)).id();

        world.select_mut::<(Health,)>().each(|_, components| {
            let health = components.0;
            health.0 -= 20;
        });

        assert_eq!(world.get::<Health>(entity).unwrap().0, 80);
    }

    #[test]
    fn query_mut_multiple_entities() {
        let mut world = World::new();
        let entity = world.spawn().insert(Health(100)).id();
        let entity2 = world.spawn().insert(Health(50)).id();

        world.select_mut::<(Health,)>().each(|_, components| {
            let health = components.0;
            health.0 -= 20;
        });

        assert_eq!(world.get::<Health>(entity).unwrap().0, 80);
        assert_eq!(world.get::<Health>(entity2).unwrap().0, 30);
    }

    #[test]
    fn query_nonexistent_component() {
        let mut world = World::new();
        world.spawn().insert(Health(100));

        let mut entity_count = 0;
        world.select::<(Position,)>().each(|_, _| {
            entity_count += 1;
        });

        assert_eq!(entity_count, 0);
    }

    #[test]
    fn query_partial_match() {
        let mut world = World::new();
        world.spawn().insert(Health(100));

        let mut entity_count = 0;
        world.select::<(Health, Position)>().each(|_, _| {
            entity_count += 1;
        });

        assert_eq!(entity_count, 0);
    }
}

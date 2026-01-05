use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Event trait - all events implement this automatically
pub trait Event: Any + Send + Sync + 'static {}
impl<T: Any + Send + Sync + 'static> Event for T {}

/// Event queue for sending and reading events
///
/// # Example
/// ```ignore
/// // Send
/// ctx.events.send(MyEvent { data: 42 });
///
/// // Read
/// for event in ctx.events.read::<MyEvent>() {
///     println!("{}", event.data);
/// }
/// ```
#[derive(Default)]
pub struct Events {
    queues: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Events {
    /// Creates a new Events queue
    pub fn new() -> Self {
        Self::default()
    }

    /// Sends an event to the queue
    pub fn send<E: Event>(&mut self, event: E) {
        let type_id = TypeId::of::<E>();
        let queue = self
            .queues
            .entry(type_id)
            .or_insert_with(|| Box::new(Vec::<E>::new()));

        if let Some(vec) = queue.downcast_mut::<Vec<E>>() {
            vec.push(event);
        }
    }

    /// Reads all events of a specific type
    pub fn read<E: Event>(&self) -> impl Iterator<Item = &E> {
        let type_id = TypeId::of::<E>();
        self.queues
            .get(&type_id)
            .and_then(|queue| queue.downcast_ref::<Vec<E>>())
            .map(|vec| vec.iter())
            .unwrap_or_else(|| [].iter())
    }

    /// Clears events of a specific type
    pub fn clear<E: Event>(&mut self) {
        let type_id = TypeId::of::<E>();
        if let Some(queue) = self.queues.get_mut(&type_id) {
            if let Some(vec) = queue.downcast_mut::<Vec<E>>() {
                vec.clear();
            }
        }
    }

    /// Clears all events
    pub fn clear_all(&mut self) {
        self.queues.clear();
    }
}

/// Application exit event
#[derive(Debug, Clone)]
pub struct AppExit;

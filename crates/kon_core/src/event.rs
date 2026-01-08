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

    /// Reads and clears all events of a specific type
    pub fn consume<E: Event>(&mut self) -> impl Iterator<Item = E> {
        let type_id = TypeId::of::<E>();
        self.queues
            .remove(&type_id)
            .and_then(|queue| queue.downcast::<Vec<E>>().ok())
            .map(|boxed_vec| (*boxed_vec).into_iter())
            .unwrap_or_else(|| vec![].into_iter())
    }

    /// Clears all events of a specific type
    pub fn clear<E: Event>(&mut self) {
        let type_id = TypeId::of::<E>();
        if let Some(queue) = self.queues.get_mut(&type_id)
            && let Some(vec) = queue.downcast_mut::<Vec<E>>()
        {
            vec.clear();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestEvent {
        value: i32,
    }

    #[derive(Debug, Clone)]
    struct OtherEvent {
        value: &'static str,
    }

    #[test]
    fn send_and_read_event() {
        let mut events = Events::new();
        events.send(TestEvent { value: 10 });

        let received: Vec<_> = events.read::<TestEvent>().collect();
        assert_eq!(received[0].value, 10);
    }

    #[test]
    fn read_empty_queue() {
        let events = Events::new();
        assert_eq!(events.read::<TestEvent>().count(), 0);
    }

    #[test]
    fn multiple_same_type_events() {
        let mut events = Events::new();

        events.send(TestEvent { value: 10 });
        events.send(TestEvent { value: 20 });
        events.send(TestEvent { value: 30 });

        let received: Vec<_> = events.read::<TestEvent>().collect();

        assert_eq!(received.len(), 3);
        assert_eq!(received[0].value, 10);
        assert_eq!(received[1].value, 20);
        assert_eq!(received[2].value, 30);
    }

    #[test]
    fn different_event_types_separate_queues() {
        let mut events = Events::new();

        events.send(TestEvent { value: 5 });
        events.send(OtherEvent { value: "test" });

        let received_test_event: Vec<_> = events.read::<TestEvent>().collect();
        let received_other_event: Vec<_> = events.read::<OtherEvent>().collect();

        assert_eq!(received_test_event.len(), 1);
        assert_eq!(received_other_event.len(), 1);
        assert_eq!(received_test_event[0].value, 5);
        assert_eq!(received_other_event[0].value, "test");
    }

    #[test]
    fn consume_removes_events() {
        let mut events = Events::new();
        events.send(TestEvent { value: 5 });

        let consumed: Vec<_> = events.consume::<TestEvent>().collect();

        assert_eq!(consumed.len(), 1);
        assert_eq!(consumed[0].value, 5);
        assert_eq!(events.read::<TestEvent>().count(), 0);
    }

    #[test]
    fn clear_specific_type() {
        let mut events = Events::new();

        events.send(TestEvent { value: 5 });
        events.send(OtherEvent { value: "test" });

        events.clear::<TestEvent>();

        assert_eq!(events.read::<TestEvent>().count(), 0);
        assert_eq!(events.read::<OtherEvent>().count(), 1);
    }

    #[test]
    fn clear_all() {
        let mut events = Events::new();

        events.send(TestEvent { value: 5 });
        events.send(OtherEvent { value: "test" });

        events.clear_all();

        assert_eq!(events.read::<TestEvent>().count(), 0);
        assert_eq!(events.read::<OtherEvent>().count(), 0);
    }
}

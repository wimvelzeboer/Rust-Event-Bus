#![allow(dead_code)]

use std::any::{Any, TypeId};
use std::collections::HashMap;
use super::Event;
use super::Subscriber;
use log::{info, error, warn};

/// # Event Bus
///
/// The event bus is a central hub for all events.
/// It is responsible for managing all subscribers and publishing events
/// related to the event bus.
///
/// ## Fields
///
/// * `events` - A vec of events grouped by their name that have been published to the event bus.
///
/// * `subscribers` - A vec of subscribers grouped by an event name.
///
/// ## Methods
///
/// * `publish` - Publishes an event to the event bus.
///
/// * `subscribe_listener` - Subscribes a listener to the event bus.
///
/// * `run` - Runs through each event, and calls each listener's on_event method.
///
/// * `clear` - Clears all events from the event bus.

pub struct EventBus {
    /// A vec of events grouped by an event name that have been published to the event bus.
    events: HashMap<String, Vec<Box<Event>>>,
    /// A vec of all subscribers that are linked to the event bus.
    subscribers: HashMap<String, Vec<Box<dyn Subscriber>>>,

    suppress_subscribers: Option<Vec<TypeId>>,

    fail_on_error: bool
}

impl EventBus {
    /// # New
    ///
    /// Creates a new event bus.
    pub fn new() -> EventBus {
        EventBus {
            events: HashMap::new(),
            subscribers: HashMap::new(),
            suppress_subscribers: None,
            fail_on_error: true,
        }
    }

    /// # Register
    ///
    /// Registers an event with the event bus.
    pub fn register(&mut self, event_name: &str, message: Event) -> &mut Self {
        info!("EVENT: Register '{}' event with message: {:?}", event_name, &message);

        if self.events.contains_key(event_name) {
            self.events.get_mut(event_name).unwrap()
                .push(Box::new(message));
        } else {
            self.events.insert(event_name.to_string(), vec![Box::new(message)]);
        }
        self
    }

    /// # Subscribe Listener
    ///
    /// Subscribes a listener to the event bus.
    pub fn subscribe_listener<R: Subscriber + 'static>(&mut self, event_name:&str, listener: R) -> &mut Self {
        if self.subscribers.contains_key(event_name) {
            self.subscribers.get_mut(event_name).unwrap()
                .push(Box::new(listener));
        } else {
            self.subscribers.insert(event_name.to_string(), vec![Box::new(listener)]);
        }
        self
    }

    /* Upon run, messages will be cleared! */

    /// # Publish
    ///
    /// Publishes each event, and calls each listener's methods.
    /// The on_before of all listeners is called first, then the on_event and finally the on_after
    pub fn publish(&mut self) -> Result<(), String> {
        for (event, mut messages) in self.events.drain() {
            if self.subscribers.contains_key(&event) {
               'message_loop: for message in &mut messages {

                    // on before
                    for listener in self.subscribers.get_mut(&event).unwrap().iter_mut() {
                        match listener.on_before(message) {
                            Err(message) => {
                                error!("Subscriber error: {}", message);
                                if self.fail_on_error { return Err(message)}
                                break 'message_loop;
                            }
                            _ => {}
                        }
                    }

                    // on event
                    for listener in self.subscribers.get_mut(&event).unwrap().iter_mut() {
                        match listener.on_event(message) {
                            Err(message) => {
                                error!("Subscriber error: {}", message);
                                if self.fail_on_error { return Err(message)}
                                break 'message_loop;
                            }
                            _ => {}
                        }
                    }

                    // on after
                    for listener in self.subscribers.get_mut(&event).unwrap().iter_mut() {
                        match listener.on_after(message) {
                            Err(message) => {
                                error!("Subscriber error: {}", message);
                                if self.fail_on_error { return Err(message)}
                                break 'message_loop;
                            }
                            _ => {}
                        }
                    }
                }
            } else {
                warn!("No event subscribers for '{}'", event);
            }
        }
        Ok(())
    }

    pub fn suppress_subscriber<R: Subscriber + 'static>(&mut self, listener: R ) {
        let type_id = listener.type_id();
        match &mut self.suppress_subscribers {
            Some(subscribers) => {
                if subscribers.contains(&type_id) == false {
                    subscribers.push(type_id);
                }
            }
            None => {
                self.suppress_subscribers = Some(
                    vec![type_id]
                )
            }
        }
    }


    /// # Clear
    ///
    /// Clears all events from the event bus.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use log::{debug};
    use crate::{Event, EventBus, Subscriber};

    struct ExampleSubscriber {
    }

    impl ExampleSubscriber {
        const NAME: &'static str = "ExampleSubscriber";

        pub fn new() -> ExampleSubscriber {
            ExampleSubscriber { }
        }
    }

    impl Subscriber for ExampleSubscriber {

        fn on_event(&mut self, event: &mut Event) -> Result<(), String>{
            match event.get_data::<String>() {
                Some(value) => {
                    debug!("{} received STRING message: {}", ExampleSubscriber::NAME, value);
                    Ok(())
                }
                None => {
                    let message = format!("{} received UNKNOWN message", ExampleSubscriber::NAME);
                    Err(message)
                }
            }
        }
    }

    #[test]
    fn test_publisher() {
        let mut event_bus = EventBus::new();
        event_bus.subscribe_listener("bar", ExampleSubscriber::new());
        let result =
            event_bus
                .register("bar", Event::new("hello".to_string()))
                .publish();
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn test_publisher_with_invalid_payload() {
        let mut event_bus = EventBus::new();
        event_bus.subscribe_listener("bar", ExampleSubscriber::new());
        let result =
            event_bus
                .register("bar", Event::new(32u32))
                .publish();
        let message = "ExampleSubscriber received UNKNOWN message".to_string();
        let expected = Err(message.clone());
        assert_eq!(expected, result, "Expected error message: '{}'", message);
    }
}
#![allow(unused_variables)]
use super::Event;

/// # Subscriber
///
/// A subscriber is a listener that is linked to an event bus.
/// This is defined by the Subscriber trait, which can be implemented
/// by any type that wants to subscribe to an event bus.
///
/// The subscriber will receive an event when the event bus is run.
/// It will be called through the on_event method, where it
/// will also recieve an event (and the data that is held by the event).
///
/// ## Type Parameters
///
/// * `Input` - The type of data that is held by the event.
///
/// ## Methods
///
/// * `on_event` - Called when the event bus is run.
pub trait Subscriber {

    /// Called before the on_event is run by the event bus
    fn on_before(&mut self, event: &mut Event) -> Result<(), String> {
        Ok(())
    }

    /// Called when the event bus is run.
    fn on_event(&mut self, event: &mut Event) -> Result<(), String> {
        Ok(())
    }

    /// Called after the on_event is run by the event bus
    fn on_after(&self, event: &Event) -> Result<(), String> {
        Ok(())
    }
}
# Simple Event Bus

Minimal event bus in rust.
It provides a basic event bus system that works with (probably) any type.


Example Usage:

```rust
use std::string::ToString;
use simple_event_bus::{Event, EventBus, Subscriber};
use env_logger::Env;
use log::debug;

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

struct NumberSubscriber {
}

impl NumberSubscriber {
    const NAME: &'static str = "NumberSubscriber";
    pub fn new() -> NumberSubscriber {
        NumberSubscriber { }
    }
}

impl Subscriber for NumberSubscriber {
    fn on_before(&mut self, event: &mut Event) -> Result<(), String> {
        match event.get_data::<u32>() {
            Some(value) => {
                let new_data = *&value + 1;
                debug!("Changing {} into {}", value, new_data);
                event.set_data::<u32>(new_data);
                Ok(())
            }
            None => {
                let message = format!("{} received invalid message", NumberSubscriber::NAME);
                Err(message)
            }
        }
    }

    fn on_event(&mut self, event: &mut Event) -> Result<(), String> {
        match event.get_data::<u32>() {
            Some(value) => {
                debug!("{} received u32 message: {}", NumberSubscriber::NAME, value);
                Ok(())
            }
            None => {
                let message = format!("{} received invalid message", NumberSubscriber::NAME);
                Err(message)
            }
        }
    }
}


fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let mut event_bus = EventBus::new();

    // We have to manually create and add each subscriber to the event bus.
    event_bus.subscribe_listener("bar", ExampleSubscriber::new());
    event_bus.subscribe_listener("foo", NumberSubscriber::new());

    // We can manually register an event to the event bus.
    event_bus.register("foo", Event::new(42u32));
    event_bus.register("bar", Event::new("hello".to_string()));
    event_bus.register("foo", Event::new("hello".to_string()));
    event_bus.register("hello", Event::new("hello".to_string()));

    // Publishes each event, and calls each listener's on_* methods.
    event_bus.publish();
}
```

Feel free to fork this implementation to add your own features!
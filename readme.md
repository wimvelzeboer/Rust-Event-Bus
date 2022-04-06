# Simple Event Bus

Minimal event bus in rust.
It provides a basic event bus system that works with (probably) any type.


Example Usage:

```rust
use simple_event_bus::{Event, EventBus, Subscriber};

struct ExampleSubscriber{
    pub name: String,
}

impl ExampleSubscriber{
    pub fn new(name: String) -> ExampleSubscriber{
        ExampleSubscriber{
            name: name,
        }
    }
}

impl Subscriber for ExampleSubscriber{
    type Input = String;

    fn on_message(&mut self, message: &Event<Self::Input>){
        println!("{} received message: {}", self.name, message.get_data());
    }
}

fn main(){
    let mut message_queue = EventBus::new();
    
    message_queue.subscribe_listener(Box::new(ExampleSubscriber::new("listener 1".to_string())));
    message_queue.subscribe_listener(Box::new(ExampleSubscriber::new("listener 2".to_string())));

    message_queue.publish(Event::new("hello".to_string()));
    message_queue.publish(Event::new("world".to_string()));
    
    // Note: Once all active subscribers have processed the published events,
    // the bus will be cleared of all messages.
    message_queue.run();
}
```

Feel free to fork this implementation to add your own features!
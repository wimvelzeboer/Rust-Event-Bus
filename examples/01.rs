use simple_event_bus::{Event, EventBus, Subscriber};

struct ExampleSubscriber {
    pub name: String,
}

impl ExampleSubscriber {
    pub fn new(name: String) -> ExampleSubscriber {
        ExampleSubscriber { name }
    }
}

impl Subscriber for ExampleSubscriber {
    fn on_event(&mut self, event: &mut Event) -> Result<(), String>{
        match event.get_data::<String>() {
            Some(value) => {
                println!("{} received STRING message: {}", self.name, value);
                Ok(())
            }
            None => {
                let message = format!("{} received UNKNOWN message", self.name);
                Err(message)
            }
        }
    }
}

struct NumberSubscriber {
    pub name: String,
}

impl NumberSubscriber {
    pub fn new(name: String) -> NumberSubscriber {
        NumberSubscriber { name }
    }
}

impl Subscriber for NumberSubscriber {
    fn on_before(&mut self, event: &mut Event) -> Result<(), String> {
        match event.get_data::<u32>() {
            Some(value) => {
                let new_data = *&value + 1;
                println!("Changing {} into {}", value, new_data);
                event.set_data::<u32>(new_data);
                Ok(())
            }
            None => {
                let message = format!("{} received invalid message", self.name);
                Err(message)
            }
        }
    }

    fn on_event(&mut self, event: &mut Event) -> Result<(), String> {
        match event.get_data::<u32>() {
            Some(value) => {
                println!("{} received u32 message: {}", self.name, value);
                Ok(())
            }
            None => {
                let message = format!("{} received invalid message", self.name);
                Err(message)
            }
        }
    }
}


fn main() {
    let mut event_bus = EventBus::new();

    // We have to manually create and add each subscriber to the event bus.
    event_bus.subscribe_listener("bar".to_string(), ExampleSubscriber::new("String Subscriber 1".to_string()));
    event_bus.subscribe_listener("bar".to_string(), ExampleSubscriber::new("String Subscriber 2".to_string()));
    event_bus.subscribe_listener("foo".to_string(), NumberSubscriber::new("Number Subscriber".to_string()));

    // We can manually define an event and publish it to the event bus.
    // event_bus.publish(Event::new("hello".to_string()));
    let number: u32 = 32;
    event_bus.publish("foo".to_string(), Event::new(number));
    event_bus.publish("bar".to_string(), Event::new("hello".to_string()));
    event_bus.publish("foo".to_string(), Event::new("hello".to_string()));


    // Runs through each event, and calls each listener's on_event method.
    event_bus.run();
}

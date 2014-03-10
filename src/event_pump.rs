use rsfml::window::event;
use std::hashmap::HashMap;


pub trait EventHandler {
	fn handle_event(&mut self, event : &event::Event);
}

pub struct EventPump {
	handlers: ~[~EventHandler],
}

impl EventPump {
	pub fn new() -> EventPump {
		EventPump { handlers: ~[] }
	}
	pub fn attach(&mut self, handler : ~EventHandler) {
		self.handlers.push(handler);
	}

	pub fn pump(&mut self, event: &event::Event) {
		for handler in self.handlers.mut_iter() {
			handler.handle_event(event);
		}
	}

}


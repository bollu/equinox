use rsfml::window::event;
use std::hashmap::HashMap;
use collections::ringbuf::RingBuf;
use collections::deque::Deque;

pub trait EventHandler {
	fn handle_event(&mut self, event : &event::Event);
}

pub struct EventQueue<'a> {
	handlers: RingBuf<&'a mut EventHandler>,
}

impl<'a> EventQueue<'a> {
	pub fn new() -> EventQueue {
		EventQueue { handlers: RingBuf::new() }
	}
	pub fn attach(&mut self, handler : &'a mut EventHandler) {
		self.handlers.push_back(handler);
	}

	
	pub fn send_event(&mut self, event: &event::Event) {
		for handler in self.handlers.mut_iter() {
			handler.handle_event(event);
		}
	}
	
	pub fn clear(&mut self) {
		self.handlers.clear();
	}

}


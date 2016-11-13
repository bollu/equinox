use sfml::window::event;

use std::collections::VecDeque;

pub trait EventHandler {
	fn handle_event(&mut self, event : &event::Event);
}

pub struct EventQueue<'a> {
	handlers: VecDeque<&'a mut EventHandler>,
}

impl<'a> EventQueue<'a> {
	pub fn new() -> EventQueue<'a> {
		EventQueue { handlers: VecDeque::new() }
	}
	pub fn push(&mut self, handler : &'a mut EventHandler) {
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


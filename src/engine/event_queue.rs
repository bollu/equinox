use rsfml::window::event;
use rsfml::window::keyboard::Key;
use rsfml::window::mouse::MouseButton;
use rsfml::system::vector2::Vector2i;

use collections::ringbuf::RingBuf;
use collections::deque::Deque;

use engine::rendering::Window;
use engine::math::Vector2;

pub enum RawEvent {
	Closed,
	KeyPressed {
		code: Key,
		alt: bool,
		ctrl: bool,
		shift: bool,
		system: bool,
	},

	KeyReleased {
		code: Key,
		alt: bool,
		ctrl: bool,
		shift: bool,
		system: bool,
	},

	MouseWheelMoved {
		delta: int,
		pos: Vector2,
	},

	MouseButtonPressed {
		button: MouseButton,
		pos: Vector2,
	},

	MouseButtonReleased {
		button: MouseButton,
		pos: Vector2,
	},

	MouseMoved {
		pos: Vector2,
	},
	NoEvent,
}

pub trait EventHandler {
	fn handle_event(&mut self, event : &RawEvent);
}

pub struct EventQueue<'a> {
	handlers: RingBuf<&'a mut EventHandler>,
}

impl<'a> EventQueue<'a> {
	pub fn new() -> EventQueue {
		EventQueue { handlers: RingBuf::new() }
	}
	pub fn push(&mut self, handler : &'a mut EventHandler) {
		self.handlers.push_back(handler);
	}

	
	pub fn send_event(&mut self, window: &Window, event: &event::Event) {
		let raw_event = EventQueue::sfml_to_raw_event(window, event);

		for handler in self.handlers.mut_iter() {
			handler.handle_event(&raw_event);
		}
	}
	
	pub fn clear(&mut self) {
		self.handlers.clear();
	}

	fn sfml_to_raw_event(window: &Window, event: &event::Event) -> RawEvent {
		return match *event {
			event::Closed => Closed,

			event::KeyPressed { code, alt, ctrl, shift, system } => 
			KeyPressed { 
				code: code,
				alt: alt,
				ctrl: ctrl,
				shift: shift,
				system: system
			},

			event::KeyReleased { code, alt, ctrl, shift, system } => 
			KeyReleased { 
				code: code,
				alt: alt,
				ctrl: ctrl,
				shift: shift,
				system: system
			},

			event::MouseWheelMoved { delta, x, y } =>
			MouseWheelMoved { delta: delta, pos: screen_to_game(window, x, y) },

			event::MouseMoved {x, y} =>
			MouseMoved { pos: screen_to_game(window, x, y) },

			event::MouseButtonPressed {button, x, y} =>
			MouseButtonPressed { button: button, pos: screen_to_game(window, x, y) },

			event::MouseButtonReleased {button, x, y} =>
			MouseButtonReleased { button: button, pos: screen_to_game(window, x, y) },

			_ => NoEvent,
		} //end match

	}
}


fn screen_to_game(window: &Window, x: int, y: int) -> Vector2 {
	 window.screen_to_game(&Vector2i::new(x as i32, y as i32))
}
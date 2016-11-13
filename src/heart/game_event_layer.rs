use sfml::window::Key;

use sfml::window::event;

use engine::event_queue::{EventHandler};
use engine::settings::Settings;

type keyboard = Key;

pub enum GameEvents {
	PlayerLeft,
	PlayerRight,
	//PlayerTop,
	//PlayerBottom,

	PlayerFireSide,
	PlayerFireTop,
	PlayerFireBottom,
	
	PlayerJump,

	MetaPause, //escape
	MetaQuit, //alt+f4 :(
}

pub struct GameEventLayer {
	moving_left: bool,
	moving_right: bool,

	facing_top: bool,
	firing: bool,

	jumping: bool,

	meta_paused: bool,
	meta_quit: bool,
}

impl GameEventLayer {
	pub fn new(settings: &Settings) -> GameEventLayer {
		GameEventLayer{
			moving_left: false,
			moving_right: false,
			facing_top: false,
			firing: false,
			jumping: false,

			meta_paused: false,
			meta_quit: false,
		}
	}


	pub fn should_quit(&self) -> bool {
		self.meta_quit
	}
	
	pub fn handle_key_press(&mut self, key: Key) {
		match key {
			keyboard::Up => { self.facing_top = true },
			keyboard::Left => { self.moving_left = true },
			keyboard::Right => { self.moving_right = true },

			keyboard::Escape => { self.meta_paused = true },
			_ => return
		};
	}

	pub fn handle_key_release(&mut self, key: Key) {
		match key {
			keyboard::Up => { self.facing_top = false },
			keyboard::Left => { self.moving_left = false },
			keyboard::Right => { self.moving_right = false } ,
			_ => return,
		};
	}

}

impl EventHandler for GameEventLayer {
	fn handle_event(&mut self, event : &event::Event) {

		match *event {
			event::KeyPressed { code, .. }  => self.handle_key_press(code),
			event::KeyReleased { code, .. }  => self.handle_key_release(code),

			event::Closed => self.meta_quit = true,

			_ => return
		};
	}
}

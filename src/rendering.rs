extern crate rsfml;

use rsfml::graphics::{RenderWindow, Color};
use rsfml::window::{event, ContextSettings, VideoMode, event, Close, Fullscreen, WindowStyle};

pub struct Window {
	window: RenderWindow,
	clearColor: Color,
}
impl Window {
	pub fn new(width: uint, height: uint, title: ~str, fullscreen: bool) -> Window{
		
		let videoMode = VideoMode::new_init(width, height, 32);
		let style : WindowStyle = if fullscreen { Fullscreen } else { Close };
		let setting = ContextSettings::default();

		Window { window: RenderWindow::new(videoMode, title, style, &setting).unwrap(), 
				clearColor: Color::new_RGB(255, 0, 0) }
	}

	pub fn set_clear_color(&mut self, clearColor : Color) {
		self.clearColor = clearColor;
	}

	pub fn is_open(&self) -> bool {
		self.window.is_open()
	}

	pub fn poll(&mut self) -> event::Event {
		self.window.poll_event()
	}
	pub fn close(&mut self) {
		self.window.close()
	}

	pub fn clear(&mut self) {
		self.window.clear(&self.clearColor);
	}

	pub fn display(&mut self) {
		self.window.display();
	}

	pub fn width(&self) -> uint {
		self.window.get_size().x as uint
	}
	
	pub fn height(&self) -> uint {
		self.window.get_size().y as uint
	}

	//only use if you're sure
	pub fn get_render_window<'a>(&'a mut self) -> &'a mut RenderWindow {
		&mut self.window
	}
}



pub struct RenderPump {
	renderers: ~[~rsfml::traits::Drawable:],
}

impl RenderPump {
	pub fn new() -> RenderPump {
		RenderPump { renderers: ~[] }
	}

	pub fn attach(&mut self, renderer: ~rsfml::traits::Drawable: ) {
		self.renderers.push(renderer);
	}

	pub fn pump(&self, window: &mut Window) -> () {
		for renderer in self.renderers.iter() {
			renderer.draw_in_render_window(window.get_render_window())
		}
	}

}

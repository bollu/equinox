extern crate rsfml;
use engine::math::Vector2;

use rsfml::system::vector2::Vector2i;
use rsfml::graphics::{RenderWindow, Color};
use rsfml::window::{ContextSettings, VideoMode, event, Close, Fullscreen, WindowStyle};
use rsfml::graphics::View;
use rsfml::traits::Drawable;

use collections::ringbuf::RingBuf;
use collections::deque::Deque;

use std::rc::Rc;
use std::cell::RefCell;


pub struct RenderContext {
	viewport_dim: Vector2,
}

pub struct Window {
	window: RenderWindow,
}

impl Window {
	pub fn new(width: uint, height: uint, title: ~str, fullscreen: bool) -> Window{
		
		let videoMode = VideoMode::new_init(width, height, 32);
		let style : WindowStyle = if fullscreen { Fullscreen } else { Close };
		let setting = ContextSettings::default();

		Window { window: RenderWindow::new(videoMode, title, style, &setting).unwrap() }
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

	pub fn clear(&mut self, clear_color: Color) {
		self.window.clear(&clear_color);
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

	pub fn get_context(&self) -> RenderContext {
		let size = self.window.get_size();
		RenderContext { viewport_dim: Vector2::new(size.x as f32, size.y as f32) }
	}


	pub fn screen_to_game(&self, screen: &Vector2i) -> Vector2 {
		Vector2::from_sfml_f(&self.window.map_pixel_to_coords_current_view(screen))
	}

	//only use if you're sure
	pub fn get_render_window<'a>(&'a mut self) -> &'a mut RenderWindow {
		&mut self.window
	}
}

pub trait EngineDrawable {
	fn Draw(&mut self, render_window: &mut RenderWindow);
}

#[feature(macro_rules)]
macro_rules! impl_engine_drawable {
    ($T:ty) => {
        impl<'a> EngineDrawable for $T {
            fn Draw(&mut self, render_window: &mut RenderWindow) {
                self.draw_in_render_window(render_window);
            }
        }
    }
}

impl_engine_drawable!(rsfml::graphics::RectangleShape<'a>)
impl_engine_drawable!(rsfml::graphics::Text<'a>)
impl_engine_drawable!(rsfml::graphics::CircleShape<'a>)

pub struct RenderQueue<'a> {
	renderers: RingBuf<&'a mut EngineDrawable:>,
	clear_color: Color,
	view: Rc<RefCell<View>>,
}

impl<'a> RenderQueue<'a> {
	pub fn new(window: &mut Window) -> RenderQueue { 

		let mut view = window.get_render_window().get_default_view();
		
		RenderQueue { 
			renderers: RingBuf::new(), 
			clear_color: Color::new_RGB(0, 0, 20), 
			view: view,
		}
		
	}

	pub fn push(&mut self, renderer: &'a mut EngineDrawable:) {
		self.renderers.push_back(renderer);
	}


	pub fn set_clear_color(&mut self, clear_color: Color) {
		self.clear_color = clear_color;
	}

	pub fn render(&mut self, window: &mut Window) {
		window.clear(self.clear_color);
		let mut render_window = window.get_render_window();

		render_window.set_view(self.view.clone()); 
		
		


		for renderer in self.renderers.mut_iter() {
			renderer.Draw(render_window);
		}

		self.renderers.clear();
	}

	pub fn get_view(&mut self) -> Rc<RefCell<View>> {
		self.view.clone()
	}
}


pub struct Camera {
	center: Vector2,
}

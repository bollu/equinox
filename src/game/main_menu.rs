use engine::state::{State, StateTransition, NoTransition};

use engine::resource_loader::{ResourceLoader, Font};
use engine::rendering::{RenderContext, RenderPump};

use engine::event_pump::{EventPump, EventHandler};

use rsfml::graphics::{Color, Text, FloatRect, RectangleShape, Font};
use rsfml::window::event; 

use game::colors;


struct MenuItem<'a> {
	text: Text<'a>,
	back: RectangleShape<'a>,
	default_color: Color,
	active: bool,
}

impl<'a> MenuItem<'a> {
	pub fn new(label: &str, font: &'a Font, font_size: uint, text_color: &Color, back_color: &Color) -> MenuItem<'a> {
		
		let mut text = Text::new_init(label, font, font_size).unwrap();
		text.set_color(text_color);

		let mut back = RectangleShape::new().unwrap();
		back.set_fill_color(&colors::invisible);

		MenuItem{ text: text, back: back, default_color: *back_color, active: false}

	}

	pub fn set_bounds(&mut self, total_x: f32, y: f32, item_height: f32) {
		let text_x = MenuItem::calculate_text_x(&self.text, total_x);
		self.text.set_position2f(text_x, y);
		
		self.back.set_size2f(total_x, item_height);
		self.back.set_position2f(0 as f32, y);
	}

	pub fn update_state(&mut self, mouse_x: f32, mouse_y: f32) {
		if self.back.get_global_bounds().contains(mouse_x,mouse_y) {
			self.active = true;
			self.back.set_fill_color(&self.default_color);
		} else {
			self.active = false;
			self.back.set_fill_color(&colors::invisible);
		}
	}

	pub fn calculate_text_x<'b>(text: &'b Text, total_x: f32) -> f32{
		total_x * 0.5 - text.get_local_bounds().width * 0.5
	}

	fn attach_to_pump(&self, pump: &mut RenderPump) {
		pump.attach(&self.back);
		pump.attach(&self.text);
		
	}
}

pub struct MainMenu<'a>{
	banner: Text<'a>,
	menu_items: ~[MenuItem<'a>],
	handler: MouseHandler,
}

impl<'a> MainMenu<'a>{
	pub fn new(loader: &'a ResourceLoader, render_ctx: &RenderContext) -> MainMenu<'a> {

		let top_padding = 10.;
		let banner_menu_padding = 150.;

		let font_size_banner = 90;
		let font_size_menu = 70;
		let font = loader.getFont(~"MenuFont");

		let menu_names = ~[&"play", &"options", &"quit"];
		let menu_back_colors =  ~[colors::blue, colors::green, colors::red];
		let mut menu_items = ~[];

		
		let render_dim = render_ctx.viewport_dim;
		let mut current_y = top_padding;
		let mut current_x = 0.;
		
		//banner---------
		let mut banner = Text::new_init("equinox", font, font_size_banner).unwrap();
		banner.set_color(&colors::white);

		current_x = MenuItem::calculate_text_x(&banner, render_dim.x);
		banner.set_position2f(current_x, current_y);
		
		current_y += banner.get_local_bounds().height;
		current_y += banner_menu_padding;
		
		
		//rest of it-----
		let y_spacing = (render_dim.y - current_y) / menu_names.len() as f32;
		
		let mut index = 0;
		for name in menu_names.iter() {

			let menu_color = menu_back_colors[index];
			let mut item = MenuItem::new(*name, font, font_size_menu, &colors::white, &menu_color);
			item.set_bounds(render_dim.x, current_y, y_spacing);
			menu_items.push(item);

			current_y += y_spacing;
			index += 1;
			
		}

		return MainMenu { 
			banner: banner, 	
			menu_items: menu_items,
			handler: MouseHandler::new() 
		}
		
	}
}

impl<'a> State for MainMenu<'a> {
	fn attach_event_handlers(&mut self, pump: &mut EventPump){
		pump.attach(&mut self.handler);
	}
	
	fn update_renderers(&mut self, pump: &mut RenderPump){
		pump.set_clear_color(colors::black);

		pump.attach(&self.banner);

		for item in self.menu_items.iter() {
			item.attach_to_pump(pump);
		}
		
	}

	fn Tick(&mut self, dt: f32) -> StateTransition { 
		
		for item in self.menu_items.mut_iter() {
			item.update_state(self.handler.x as f32, self.handler.y as f32);
		}

		return NoTransition;
	}
}


struct MouseHandler {
	x: int,
	y: int,
	clicked: bool,
}

impl MouseHandler {
	pub fn new() -> MouseHandler {
		MouseHandler { x: 0, y: 0, clicked: false}
	}

	pub fn hovering(&self, aabb: &FloatRect) -> bool { aabb.contains(self.x as f32, self.y as f32)  }
	pub fn clicked(&self, aabb: &FloatRect) -> bool { self.clicked && aabb.contains(self.x as f32, self.y as f32) }
}

impl EventHandler for MouseHandler {
	fn handle_event(&mut self, event : &event::Event) {
		match *event {
			event::MouseMoved { x, y } => {
				self.x = x; 
				self.y = y;
			},

			_ => return
		} 
	}	
}
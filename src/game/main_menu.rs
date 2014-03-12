use std::mem::replace;

use engine::state::{State, StateTransition, NoTransition};

use engine::resource_loader::{ResourceLoader, Font};
use engine::rendering::{RenderContext, RenderPump};

use engine::event_pump::{EventPump, EventHandler};

use rsfml::graphics::{Text, FloatRect, RectangleShape};
use rsfml::graphics;
use rsfml::window::event; 

use game::colors;

fn calculate_text_x(viewport_width: f32, text: &Text) -> f32 {
	viewport_width * 0.5 - text.get_local_bounds().width * 0.5
}

pub struct MainMenu<'a>{
	banner: Text<'a>,
	menu_opts: ~[Text<'a>],
	menu_backs: ~[RectangleShape<'a>],
	transition_ids: ~[uint],
	current_index: uint,
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
		let mut menu_opts = ~[];
		
		let menu_back_colors =  ~[&colors::blue, &colors::green, &colors::red];
		let mut menu_backs = ~[];

		let render_dim = render_ctx.viewport_dim;
		let mut current_y = top_padding;
		let mut current_x = 0.;
		
		//banner---------
		let mut banner = Text::new_init(~"equinox", font, font_size_banner).unwrap();
		banner.set_color(&colors::white);

		current_x = calculate_text_x(render_dim.x, &banner);
		banner.set_position2f(current_x, current_y);
		
		current_y += banner.get_local_bounds().height;
		current_y += banner_menu_padding;
		
		
		//rest of it-----
		let y_spacing = (render_dim.y - current_y) / menu_names.len() as f32;
		
		let mut index = 0;
		for name in menu_names.iter() {

			//text of menu
			let mut text = Text::new_init(*name, font, font_size_menu).unwrap();
			text.set_color(&colors::white);

			current_x = calculate_text_x(render_dim.x, &text);
			text.set_position2f(current_x, current_y);
			menu_opts.push(text);

			//back part of menu
			let mut menu_back = RectangleShape::new().unwrap();
			menu_back.set_size2f(render_dim.x, y_spacing);
			menu_back.set_position2f(0. , current_y);
			let menu_color = menu_back_colors[index];
			menu_back.set_fill_color(menu_color);

			menu_backs.push(menu_back);

			current_y += y_spacing;
			index += 1;
		}

		return MainMenu { 
			banner: banner, 
			menu_opts: menu_opts,
			menu_backs: menu_backs,
			transition_ids: ~[], 
			current_index: 0, handler: 
			MouseHandler::new() 
		}
		
	}
}

impl<'a> State for MainMenu<'a> {
	fn attach_event_handlers(&mut self, pump: &mut EventPump){

	}
	
	fn update_renderers(&mut self, pump: &mut RenderPump){
		pump.set_clear_color(colors::black);

		pump.attach(&self.banner);

		for back in self.menu_backs.iter() {
			pump.attach(&*back);
		}

		for opt in self.menu_opts.iter() {
			pump.attach(&*opt);
		}
		
	}

	fn Tick(&mut self, dt: f32) -> StateTransition { NoTransition }
}


struct MouseHandler {
	x: int,
	y: int,
}

impl MouseHandler {
	pub fn new() -> MouseHandler {
		MouseHandler { x: 0, y: 0}
	}
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
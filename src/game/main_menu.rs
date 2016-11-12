use std::vec_ng::Vec;

use engine::state::{State, EngineState, NoChange, StateTransition, EngineShutdown, NoStateData, IntStateData};

use engine::resource_loader::{ResourceLoader};
use engine::rendering::{RenderContext, RenderQueue};

use engine::event_queue::EventQueue;

use rsfml::graphics::Text;

use game::colors;
use game::ui::{calculate_text_centering_x, MenuItem, SimpleMenuHandler};
use game;

pub enum MenuItemTag {
	TagPlay,
	TagOptions,
	TagQuit,
}

pub struct MainMenu<'a> {
	banner: Text<'a>,
	menu_items: Vec<MenuItem<'a, MenuItemTag>>,
	handler: SimpleMenuHandler,
}


impl<'a> MainMenu<'a>{
	pub fn new(loader: &'a ResourceLoader, render_ctx: &RenderContext) -> MainMenu<'a> {

		let top_padding = 10.;
		let banner_menu_padding = 150.;

		let font_size_banner = 90;
		let font_size_menu = 70;
		let font = loader.get_font("MenuFont".to_string());

		let render_dim = render_ctx.viewport_dim;
		let mut current_y = top_padding;
		let mut current_x : f32 = 0.;
		
		//banner-----------------
		let mut banner = Text::new_init("equinox", font, font_size_banner).unwrap();
		banner.set_color(&colors::white);

		current_x = calculate_text_centering_x(&banner, render_dim.x);
		banner.set_position2f(current_x, current_y);
		
		current_y += banner.get_local_bounds().height;
		current_y += banner_menu_padding;
		
		
		//create menu items----------------
		let menu_tags = [TagPlay, TagOptions, TagQuit];
		let menu_names = ["play", "options", "quit"];
		let menu_back_colors = [colors::green, colors::blue, colors::red];
	
		
		let mut menu_items = Vec::new();
		//space between menu items
		let y_spacing = (render_dim.y - current_y) / menu_names.len() as f32;
		

		for i in range(0, menu_names.len()) {	
			let name = menu_names[i];
			let tag = menu_tags[i];
			let color = menu_back_colors[i];

			let mut item = MenuItem::new(tag, name, font, font_size_menu, &colors::white, &color);
			item.set_bounds(render_dim.x, current_y, y_spacing);

			menu_items.push(item);
			current_y += y_spacing;
			
		}

		return MainMenu { 
			banner: banner, 	
			menu_items: menu_items,
			handler: SimpleMenuHandler::new() 
		}
	}

	fn handle_click(tag: MenuItemTag) -> EngineState{
		match tag {
			TagPlay => StateTransition(game::SlotSelectStateID as isize, IntStateData(0)),
			TagOptions => StateTransition(game::OptionsStateID as isize, NoStateData),
			TagQuit => EngineShutdown,
		}
	}
}

impl<'a> State for MainMenu<'a> {
	fn queue_event_handlers(&mut self, event_queue: &mut EventQueue){
		event_queue.push(&mut self.handler);
	}
	
	fn queue_renderers(&mut self, render_queue: &mut RenderQueue){
		render_queue.set_clear_color(colors::black);
		render_queue.push(&self.banner);

		for item in self.menu_items.iter() {
			item.push_to_queue(render_queue);
		}
	}

	fn tick(&mut self, dt: f32) -> EngineState { 
		
		for item in self.menu_items.mut_iter() {
			item.update_state(dt, self.handler.x, self.handler.y);
			
			if self.handler.clicked && item.contains(self.handler.x, self.handler.y) {
				return MainMenu::handle_click(item.data)
			}	
		}
		
		self.handler.reset();
		NoChange
	}
}

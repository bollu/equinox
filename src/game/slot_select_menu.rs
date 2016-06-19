use std::vec_ng::Vec;

use engine::state::{State, EngineState, NoChange, StateTransition, IntStateData, EngineShutdown};
use engine::resource_loader::{ResourceLoader};
use engine::rendering::{RenderContext, RenderQueue};
use engine::event_queue::{EventQueue};

use game;
use game::colors;
use game::ui::{MenuItem, SimpleMenuHandler};
use game::NUM_SLOTS;
use game::savefile::{Savefile, read_save_from_disk, write_save_to_disk, save_exists};


pub struct SlotSelectMenu<'a> {
	menu_items: Vec<MenuItem<'a, int>>,
	handler: SimpleMenuHandler,

}

impl<'a> SlotSelectMenu<'a> {
	pub fn new(loader: &'a ResourceLoader, render_ctx: &RenderContext) -> SlotSelectMenu<'a> {

		let top_padding = 0.;
		let font_size_menu = 40;
		let font = loader.get_font("MenuFont".to_string());

		let render_dim = render_ctx.viewport_dim;
		let mut current_y = top_padding;
		
		//create menu items-------------------
		let mut menu_items = Vec::new(); 
		
		//space between menu items
		let y_spacing = (render_dim.y - current_y) / NUM_SLOTS as f32;

		for index in range(0, NUM_SLOTS) {

			let savefile = {
				let save_path = gen_save_path(index);

				if save_exists(save_path) {
					read_save_from_disk(save_path)

				} else {
					let save = SlotSelectMenu::create_first_time_save(index);
					write_save_to_disk(&save, save_path);
					save
				}
			};

			let name = "slot " + index.to_str();

			let color = if savefile.open {
				colors::green
			} else {
				colors::blue
			};


			let mut item = MenuItem::new(index, name, font, font_size_menu, &colors::white, &color);
			item.set_bounds(render_dim.x, current_y, y_spacing);

			menu_items.push(item);
			current_y += y_spacing;
			
		}

		SlotSelectMenu {
			menu_items: menu_items,
			handler: SimpleMenuHandler::new(),
		}
	}

	pub fn create_first_time_save(index: int) -> Savefile{
		Savefile::new(index, true, 0)
	}

	pub fn handle_click(data: int) -> EngineState {
		StateTransition(game::GameStateID as int, IntStateData(data))
	} 
}

impl<'a> State for SlotSelectMenu<'a> {
	fn queue_event_handlers(&mut self, event_queue: &mut EventQueue){
		event_queue.push(&mut self.handler);
	}
	
	fn queue_renderers(&mut self, render_queue: &mut RenderQueue){
		render_queue.set_clear_color(colors::black);
		
		for item in self.menu_items.iter() {
			item.push_to_queue(render_queue);
		}
	}

	fn tick(&mut self, dt: f32) -> EngineState { 
		
		for item in self.menu_items.mut_iter() {
			item.update_state(dt, self.handler.x, self.handler.y);
			
			if self.handler.clicked && item.contains(self.handler.x, self.handler.y) {
				return SlotSelectMenu::handle_click(item.data)	
			}	
		}

		return if self.handler.should_quit() {
			EngineShutdown
		} else {
			NoChange
		}
	}
}


fn gen_save_path(index: int) -> String {
	"slot_" + index.to_str()
}

use rsfml::graphics;
use engine::resource_loader::{ResourceLoader, Font};

use engine::state::StateMachine;

use engine::rendering::RenderContext;

pub mod colors;
pub mod main_menu;


pub fn load_resources(loader: &mut ResourceLoader) {
 	let obelix_font = graphics::Font::new_from_file("res/font/AlegreyaSansSC-Light.ttf").unwrap();
    loader.addResource(~"MenuFont", Font(obelix_font));
}

pub enum State {
	MAIN_MENU_STATE_ID = 1,
	GAME_STATE_ID = 2,
	OPTIONS_STATE_ID = 3,
}

pub fn init_states(state_machine: &mut StateMachine, loader: &ResourceLoader, ctx: &RenderContext) {
    state_machine.add_state(MAIN_MENU_STATE_ID as int, ~main_menu::MainMenu::new(loader, ctx));
    state_machine.set_default_state(MAIN_MENU_STATE_ID as int);
}


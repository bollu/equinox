use sfml::graphics;
use engine::resource_loader::{ResourceLoader, Resource};

use engine::state::StateMachine;
use engine::rendering::RenderContext;
use engine::settings::Settings;

use self::State::*;

pub mod colors;
pub mod main_menu;
pub mod game_state;
pub mod slot_select_menu;
pub mod level;
pub mod ui;
pub mod savefile;

pub fn load_resources(loader: &mut ResourceLoader) {
 	let obelix_font = graphics::Font::new_from_file("res/font/AlegreyaSansSC-Light.ttf").unwrap();
    loader.add_resource("MenuFont".to_string(), Resource::Font(obelix_font));
}

pub enum State {
	MainMenuStateID,
	GameStateID,
	SlotSelectStateID,
	OptionsStateID,
}

pub static NUM_SLOTS : isize = 6;


pub fn init_states(state_machine: &mut StateMachine, 
				loader: &ResourceLoader, 
				ctx: &RenderContext, 
				settings: &Settings) {

    state_machine.add_state(MainMenuStateID as isize, Box::new(main_menu::MainMenu::new(loader, ctx)));
    state_machine.set_default_state(MainMenuStateID as isize);

    state_machine.add_state(SlotSelectStateID as isize, Box::new(slot_select_menu::SlotSelectMenu::new(loader, ctx)));
    state_machine.add_state(GameStateID as isize, Box::new(game_state::GameState::new(loader, ctx, settings)));
}


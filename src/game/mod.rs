use rsfml::graphics;
use engine::resource_loader::{ResourceLoader, Font};

use engine::state::StateMachine;
use engine::rendering::RenderContext;
use engine::settings::Settings;

pub mod colors;
pub mod main_menu;
pub mod game_state;
pub mod slot_select_menu;
pub mod level;
pub mod terrain;
pub mod ui;
pub mod savefile;

pub fn load_resources(loader: &mut ResourceLoader) {
 	let obelix_font = graphics::Font::new_from_file("res/MenuFont.ttf").unwrap();
    loader.add_resource(~"MenuFont", Font(obelix_font));
}

pub enum State {
	MainMenuStateID,
	GameStateID,
	SlotSelectStateID,
	OptionsStateID,
}

pub static NUM_SLOTS : int = 6;


pub fn init_states(state_machine: &mut StateMachine, 
				loader: &ResourceLoader, 
				ctx: &RenderContext, 
				settings: &Settings) {

    state_machine.add_state(MainMenuStateID as int, ~main_menu::MainMenu::new(loader, ctx));
    state_machine.set_default_state(MainMenuStateID as int);

    state_machine.add_state(SlotSelectStateID as int, ~slot_select_menu::SlotSelectMenu::new(loader, ctx));
    state_machine.add_state(GameStateID as int, ~game_state::GameState::new(loader, ctx, settings));
}


use rsfml::graphics;
use engine::resource_loader::{ResourceLoader, Font};
use engine::state::StateMachine;

pub mod colors;
pub mod main_menu;


fn load_resources(loader: &mut ResourceLoader) {
 	let obelix_font = graphics::Font::new_from_file("res/font/Obelix.ttf").unwrap();
    loader.addResource(~"MenuFont", Font(obelix_font));
}

fn init_states(state_machine: &mut StateMachine, loader: &ResourceLoader) {
    state_machine = StateMachine::new(0, main_menu::MainMenu::new(loader));
}


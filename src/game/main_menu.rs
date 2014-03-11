use engine::state::State;

use engine::resource_loader::{ResourceLoader, Font};
use rsfml::graphics;

pub struct MainMenu {
	menu_opts: ~[graphics::Text],
	transition_ids: ~[uint],
	current_index: uint,
}

impl MainMenu {
	pub fn new(loader: &ResourceLoader) -> MainMenu {
		let font = loader.getFont(~"MennuFont");
	}
}
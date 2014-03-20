use std::vec_ng::Vec;
use engine::rendering::RenderQueue;


pub enum LevelState {
	QuitGame,
	NoChange,
	NextLevel,
	RestartLevel,
}

pub struct Level {
	active: bool,
}

impl Level {
	pub fn new() -> Level {
		Level {
			active: true
		}
	}

	pub fn queue_renderers(&mut self, render_queue: &mut RenderQueue){}

	pub fn update(&mut self, dt: f32) -> LevelState { 
		NoChange
	}

}
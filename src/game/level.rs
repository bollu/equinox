use std::vec::Vec;
use engine::math::Vector2;
use engine::rendering::RenderQueue;

use heart::player::Player;

pub enum LevelState {
	QuitGame,
	NoChange,
	NextLevel,
	RestartLevel,
}

pub struct Level<'a> {
	active: bool,

	player: Player<'a>,
}

impl<'a> Level<'a> {
	pub fn new() -> Level<'a> {
		Level {
			active: true,
			player: Player::new(Vector2::new(300., 400.))
		}
	}

	pub fn queue_renderers(&mut self, render_queue: &mut RenderQueue){
		self.player.push_to_queue(render_queue);
	}

	pub fn update(&mut self, dt: f32) -> LevelState { 
		self.player.update(dt);
		NoChange
	}

}

use std::Vec;
use engine::math::Vector2;
use engine::rendering::RenderQueue;

use heart::player::Player;
use heart::tilemap::Tilemap;

pub enum LevelState {
	QuitGame,
	NoChange,
	NextLevel,
	RestartLevel,
}

pub struct Level<'a> {
	active: bool,
	player: Player<'a>,
	tilemap: Tilemap<'a>,
}

impl<'a> Level<'a> {
	pub fn new() -> Level<'a> {
		let num_tiles =  Vector2::new(20., 20.);
		let tile_dim =  Vector2::new(32., 32.);

		let player_spawn_pos = Vector2::new(300., 400.);

		Level {
			active: true,
			player: Player::new(player_spawn_pos),
			tilemap: Tilemap::new(num_tiles, tile_dim)
		}
	}

	pub fn queue_renderers(&mut self, render_queue: &mut RenderQueue){
		self.player.push_to_queue(render_queue);
		render_queue.push(&mut self.tilemap);
	}

	pub fn update(&mut self, dt: f32) -> LevelState { 
		self.player.update(dt);
		NoChange
	}

}
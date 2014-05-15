use engine::math::{Vector2, Angle};
use engine::rendering::{EngineDrawable};
use std::vec::Vec;

use game::colors;

use rsfml;
use rsfml::system::vector2::Vector2f;
use rsfml::graphics::RenderWindow;
use rsfml::traits::drawable::Drawable;
use engine::matrix::Mat3;


#[deriving(Eq)]
enum TileType {
	Empty,
	Full,
}

pub struct Tilemap<'a> {
	tiles: Vec<TileType>,
	tile_dim: Vector2,
	num_tiles: Vector2,
	rect: rsfml::graphics::RectangleShape<'a>,
}

impl<'a> Tilemap<'a> {
	pub fn new(num_tiles: Vector2, tile_dim: Vector2) -> Tilemap<'a> {
		let total_tiles = (num_tiles.x * num_tiles.y) as uint;

		let mut tiles = Vec::with_capacity(total_tiles);
		for i in range(0, total_tiles) {
			tiles.push(Full);
		}

		Tilemap { 
			tiles: tiles, 
			tile_dim: tile_dim,
			num_tiles: num_tiles,
			rect: rsfml::graphics::RectangleShape::new_init(&tile_dim.to_sfml_f()).unwrap()
		}
	}
	
	pub fn init_tilemap(&mut self, tilemap: Vec<TileType>) {
		self.tiles = tilemap;
	}

	pub fn can_occupy(&self, game_pos: Vector2) -> bool {
		let tile_pos_x = game_pos.x as uint / self.tile_dim.x as uint;
		let tile_pos_y = game_pos.y as uint / self.tile_dim.y as uint;

		self.get_tile2f(tile_pos_x, tile_pos_y) ==  Empty
	}

	pub fn get_tile(&self, tile_pos: Vector2) -> TileType {
		self.get_tile2f(tile_pos.x as uint, tile_pos.y as uint)

	}

	pub fn get_tile2f(&self, tile_pos_x: uint, tile_pos_y: uint) -> TileType {
		assert!(tile_pos_x > 0 && tile_pos_x < self.num_tiles.x as uint);
		assert!(tile_pos_y > 0 && tile_pos_y < self.num_tiles.y as uint);

		*self.tiles.get(tile_pos_x + tile_pos_y * self.num_tiles.x as uint)
	}


}


impl<'a> EngineDrawable for Tilemap<'a> {
	fn Draw(&mut self, render_window: &mut RenderWindow) {
		let mut tile_x = 0;
		let mut tile_y = 0;

		for tile in self.tiles.iter() {
			let render_pos = Vector2::new(tile_x as f32 * self.tile_dim.x, tile_y as f32 * self.tile_dim.y);

			self.rect.set_fill_color(&colors::black);
			self.rect.set_position(&render_pos.to_sfml_f());
			self.rect.draw_in_render_window(render_window);

			tile_x = tile_x + 1;
			
			if tile_x == self.num_tiles.x as uint {
				tile_y = tile_y + 1;
				tile_x = 0	
			}

		}
	}
}
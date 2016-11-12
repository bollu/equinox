use engine::math::{Vector2, Angle};
use heart::newtonian::Motion;

use game::colors;
use sfml::graphics::CircleShape;
use engine::rendering::RenderQueue;

pub struct Player<'a> {
	representation: PlayerRepr,
	renderer: PlayerRenderer<'a>,
}

impl<'a> Player<'a> {
	pub fn new(pos: Vector2) -> Player<'a> {
		Player { representation: PlayerRepr::new(pos), renderer: PlayerRenderer::new() }
	}

	pub fn update(&mut self, dt: f32) {
		self.representation.update(dt);
		self.renderer.update(&self.representation, dt);
	}

	pub fn push_to_queue(&self, render_queue: &mut RenderQueue) {
		render_queue.push(&self.renderer.render);
	}
}


enum Facing {
	Left,
	Right,
	Top,
}

struct PlayerRepr {
	pos: Motion,

	jump_enabled: bool,
	jumping: bool,
}

impl PlayerRepr {
	pub fn new(pos: Vector2) -> PlayerRepr{
		let mut pos = Motion::new(1.0, pos, Vector2::new(300., -500.), Vector2::new(0.,1000.));

		PlayerRepr{ 
			pos: pos, 
			jump_enabled: true, 
			jumping: false 
		}
	}

	pub fn update(&mut self, dt: f32){
		self.pos.update(dt);
	}

	pub fn get_position(&self) -> Vector2 {
		self.pos.get_s()
	}
}


struct PlayerRenderer<'a> {
	render: CircleShape<'a>,
}

impl<'a> PlayerRenderer<'a> {
	pub fn new() -> PlayerRenderer<'a> {
		let mut render = CircleShape::new_init(32., 30).unwrap();
		render.set_fill_color(&colors::blue);
		PlayerRenderer { render: render }
	} 

	pub fn update(&mut self, repr: &PlayerRepr, dt: f32) {
		self.render.set_position(&repr.get_position().to_sfml_f())
	}
}

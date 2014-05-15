use engine::math::Vector2;
use std::num::abs;

//(0, 0) is bottom left
pub struct AABB {
	center: Vector2,
	halfdim: Vector2,
}


impl AABB {
	pub fn new(center: Vector2, halfdim: Vector2) -> AABB {
		AABB { center: center, halfdim: halfdim }
	}

	pub fn center(&self) -> Vector2 {
		self.center
	}

	pub fn halfdim(&self) -> Vector2 {
		self.halfdim
	}

	pub fn top_left(&self) -> Vector2 {
		Vector2::new(self.center.x - self.halfdim.x, self.center.y + self.halfdim.y)
	}

	pub fn bottom_right(&self) -> Vector2 {
		Vector2::new(self.center.x + self.halfdim.x, self.center.y - self.halfdim.y)
	}

	pub fn contains(&self, point: Vector2) -> bool {
		let top_left = self.top_left();
		let bottom_right = self.bottom_right();

		point.x > top_left.x && point.x < bottom_right.x && point.y > bottom_right.y && point.y < top_left.y

	}

	pub fn intersects(&self, other: &AABB) -> bool {
		let other_center = other.center();
		let other_halfdim = other.halfdim();

		abs(other_center.x - self.center.x) < self.halfdim.x + other_halfdim.x &&
		abs(other_center.y - self.center.y)< self.halfdim.y + other_halfdim.y
	} 
}
use engine::math::Vector2;

pub struct Motion {
	m: f32,
	s: Vector2,
	v: Vector2,
	a: Vector2,
}

impl Motion {
	pub fn new(m: f32, s: Vector2, v: Vector2, a: Vector2) -> Motion{
		Motion {m:m, s:s, v:v, a:a}
	}

	pub fn update(&mut self, dt: f32) {
		//Euler. should switch to RK
		self.v = self.v + self.a * dt;
		self.s = self.s + self.v * dt + self.a * dt * dt * 0.5;
	}

	pub fn get_s(&self) -> Vector2 {
		self.s
	}

	pub fn set_a(&mut self, a: Vector2) {
		self.a = a;
	}
}
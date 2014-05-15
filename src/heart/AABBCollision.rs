use engine::aabb::AABB;
use std::vec::Vec;

struct Collider {
	aabb: AABB,
	collided: bool,
}

impl Collider {
	fn new (aabb: AABB) {
		return Collider {aabb: AABB, collided: false}
	}
}

struct <'a>CollisionDetection {
	colliders: Vec<&'a Collider>,
	
}
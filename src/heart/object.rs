use heart::common_components::{Facing, Position};
use std::option::{Option, None};
use heart::UUID;

pub struct Object {
	uniq_id: UUID,
	facing:  Option<Facing>,
	position: Option<Position>,
}

impl Object {
	pub fn new(uniq_id : UUID) -> Object {
		Object { 
			uniq_id: uniq_id, 
			facing : None,
			position: None,
		}
	}
}
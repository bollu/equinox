use Components::{Position, UID};

use std::hashmap::HashMap;

pub struct ComponentBag {
	positions: HashMap<UID, Position>,
	//renderers: HashMap<UID, Renderer>
}

impl ComponentBag {
	pub fn removeObject(&mut self, tag: UID) {
		self.positions.pop(&tag);
		//self.renderers.pop(&tag);
	}
}



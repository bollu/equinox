use heart::common_components::{Position, Facing};
use std::hashmap::HashMap;

pub type UniqID = uint;

pub struct SimulationState {
	positions: HashMap<UniqID, Position>,
	facings: HashMap<UniqID, Facing>,
}

impl SimulationState {
	pub fn new() -> SimulationState {
		SimulationState { positions: HashMap::new(),
			facings: HashMap::new()
		}
	}
}

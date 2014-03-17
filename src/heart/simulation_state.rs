use heart::common_components::{Position, Facing};

#[deriving(Clone)]
pub struct SimulationState {
	positions: ~[Position],
	facings: ~[Facing],
}

impl SimulationState {
	pub fn new() -> SimulationState {
		SimulationState { 
			positions: ~[],
			facings: ~[]
		}
	}
}

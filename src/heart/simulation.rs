use engine::double_buffer::DoubleBuffer;
use engine::rendering::RenderQueue;

use heart::simulation_state::SimulationState;
use heart::object::Object;
use std::hashmap::HashMap;
use heart::UUID;

pub struct Simulation<'a> {
	objects: HashMap<UUID, Object>,
	simulation: DoubleBuffer<SimulationState>,
}

impl<'a> Simulation<'a> {
	pub fn new() -> Simulation<'a> {
		let mut simulation = DoubleBuffer::new( SimulationState::new(), SimulationState::new()); 
			Simulation { 
				simulation: simuation,
				objects: HashMap::new()
			}
	}
	
	pub fn tick(&mut self, dt: float) {
	}

	pub fn render(&mut self, render_queue: &mut RenderQueue) {

	}

	pub fn swap(&mut self) {
		self.simulation.swap();
	}
}
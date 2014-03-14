use engine::double_buffer::DoubleBuffer;
use heart::simulation_state::SimulationState;


struct Simulation {
	simulation: DoubleBuffer<SimulationState>,
}

pub fn new() -> Simulation{
	Simulation { simulation: DoubleBuffer::new( SimulationState::new(), SimulationState::new() )}
}
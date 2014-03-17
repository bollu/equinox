use engine::state::{State, EngineState, NoChange, StateTransition, EngineShutdown};
use engine::resource_loader::{ResourceLoader};
use engine::rendering::{RenderContext, RenderQueue};
use engine::event_queue::{EventQueue, EventHandler};

use heart::object::Object;
use heart::simulation_state::SimulationState;
use heart::simulation::Simulation;

use game::colors;

pub struct GameState {
	active: bool,
	simulation: Simulation,
}

impl GameState {
	pub fn new(loader: &ResourceLoader, render_ctx: &RenderContext) -> GameState {
		GameState { 
			active: true,
			simulation: Simulation::new()
		}
	}
}

impl State for GameState {
	fn queue_event_handlers(&mut self, event_queue: &mut EventQueue){}
	fn queue_renderers(&mut self, render_queue: &mut RenderQueue){
		render_queue.set_clear_color(colors::blue);
	}
	
	fn tick(&mut self, dt: f32) -> EngineState { 
		simulation.tick(dt);
	}
}
use engine::state::{State, EngineState, NoChange, StateTransition, EngineShutdown};
use engine::resource_loader::{ResourceLoader};
use engine::rendering::{RenderContext, RenderQueue};
use engine::event_queue::{EventQueue, EventHandler};

use game::colors;

pub struct GameState {
	active: bool,
}

impl GameState {
	pub fn new(loader: &ResourceLoader, render_ctx: &RenderContext) -> GameState {
		GameState { active: true}
	}
}

impl State for GameState {
	fn queue_event_handlers(&mut self, event_queue: &mut EventQueue){}
	fn queue_renderers(&mut self, render_queue: &mut RenderQueue){
		render_queue.set_clear_color(colors::blue);
	}
	fn tick(&mut self, dt: f32) -> EngineState {  NoChange }
}
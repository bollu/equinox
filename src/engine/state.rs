use std::hashmap::HashMap;
use engine::rendering::RenderQueue;
use engine::event_queue::EventQueue;

type StateId = int;

#[deriving(Eq)]
pub enum EngineState {
	StateTransition(StateId), //UID of state
	EngineShutdown, //shut down and close
	NoChange
}

pub trait State {
	//TODO: make these opposites...
	fn queue_event_handlers(&mut self, queue: &mut EventQueue);
	fn queue_renderers(&mut self, queue: &mut RenderQueue);

	fn Tick(&mut self, dt: f32) -> EngineState;
}

struct TransitionInfo {
	transitioning: bool,
	next_state_id: StateId,
}

pub struct StateMachine {
	states: HashMap<StateId, ~State:>,
	current_id: StateId,
	engine_state: EngineState
}

impl TransitionInfo {

}

impl StateMachine {
	pub fn uninitialized() -> StateMachine {
		StateMachine { states: HashMap::new(), current_id: 0, engine_state: NoChange  }
	}
	
	pub fn set_default_state(&mut self, id: StateId) {
		self.current_id = id;
	}

	pub fn add_state(&mut self, id: StateId, state: ~State:) {
		self.states.insert(id, state);
	}

	pub fn initialize(&mut self, event_queue: &mut EventQueue, render_queue: &mut RenderQueue) {
		let state = self.get_current_state();
		state.queue_event_handlers(event_queue);
		state.queue_renderers(render_queue);

	}


	pub fn Tick(&mut self, dt: f32, event_queue: &mut EventQueue, render_queue: &mut RenderQueue) -> EngineState {
		self.engine_state = {
			let current_state = self.get_current_state();

			current_state.queue_event_handlers(event_queue);
			current_state.queue_renderers(render_queue);

			current_state.Tick(dt)
		};
		 
		
		//TODO - transition!
		match self.engine_state {
			StateTransition(id) => {},
			_ => {} 
		}

		return self.engine_state
	}

	fn get_current_state<'a>(&'a mut self) -> &'a mut ~State: {
		return self.states.get_mut(&self.current_id)	
	}
}
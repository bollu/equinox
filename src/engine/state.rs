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

	fn startup(&mut self) { }
	fn shutdown(&mut self) { }

	fn tick(&mut self, dt: f32) -> EngineState;
}

struct TransitionInfo {
	transitioning: bool,
	next_state_id: StateId,
}

pub struct StateMachine {
	states: HashMap<StateId, ~State:>,
	current_id: StateId,
	transition_info: TransitionInfo,

}

impl TransitionInfo {
	pub fn new() -> TransitionInfo {
		TransitionInfo { transitioning: false, next_state_id: 0 }
	}
}

impl StateMachine {
	pub fn uninitialized() -> StateMachine {
		StateMachine { states: HashMap::new(), current_id: 0, transition_info: TransitionInfo::new() }
	}
	
	pub fn set_default_state(&mut self, id: StateId) {
		self.current_id = id;
	}

	pub fn add_state(&mut self, id: StateId, state: ~State:) {
		self.states.insert(id, state);
	}

	pub fn tick(&mut self, dt: f32, event_queue: &mut EventQueue, render_queue: &mut RenderQueue) -> EngineState {
		
		if self.transition_info.transitioning {
			//ask current state to shutdown
			self.get_current_state().shutdown();

			self.current_id = self.transition_info.next_state_id;
			self.transition_info.transitioning = false;
			
			//now that the current_id has changed, let's launch the next state
			self.get_current_state().startup();

		}

		let engine_state = {
			let current_state = self.get_current_state();

			current_state.queue_event_handlers(event_queue);
			current_state.queue_renderers(render_queue);

			current_state.tick(dt)
		};
		 
		
		match engine_state {
			StateTransition(id) => {
				self.transition_info.transitioning = true;
				self.transition_info.next_state_id = id; 
			},
			_ => {} 
		}

		return engine_state
	}

	fn get_current_state<'a>(&'a mut self) -> &'a mut ~State: {
		return self.states.get_mut(&self.current_id)	
	}
}
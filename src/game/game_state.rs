use engine::state::{State, EngineState, NoChange, EngineShutdown, StateData, IntStateData};
use engine::resource_loader::{ResourceLoader};
use engine::settings::Settings;
use engine::rendering::{RenderContext, RenderQueue};
use engine::event_queue::{EventQueue};


use game::colors;
use game::level::Level;

use heart::game_event_layer::GameEventLayer;

pub struct GameState<'a> {
	active: bool,
	level: Level<'a>,
	event_layer: GameEventLayer,
}

impl<'a> GameState<'a> {
	pub fn new(loader: &ResourceLoader, _render_ctx: &RenderContext, settings: &Settings) -> GameState<'a> {
		GameState { 
			active: true,
			level: Level::new(),
			event_layer: GameEventLayer::new(settings),
		}
	}

}

impl<'a> State for GameState<'a> {
	fn startup(&mut self, data: StateData) { 
		let value = match data {
			IntStateData(i) => i,
			_ => panic!()
		};
	}

	fn queue_event_handlers(&mut self, event_queue: &mut EventQueue){
		event_queue.push(&mut self.event_layer);
	}
	
	fn queue_renderers(&mut self, render_queue: &mut RenderQueue){
		render_queue.set_clear_color(colors::yellow);
		self.level.queue_renderers(render_queue);
	}
	
	fn tick(&mut self, dt: f32) -> EngineState { 
		self.level.update(dt);

		return if self.event_layer.should_quit() {
			 EngineShutdown
		} else {
			NoChange
		}
		
	}
}

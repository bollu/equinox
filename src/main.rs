extern crate num;
extern crate native;
extern crate rsfml;
extern crate collections;

use rsfml::window::{event};
use rsfml::graphics::{CircleShape, RectangleShape, Color, Font};
use rsfml::system::vector2::Vector2f;

use engine::world;
use engine::settings;
use engine::rendering;
use engine::event_queue;
use engine::state;
use engine::state::EngineShutdown;
use engine::math;
use engine::resource_loader::{ResourceLoader, Resource};

pub mod engine;
pub mod game;


fn main () -> () {
    let settings = engine::settings::Settings::new(~"settings");
    let mut window =  engine::rendering::Window::new(800, 600, ~"equinox", false);
    let mut event_queue = engine::event_queue::EventQueue::new();
    let mut render_queue = engine::rendering::RenderQueue::new();
    let mut resource_loader = engine::resource_loader::ResourceLoader::new();
    let mut state_machine = engine::state::StateMachine::uninitialized();

    //load all resources required by the game
    game::load_resources(&mut resource_loader);

    //create all states
    let render_context = window.get_context(); 
    game::init_states(&mut state_machine, &resource_loader, &render_context);
    
    //init the state machine and kick the first state into action!
    state_machine.initialize(&mut event_queue, &mut render_queue);

    while window.is_open() {
        loop {
            let event = window.poll();

            match event {
                event::Closed => window.close(),
                
                event::NoEvent => break,
                _ => { event_queue.send_event(&event) }
            }
        }

        let engine_state =  state_machine.Tick(1.0 / 60.0, &mut event_queue, &mut render_queue);

        match engine_state {
            EngineShutdown => { window.close() }
            _ => {}
        }

        render_queue.render(&mut window);
        window.display();
    }


}

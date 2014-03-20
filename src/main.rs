extern crate serialize;
extern crate num;
extern crate native;
extern crate rsfml;
extern crate collections;

#[feature(phase)] extern crate log;
#[phase(syntax, link)]



use rsfml::window::{event};

use engine::resource_loader;
//use engine::world;
use engine::settings;
use engine::rendering;
use engine::event_queue;
use engine::state;
use engine::state::EngineShutdown;

//use engine::math;

pub mod engine;
pub mod game;
pub mod heart;

fn main () -> () {
    let mut settings = settings::Settings::new(~"settings");
    let mut window =  rendering::Window::new(800, 600, ~"equinox", false);
    let mut event_queue = event_queue::EventQueue::new();
    let mut render_queue = rendering::RenderQueue::new();
    let mut resource_loader = resource_loader::ResourceLoader::new();
    let mut state_machine = state::StateMachine::uninitialized();

    //load all resources required by the game
    game::load_resources(&mut resource_loader);

    //create all states
    let render_context = window.get_context(); 
    game::init_states(&mut state_machine, &resource_loader, &render_context, &settings);
    
    while window.is_open() {
        loop {
            let event = window.poll();

            match event {
                //HACK - this should be removed..
                //event::Closed => window.close(),
                event::NoEvent => break,

                _ => { event_queue.send_event(&event) }
            }
        }

        let engine_state =  state_machine.tick(1.0 / 60.0, &mut event_queue, &mut render_queue);

        match engine_state {
            EngineShutdown => { window.close() }
            _ => {}
        }

        render_queue.render(&mut window);
        window.display();
    }


}

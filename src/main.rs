extern crate rustc_serialize;
extern crate num;
// Couldn't find anything using this so ¯\_(ツ)_/¯
//extern crate native;
// This is grabbed from a .so file. When googling "librsfml" I literally
// got 6 results, 1 from this project. It seems to have been renamed
// so let's try to use that instead even if it looks like it will break
// many things.
//extern crate rsfml;
extern crate sfml;

// idk wth these lines were used for, couldn't find any other references
// and the compiler complained. If the code doesn't use it maybe the
// compiler does?
//#[feature(phase)]extern crate log;
//#[phase(syntax, link)]

use engine::resource_loader;
//use engine::world;
use engine::settings;
use engine::rendering;
use engine::event_queue;
use engine::state;
use engine::state::EngineState::EngineShutdown;

use std::collections;

//use engine::math;

pub mod engine;
pub mod game;
pub mod heart;

fn main () -> () {
    let mut settings = settings::Settings::new("settings".to_string());
    let mut window =  rendering::Window::new(800, 600, "equinox".to_string(), false);
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
                
                // I'm not sure what the above comment is talking
                // about, but NoEvent is removed in favor of options
                // https://github.com/jeremyletang/rust-sfml/commit/a0649ba936879041619e2ab9b3c569bdd16c0851
                // Now this never breaks?
                //event::NoEvent => break,

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

extern crate num;
extern crate native;
extern crate rsfml;

use rsfml::window::{event};
use rsfml::graphics::{CircleShape, RectangleShape, Color, Font};
use rsfml::system::vector2::Vector2f;

use engine::world;
use engine::settings;
use engine::rendering;
use engine::event_pump;
use engine::state;
use engine::math;
use engine::resource_loader::{ResourceLoader, Resource};


pub mod engine;
pub mod game;


fn main () -> () {
    let settings = engine::settings::Settings::new(~"settings");
    let mut window =  engine::rendering::Window::new(800, 600, ~"equinox", false);
    let mut event_pump = engine::event_pump::EventPump::new();
    let mut render_pump = engine::rendering::RenderPump::new();
    let mut resource_loader = engine::resource_loader::ResourceLoader::new();
    let mut state_machine = engine::state::StateMachine::uninitialized();

    //load all resources required by the game
    game::load_resources(&mut resource_loader);

    //create all states
    let render_context = window.get_context(); 
    game::init_states(&mut state_machine, &resource_loader, &render_context);
    
    //init the state machine and kick the first state into action!
    state_machine.initialize(&mut event_pump, &mut render_pump);

    /*
    let mut colors : ~[Color] = ~[];
    colors.push(game::colors::black);
    colors.push(game::colors::gray);
    colors.push(game::colors::navy);
    colors.push(game::colors::blue);
    colors.push(game::colors::orange);
    colors.push(game::colors::red);
    colors.push(game::colors::green);
    colors.push(game::colors::dark_green);
    colors.push(game::colors::yellow);
    
    let mut i = 0;
    for color in colors.iter() {
        let width = (window.width() / colors.len()) as f32;
        let height = (window.height() as f32) as f32;

        let dim = Vector2f::new(width, height);
        let mut rect = ~RectangleShape::new_init(&dim).unwrap();
       
        //not sure if the math checks out. too lazy to check it.
        rect.set_position2f(i as  f32 * width, 0.);
        rect.set_fill_color(color);
        render_pump.attach(rect);
       
        i += 1;
        
    }*/

    while window.is_open() {
        loop {
            let event = window.poll();

            match event {
                event::Closed => window.close(),
                event::NoEvent => break,
                _ => { event_pump.pump(&event) }
            }
        }

        render_pump.pump(&mut window);
        window.display();
    }

   
}

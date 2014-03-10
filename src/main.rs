extern crate num;
extern crate native;
extern crate rsfml;


use rsfml::window::{event};
use rsfml::graphics::{CircleShape, RectangleShape, Color, Font};
use rsfml::system::vector2::Vector2f;
use resource_loader::{ResourceLoader, Resource};

mod world;
mod Components;
mod settings;
mod rendering;
mod event_pump;
mod state;
mod math;
mod resource_loader;
mod game;

fn main () -> () {
    let settings = settings::Settings::new(~"settings");
    let mut window =  rendering::Window::new(800, 600, ~"equinox", false);
    let mut eventPump = event_pump::EventPump::new();
    let mut renderPump = rendering::RenderPump::new();
    let mut resourceLoader = resource_loader::ResourceLoader::new();

    //load all resources required by the game
    load_resources(&mut resourceLoader);

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
        renderPump.attach(rect);
       
        i += 1;
        
    }

    while window.is_open() {
        loop {
            let event = window.poll();

            match event {
                event::Closed => window.close(),
                event::NoEvent => break,
                _ => { eventPump.pump(&event) }
            }
        }

        window.clear();
        renderPump.pump(&mut window);
        window.display();
    }

   
}

fn load_resources(loader: &mut ResourceLoader) {
    let obelix_font = Font::new_from_file("res/font/Obelix.ttf").unwrap();
    loader.addResource(~"MenuFont", resource_loader::Font(obelix_font));
}

extern crate native;
extern crate rsfml;


use rsfml::system::Vector2f;
use rsfml::window::{ContextSettings, VideoMode, event, Close};
use rsfml::graphics::{RenderWindow, CircleShape, Color};

mod World;
mod Components;
mod Init;



#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}



fn main () ->   () {
    let settings = Init::Settings::new(~"settings");
    

    let setting = ContextSettings::default();
    let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32), "SFML Example", Close, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };

    // Create a CircleShape
    let mut circle = match CircleShape::new() {
        Some(circle)    => circle,
        None()          => fail!("Error, cannot create ball")
    };
    circle.set_radius(30.);
    circle.set_fill_color(&Color::red());
    circle.set_position(&Vector2f::new(100., 100.));


    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed => window.close(),
                event::NoEvent => break,
                _ => {}
            }
        }

        // Clear the window
        window.clear(&Color::new_RGB(0, 200, 200));
        // Draw the shape
           window.draw(&circle);
        // Display things on screen
        window.display()
    }
}
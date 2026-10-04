extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video()
        .expect("Couldn't get SDL video subsystem");

    let window = video_subsystem.window("Tetris", 800, 600)
        .position_centered() // puts it in the center of the screen
        .build()
        .expect("Failed to create window");
    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
        // To make things easier to read, creating a constant which will be the texture's size
        const TEXTURE_SIZE: u32 = 32;

    let mut square_texture: Texture = 
        texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
        .expect("Failed to create a texture");

    // Using the canvas to draw into square texture
    canvas.with_texture_canvas(&mut square_texture, |texture|  {
        texture.set_draw_color(Color::RGB(0, 255, 0));
        // Texture "cleared" so it'll be filled with green
        texture.clear();
    }).expect("Failed to color a texture");

    // First we get the event handler:
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    // Then an infinite loop is created to loop over events:
    'running: loop {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                break 'running;
            }
            _ => {}
        }
    }

    // whatever drawing code you have here...

    canvas.present();

    sleep(Duration::new(0, 1_000_000_000u32 / 60));
}

}

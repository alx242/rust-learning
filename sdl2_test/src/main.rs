extern crate sdl2;

use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
// use sdl2::gfx::primitives::DrawRenderer;
// use sdl2::gfx::primitives::ToColor;
// use std::time::Duration;
use std::{thread, time};

pub fn main() {
    // Init the SDL stuff
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create a window
    let window = video_subsystem.window("Helo!", 1024, 768)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let ten_millis = time::Duration::from_millis(10);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    // Named loop to succesfully break it
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 0, 255));
        canvas.draw_line(Point::new(10, 10),
                         Point::new(100, 100));

        // Need to pass by the window events or the window wont be
        // displayed at all
        for event in event_pump.poll_iter() {
            match event {
                Event::Window { win_event: WindowEvent::Shown, .. } => {
                    println!("Displaying the window, important");
                }
                Event::Window { win_event: SomeOtherEvent, .. } => {
                    println!("Window event: {:?}", SomeOtherEvent);
                }
                Event::Quit {..} => {
                    println!("Quit using quit button");
                    break 'running
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("Quit using escape key!");
                    break 'running
                },
                _ => {
                    // println!("Unknown event: {:?}", event);
                }
            }
        }
        // More crap goes here...

        canvas.present();
        thread::sleep(ten_millis);
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

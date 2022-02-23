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
use libm::{sin, cos};

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
    let mut angle = 0.0;
    let angle_step = 0.01;
    const PI:f64 = 3.14;
    // Named loop to succesfully break it
    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        // Calculate sinus position and cast to i32 for drawing the
        // funky line
        angle = (angle + angle_step) % (2.0*PI);
        let pos_x = (200.0*libm::cos(angle)) as i32 + 300;
        let pos_y = (200.0*libm::sin(angle)) as i32 + 300;
        println!("pos_x: {} pos_y: {}", pos_x, pos_y);
        canvas.set_draw_color(Color::RGB(255, 0, 255));
        let aa:i32 = 10;
        let bb:i32 = 100;
        canvas.draw_line(Point::new(pos_x, pos_y),
                         Point::new(pos_x+300, pos_y+300));

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

extern crate sdl2;

use sdl2::{pixels::Color, event::Event, keyboard::Keycode};

mod complex;

static WIDTH: u32 = 1280;
static HEIGHT: u32 = 720;

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video.window("Mandelbrot", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    //  Initialize canvas and event pump
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = context.event_pump().unwrap();

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop;
                }
                _ => {}
            }

            canvas.present();
        }
    }
}

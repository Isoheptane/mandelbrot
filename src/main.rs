extern crate sdl2;

use sdl2::{pixels::Color, event::Event, keyboard::Keycode};

mod complex;
use complex::Complex;

enum TestResult {
    Converge,
    Diverge {
        iteration: u32,
        z: Complex,
        c: Complex
    },
}

fn mandelbrot_iterate(z: Complex, c: Complex) -> Complex {
   z * z + c 
}

fn mandelbrot_test(c: Complex) -> TestResult {
    static MAX_ITERATION: u32 = 32;
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITERATION {
        z = mandelbrot_iterate(z, c);
        if z.norm_sq() < 0.25 {
            return TestResult::Converge
        }
        if z.norm_sq() > 4.0 {
            return TestResult::Diverge { iteration: i, z, c };
        }
    }
    return TestResult::Converge;
}

fn colorize(result: TestResult) -> Color {
    let (i, z, c) = match result {
        TestResult::Converge => return Color::RGB(0, 0, 0),
        TestResult::Diverge { iteration, z, c } => (iteration, z, c)
    };
    let brightness = 1.0 - (i as f64 / 32 as f64);
    let brightness = (brightness * 255.0).round() as u8;
    Color::RGB(brightness, brightness, brightness)
}

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

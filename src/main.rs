extern crate sdl2;

use sdl2::{pixels::Color, event::Event, keyboard::Keycode, rect::Point};

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
   (z * z) + c 
}

fn mandelbrot_test(c: Complex) -> TestResult {
    static MAX_ITERATION: u32 = 256;
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITERATION {
        z = mandelbrot_iterate(z, c);
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

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

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

    let mut center = Complex::new(0.0, 0.0);
    let mut scale = 0.002;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop;
                }
                _ => {}
            }
        }
        const CORE_COUNT: u32 = 16;
        const BLOCK_HEIGHT: usize = (HEIGHT / CORE_COUNT) as usize;
        const BLOCK_SIZE: usize = BLOCK_HEIGHT * WIDTH as usize;
        let mut threads = vec![];
        for i in 0..CORE_COUNT {
            let handle = std::thread::spawn(move || -> (i32, Vec<Color>) {
                let mut array: Vec<Color> = vec![];
                array.reserve(BLOCK_SIZE);
                let start_y = i as i32 * BLOCK_HEIGHT as i32;
                for y in start_y..(start_y + BLOCK_HEIGHT as i32) {
                    for x in 0..WIDTH as i32 {
                        let c = Complex::new((x - WIDTH as i32 / 2) as f64 * scale, (y - HEIGHT as i32 / 2) as f64 * scale);
                        array.push(colorize(mandelbrot_test(c)));
                    }
                }
                return (start_y, array);
            });
            threads.push(handle);
        }
        for thread in threads {
            let (y, array) = thread.join().unwrap_or((0, vec![]));
            let mut counter: i32 = 0;
            for color in array {
                canvas.set_draw_color(color);
                canvas.draw_point(Point::new(counter % WIDTH as i32, y + counter / WIDTH as i32)).unwrap();
                counter += 1;
            }
        }
        println!("Render finished");
        canvas.present();
    }
}

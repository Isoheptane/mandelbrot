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
    static MAX_ITERATION: u32 = 128;
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITERATION {
        z = mandelbrot_iterate(z, c);
        if z.norm_sq() > 4.0 {
            return TestResult::Diverge { iteration: i, z, c };
        }
    }
    return TestResult::Converge;
}

fn hsv_to_rgb(hsv: (f32, f32, f32)) -> Color {
    let h = (hsv.0 % 360.0 + 360.0) % 360.0;
    let s = hsv.1;
    let v = hsv.2;
    let r = {
        let distance = f32::min((h - 0.0).abs(), (h - 360.0).abs());
        let weight = ((120.0 - distance) / 60.0).clamp(0.0, 1.0);
        (1.0 - (1.0 - weight) * s) * v
    };
    let g = {
        let distance = (h - 120.0).abs();
        let weight = ((120.0 - distance) / 60.0).clamp(0.0, 1.0);
        (1.0 - (1.0 - weight) * s) * v
    };
    let b = {
        let distance = (h - 240.0).abs();
        let weight = ((120.0 - distance) / 60.0).clamp(0.0, 1.0);
        (1.0 - (1.0 - weight) * s) * v
    };
    Color::RGB((r * 255.0).round() as u8, (g * 255.0).round() as u8, (b * 255.0).round() as u8)
}

fn colorize(result: TestResult) -> Color {
    let (i, _, _) = match result {
        TestResult::Converge => return Color::RGB(255, 255, 255),
        TestResult::Diverge { iteration, z, c } => (iteration, z, c)
    };
    let brightness = i as f32 / 128.0;
    return hsv_to_rgb((240.0 - brightness * 120.0, 1.0 - brightness.powi(3), brightness));
}

const WIDTH: u32 = 2100;
const HEIGHT: u32 = 900;

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
                },
                Event::MouseMotion { mousestate, xrel, yrel , .. } => {
                    if mousestate.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
                        center.real -= xrel as f64 * scale;
                        center.imag -= yrel as f64 * scale;
                    }
                },
                Event::MouseWheel { y, .. } => {
                    scale *= (1.2 as f64).powi(-y);
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let start_time = std::time::SystemTime::now();
        const CORE_COUNT: u32 = 24;
        
        let mut threads = vec![];
        for i in 0..CORE_COUNT {
            let start_y = (HEIGHT as f32 / CORE_COUNT as f32 * i as f32).round() as usize;
            let end_y = (HEIGHT as f32 / CORE_COUNT as f32 * (i + 1) as f32).round() as usize;
            let handle = std::thread::spawn(move || -> (i32, Vec<Color>) {
                let mut array: Vec<Color> = vec![];
                array.reserve((end_y - start_y) * WIDTH as usize);
                for y in start_y..end_y {
                    for x in 0..WIDTH as i32 {
                        let c = center + Complex::new((x - WIDTH as i32 / 2) as f64 * scale, (y as i32 - HEIGHT as i32 / 2) as f64 * scale);
                        array.push(colorize(mandelbrot_test(c)));
                    }
                }
                return (start_y as i32, array);
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
        let stop_time = std::time::SystemTime::now();
        let ms_elapsed = stop_time.duration_since(start_time).unwrap().as_nanos() as f32 / 1_000_000.0;
        println!("Render finished in {:.6} ms", ms_elapsed);
        canvas.present();
    }
}

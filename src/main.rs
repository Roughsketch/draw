#![feature(inclusive_range_syntax)]

extern crate rand;
extern crate sdl2;

use rand::distributions::{IndependentSample, Range};
use rand::Rng;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum::RGBA8888;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 256;
const ITEMS: usize = 25;
const MIN_RAD: i16 = 10;
const MAX_RAD: i16 = WIDTH as i16 / 20;
const MIN_SIDES: i16 = 3;
const MAX_SIDES: i16 = 6;

fn main() {
    let sdl_context = sdl2::init().expect("Could not initialize context.");
    let _image_context = sdl2::image::init(INIT_PNG).unwrap();
    let video = sdl_context.video().expect("Could not get video context.");
    let window = video.window("Draw", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not build window.");

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("shelterfrog_512.png").unwrap();

    canvas.copy(&texture, None, Rect::new(0, 0, WIDTH / 2, HEIGHT)).unwrap();

    canvas.present();

    let x_range = Range::new((WIDTH / 2) as i16, WIDTH as i16);
    let y_range = Range::new(0, HEIGHT as i16);
    let r_range = Range::new(MIN_RAD, MAX_RAD);
    let s_range = Range::new(MIN_SIDES, MAX_SIDES);
    let first = canvas.read_pixels(Rect::new(0, 0, WIDTH / 2, HEIGHT), RGBA8888).unwrap();
    let mut polygons = Vec::new();
    let mut last_fitness = std::f64::MAX;
    let mut rng = rand::thread_rng();
    let mut frame = 0usize;


    for _ in 0..ITEMS {
        let color = Color::RGBA(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>());

        let mut x = Vec::new();
        let mut y = Vec::new();

        for _ in 0..3 {
            x.push(x_range.ind_sample(&mut rng));
            y.push(y_range.ind_sample(&mut rng));
        }

        polygons.push((x, y, color));
    }

    'running: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                }
                Event::KeyDown {keycode: Some(Keycode::I), ..} => {
                    println!("Increasing");
                    last_fitness *= 1.5;
                }
                Event::KeyDown {keycode: Some(Keycode::F), ..} => {
                    println!("Fitness: {}", last_fitness);
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let mut new_polygons = polygons.clone();

        {
            let item = &mut new_polygons[frame % ITEMS];

            if rng.gen() {
                for i in 0..item.0.len() {
                    if rng.gen() {
                        item.0[i] = x_range.ind_sample(&mut rng);
                    }
                }
            }
            if rng.gen() {
                for i in 0..item.1.len() {
                    if rng.gen() {
                        item.1[i] = y_range.ind_sample(&mut rng);
                    }
                }
            }
            // if rng.gen() {
            //     let sides = s_range.ind_sample(&mut rng);

            //     while item.0.len() > sides as usize {
            //         item.0.pop();
            //         item.1.pop();
            //     }

            //     while item.0.len() < sides as usize {
            //         item.0.push(x_range.ind_sample(&mut rng));
            //         item.1.push(y_range.ind_sample(&mut rng));
            //     }
            // }
            if rng.gen() {
                if rng.gen() {
                    item.2.r = rng.gen::<u8>();
                }
                if rng.gen() {
                    item.2.g = rng.gen::<u8>();
                }
                if rng.gen() {
                    item.2.b = rng.gen::<u8>();
                }
                if rng.gen() {
                    item.2.a = rng.gen::<u8>();
                }
            }
        }

        for &(ref x, ref y, c) in new_polygons.iter() {
            let _ = canvas.filled_polygon(&x, &y, c);
        }

        let pixels = canvas.read_pixels(Rect::new((WIDTH / 2) as i32, 0, WIDTH / 2, HEIGHT), RGBA8888).unwrap();
        let fitness = fitness(&first, &pixels);

        if fitness < last_fitness {
            //println!("Fitness with first: {}", fitness);
            polygons = new_polygons;
            last_fitness = fitness;
            canvas.copy(&texture, None, Rect::new(0, 0, WIDTH / 2, HEIGHT)).unwrap();
            canvas.present();
        } else {
            last_fitness *= 1.00001;
        }
        frame += 1;
    }
}

fn fitness(original: &Vec<u8>, current: &Vec<u8>) -> f64 {
    original
        .iter()
        .zip(current.iter())
        .fold(0f64, |fitness, (a, b)| {
            let c = *a as f64 - *b as f64;
            fitness + c * c
        })
}
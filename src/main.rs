use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use rayon::prelude::*;

use ggez::{Context, GameResult};
use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod, MouseButton};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Font, MeshBuilder, Point2, Text};
use ggez::timer;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 256;
const ITEMS: usize = 100;
const MIN_RAD: i16 = 10;
const MAX_RAD: i16 = WIDTH as i16 / 20;

mod mainstate;
use mainstate::MainState;

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            _ => {}
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
    }
}

fn main() {
    let mut config = conf::Conf::new();
    config.window_mode.width = size;
    config.window_mode.height = size;
    config.window_mode.vsync = false;

    let ctx = &mut Context::load_from_conf("Reversi", "Roughsketch", config).unwrap();

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("assets");
        ctx.filesystem.mount(&path, true);
    }

    let mut game = match MainState::new(ctx, "face.png") {
        Ok(game) => game,
        Err(why) => {
            println!("Could not load MainState: {:?}", why);
            return;
        }
    };

    graphics::set_background_color(ctx, graphics::BLACK));
    let result = event::run(ctx, &mut game);

    if let Err(e) = result {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("face.png").unwrap();

    canvas.copy(&texture, None, Rect::new(0, 0, WIDTH / 2, HEIGHT)).unwrap();

    canvas.present();

    let x_range = Range::new((WIDTH / 2) as i16, WIDTH as i16);
    let y_range = Range::new(0, HEIGHT as i16);
    let r_range = Range::new(MIN_RAD, MAX_RAD);
    let first = canvas.read_pixels(Rect::new(0, 0, WIDTH / 2, HEIGHT), RGBA8888).unwrap();
    let mut polygons = Vec::new();
    let mut last_fitness = std::f64::MAX;
    let mut rng = rand::thread_rng();
    let mut frame = 0usize;


    for _ in 0..ITEMS {
        let color = Color::RGBA(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>());
        let _ = canvas.filled_circle(-MIN_RAD, -MIN_RAD, MIN_RAD, color);
        polygons.push((-MIN_RAD, -MIN_RAD, MIN_RAD, color));
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

            if rng.gen() || item.0 < 256 {
                item.0 = x_range.ind_sample(&mut rng);
            }
            if rng.gen() {
                item.1 = y_range.ind_sample(&mut rng);
            }
            if rng.gen() {
                item.2 = r_range.ind_sample(&mut rng);
            }
            if rng.gen() {
                if rng.gen() {
                    item.3.r = rng.gen::<u8>();
                }
                if rng.gen() {
                    item.3.g = rng.gen::<u8>();
                }
                if rng.gen() {
                    item.3.b = rng.gen::<u8>();
                }
                if rng.gen() {
                    item.3.a = rng.gen::<u8>();
                }
            }
        }

        for &(x, y, r, c) in new_polygons.iter() {
            let _ = canvas.filled_circle(x, y, r, c);
        }

        let pixels = canvas.read_pixels(Rect::new((WIDTH / 2) as i32, 0, WIDTH / 2, HEIGHT), RGBA8888).unwrap();
        let fitness = fitness(&first, &pixels);

        if fitness < last_fitness {
            //println!("Fitness with first: {}", fitness);
            polygons = new_polygons;
            last_fitness = fitness;
            if frame % ITEMS == 0 {
                canvas.copy(&texture, None, Rect::new(0, 0, WIDTH / 2, HEIGHT)).unwrap();
                canvas.present();
            }
        } else {
            last_fitness *= 1.00001;
        }
        frame += 1;
    }
}

fn fitness(original: &Vec<u8>, current: &Vec<u8>) -> f64 {
    original
        .par_iter()
        .zip(current.par_iter())
        .fold(|| 0f64, |fitness, (a, b)| {
            let c = *a as f64 - *b as f64;
            fitness + c * c
        }).sum()
}
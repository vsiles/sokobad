extern crate sdl2;
extern crate clap;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;

use clap::{Arg, App};

use std::path::Path;

const CELL_SIZE : u32 = 32;

mod game;
mod config;

fn main() {
    let matches = App::new("Sokobad")
        .version("1.0")
        .author("Vinz <vincent.siles@ens-lyon.org>")
        .about("Poor clone of Sokoban/Hinder written in Rust")
        .arg(Arg::with_name("map")
             .short("m")
             .long("map")
             .value_name("FILE")
             .help("Select the map to run")
             .takes_value(true)
             .default_value("data/maps/map0"))
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Configuration file (json)")
             .takes_value(true))
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("data/config.json");

    println!("Loading configuration: {}", config_path);
    let (keys, undo_level) = config::new(config_path).unwrap();

    let map_path = matches.value_of("map").unwrap(); /* has a default value */
    println!("Loading map: {}", map_path);

    let mut map = game::Map::new(map_path, CELL_SIZE, undo_level).unwrap();
    // map.dump();

    let sdl = sdl2::init().unwrap();
    let _sdl_image = sdl2::image::init(sdl2::image::INIT_PNG).unwrap();
    // let _sdl_ttf = sdl2::ttf::init().unwrap();

    let video_subsystem = sdl.video().unwrap();

    let window_width: u32 = (map.width as u32) * CELL_SIZE;
    let window_height: u32 = (map.height as u32) * CELL_SIZE;

    let window = video_subsystem
        .window("Sokoban", window_width, window_height)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas : sdl2::render::WindowCanvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .target_texture()
        .build()
        .unwrap();

    let mut timer_subsystem = sdl.timer().unwrap();

    let tex_creator = canvas.texture_creator();
    let tex = tex_creator.load_texture(Path::new("data/img/win.png")).unwrap();

    let mut events = sdl.event_pump().unwrap();
    'main: loop {
        let mut done = false;
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    break 'main,
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(key) => {
                            if key == keys.quit { break 'main }
                            else if key == keys.up {
                                done = map.update(game::Direction::Up)
                            } else if key == keys.down {
                                done = map.update(game::Direction::Down)
                            } else if key == keys.left {
                                done = map.update(game::Direction::Left)
                            } else if key == keys.right {
                                done = map.update(game::Direction::Right)
                            } else if key == keys.undo {
                                map.undo()
                            } else if key == keys.reset {
                                map.reset()
                            }
                        },
                        None => ()
                    }
                },
                _ => {},
            }
        }

        /* Render here */
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        map.render(&mut canvas);

        canvas.present();

        if done {
            let w2 = window_width / 2;
            let h2 = window_height / 2;
            let r = Rect::new((w2 - w2 / 2) as i32, (h2 - h2 / 2) as i32,
                              w2, h2);
            canvas.copy(&tex, None, Some(r)).unwrap();
            canvas.present();
            println!("Congratulations, you won !");
            timer_subsystem.delay(2000);
            std::process::exit(0);
        }

    }
}

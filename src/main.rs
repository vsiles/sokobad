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
mod record;

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
        .arg(Arg::with_name("record")
             .short("r")
             .long("rec")
             .value_name("FILE")
             .help("Save the run in the specified file")
             .takes_value(true)
             .default_value("/tmp/sokobad.run"))
        .arg(Arg::with_name("play")
             .short("p")
             .long("play")
             .value_name("FILE")
             .help("Run a saved file instead of interactive playing")
             .takes_value(true)
             .conflicts_with("record"))
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("data/config.json");

    println!("Loading configuration: {}", config_path);
    let game_conf = config::new(config_path).unwrap();
    let keys = &game_conf.keys;
    let undo_level = game_conf.undo_level;
    let speed = game_conf.replay_speed;

    let record_path = matches.value_of("record").unwrap(); /* has a default value */
    let mut record = if matches.occurrences_of("record") != 0 {
        record::Run::new(record_path)
    } else {
        record::Run::empty()
    };

    let mut replay = false;
    if matches.occurrences_of("play") != 0 {
        replay = true;
        record = record::Run::load(matches.value_of("play").unwrap())
    }

    let map_path = matches.value_of("map").unwrap(); /* has a default value */
    println!("Loading map: {}", map_path);

    let mut map = game::Map::new(map_path, CELL_SIZE, undo_level).unwrap();

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
    let mut now = timer_subsystem.ticks();
    let mut done: bool;
    let mut movements = 0;
    'main: loop {
        done = false;
        let mut cmd = None;
        for event in events.poll_iter() {
            if replay {
                match event {
                    Event::Quit {..} => {
                        break 'main
                    },
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'main
                    },
                    _ => {},
                }
            } else {
                match event {
                    Event::Quit {..} => {
                        record.record(record::Command::Quit);
                        break 'main
                    },
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        record.record(record::Command::Quit);
                        break 'main
                    },
                    Event::KeyDown { keycode, .. } => {
                        match keycode {
                            Some(key) => cmd = cmd_of_key(key, &keys),
                            None => ()
                        }
                    }
                    _ => {},
                }
            }
        }

        if replay {
            let cur = timer_subsystem.ticks();
            if cur - now > speed {
                now = cur;
                match record.next() {
                    Some(c) => {
                        cmd = Some(c)
                    },
                    None => break 'main
                }
            }
        }

        match cmd {
            Some(cmd) => {
                movements = movements + 1;
                if !replay { record.record(cmd) };
                match cmd {
                    record::Command::Quit => break 'main,
                    record::Command::Up => done = map.update(game::Direction::Up),
                    record::Command::Down => done = map.update(game::Direction::Down),
                    record::Command::Left => done = map.update(game::Direction::Left),
                    record::Command::Right => done = map.update(game::Direction::Right),
                    record::Command::Undo => map.undo(),
                    record::Command::Reset => map.reset(),
                }
            },
            None => ()
        }

        /* Render here */
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        map.render(&mut canvas);

        canvas.present();

        if done {
            break 'main
        }
    }
    if !replay {
        record.save(record_path)
    }

    if done {
        let w2 = window_width / 2;
        let h2 = window_height / 2;
        let r = Rect::new((w2 - w2 / 2) as i32, (h2 - h2 / 2) as i32, w2, h2);
        canvas.copy(&tex, None, Some(r)).unwrap();
        canvas.present();
        println!("Congratulations, you won !");
    } else {
        println!("Sorry, you failed");
    }
    println!("Movements: {}", movements);
    timer_subsystem.delay(2000);
}


fn cmd_of_key(key: Keycode, keys: &config::KeyBindings) -> Option<record::Command> {
    if key == keys.quit {
        Some(record::Command::Quit)
    } else if key == keys.up {
        Some(record::Command::Up)
    } else if key == keys.down {
        Some(record::Command::Down)
    } else if key == keys.left {
        Some(record::Command::Left)
    } else if key == keys.right {
        Some(record::Command::Right)
    } else if key == keys.undo {
        Some(record::Command::Undo)
    } else if key == keys.reset {
        Some(record::Command::Reset)
    } else {
        None
    }
}

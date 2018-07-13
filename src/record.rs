use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::io::{self, BufReader};

#[derive(Clone, Copy)]
pub enum Command {
    Up,
    Down,
    Left,
    Right,
    Undo,
    Reset,
    Quit
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Command::Up    => write!(f, "Up"),
                Command::Down  => write!(f, "Down"),
                Command::Left  => write!(f, "Left"),
                Command::Right => write!(f, "Right"),
                Command::Undo  => write!(f, "Undo"),
                Command::Reset => write!(f, "Reset"),
                Command::Quit  => write!(f, "Quit")
            }
    }
}

struct State {
    cmds: Vec<Command>,
}

impl State {
    #[allow(dead_code)]
    fn dump(&self) {
        for cmd in &self.cmds {
            println!("{}", cmd)
        }
    }

    fn next(&mut self) -> Option<Command> {
        if self.cmds.len() > 0 {
            let cmd = self.cmds.remove(0);
            Some(cmd)
        } else {
            None
        }
    }

    fn save(&self, path: &str) -> Result<(), io::Error> {
        let mut f = File::create(path)?;
        for cmd in &self.cmds {
            match f.write(format!("{}\n", cmd).as_bytes()) {
                Ok(_) => (),
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }

    fn parse_line(line: &String) -> Result<Command, String> {
        if line == "Up" { Ok(Command::Up) }
        else if line == "Down" { Ok(Command::Down) }
        else if line == "Left" { Ok(Command::Left) }
        else if line == "Right" { Ok(Command::Right) }
        else if line == "Undo" { Ok(Command::Undo) }
        else if line == "Reset" { Ok(Command::Reset) }
        else if line == "Quit" { Ok(Command::Quit) }
        else {
            Err(format!("Unknown command: {}", line))
        }
    }

    fn load(path: &str) -> Result<State, String> {
        let f = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e))
        };
        let buf = BufReader::new(f);
        let mut cmds = Vec::new();
        for l in buf.lines() {
            match l {
                Ok(l) => cmds.push(State::parse_line(&l)?),
                Err(e) => return Err(format!("{}", e))
            }
        }
        Ok(State { cmds: cmds })
    }
}

pub struct Run {
    empty: bool,
    state: State
}

impl Run {
    pub fn new(path: &str) -> Run {
        println!("New run {}", path);
        let state = State {
            cmds: Vec::new()
        };
        Run { empty: false, state: state }
    }

    pub fn load(path: &str) -> Run {
        println!("Loading {}", path);
        let state = match State::load(path) {
            Ok(s) => s,
            Err(e) => panic!("Failure to load '{}': {}", path, e)
        };
        Run { empty: false, state: state }
    }

    pub fn next(&mut self) -> Option<Command> {
        if self.empty {
            None
        } else {
            self.state.next()
        }
    }

    pub fn empty() -> Run {
        Run { empty: true, state: State { cmds: Vec::new() } }
    }

    pub fn record(&mut self, cmd: Command) {
        if !self.empty {
            self.state.cmds.push(cmd);
        }
    }

    pub fn save(&self, path: &str) {
        if !self.empty {
            match self.state.save(path) {
                Ok(_) => (),
                Err(e) => eprintln!("Error while saving run to '{}': {}",
                                    path, e)
            }
        }
    }
}

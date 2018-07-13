use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::io;

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
    path: String
}

impl State {
    #[allow(dead_code)]
    fn dump(&self) {
        for cmd in &self.cmds {
            println!("{}", cmd)
        }
    }

    fn save(&self) -> Result<(), io::Error> {
        let mut f = File::create(&self.path)?;
        for cmd in &self.cmds {
            match f.write(format!("{}\n", cmd).as_bytes()) {
                Ok(_) => (),
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }
}

pub struct Run {
    state: Option<State>
}

impl Run {
    pub fn new(path: String) -> Run {
        println!("New run {}", path);
        let state = State {
            cmds: Vec::new(),
            path: path
        };
        Run { state: Some(state) }
    }

    pub fn empty() -> Run {
        Run { state: None }
    }

    pub fn record(&mut self, cmd: Command) {
        match self.state {
            Some(ref mut state) => {
                state.cmds.push(cmd);
            },
            None => ()
        }
    }

    pub fn save(&self) {
        match self.state {
            None => (),
            Some(ref state) => match state.save() {
                Ok(_) => (),
                Err(e) => eprintln!("Error while saving run to '{}': {}",
                                    state.path, e)
            }
        }
    }
}

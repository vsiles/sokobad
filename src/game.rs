extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn read_int(line: &String) -> Result<i32, String> {
    match line.parse::<i32>() {
        Ok(i) => Ok(i),
        Err(e) => return Err(format!("Can't parse {} as i32: {}",
                                     line, e))
    }
}


pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone)]
enum CellType {
    Wall,
    Crate,
    Empty,
    Block,
    Exit,
}

#[derive(Copy, Clone)]
struct Cell {
    kind: CellType,
    goal: bool
}

impl Cell {
    fn color(&self, goals_left: i32) -> Color {
        match self.kind {
            CellType::Wall => Color::RGB(96, 96, 96),
            CellType::Block => if self.goal {
                Color::RGB(103, 240, 139)
            } else {
                Color::RGB(102, 51, 0)
            },
            CellType::Crate => Color::RGB(255, 128, 0),
            CellType::Exit => if goals_left > 0 {
                Color::RGB(0, 0, 0)
            } else {
                Color::RGB(255, 255, 255)
            },
            CellType::Empty => if self.goal {
                Color::RGB(255, 255, 51)
            } else {
                Color::RGB(192, 192, 192)
            }
        }
    }

    fn is_free(&self, solved: bool) -> bool {
        match self.kind {
            CellType::Wall => false,
            CellType::Exit => solved,
            CellType::Block => false,
            CellType::Empty => true,
            CellType::Crate => false,
        }
    }

    fn is_movable(&self) -> bool {
        match self.kind {
            CellType::Block => true,
            CellType::Crate => true,
            _ => false
        }
    }

    fn is_crate(&self) -> bool {
        match self.kind {
            CellType::Crate => true,
            _ => false
        }
    }

    fn is_goal(&self) -> bool {
        self.goal
    }

    fn is_exit(&self) -> bool {
        match self.kind {
            CellType::Exit => true,
            _ => false
        }
    }

    fn non_goal(kind: CellType) -> Cell {
        Cell { kind: kind, goal: false }
    }
}

#[derive(Clone)]
struct Player {
    x: i32,
    y: i32
}

#[derive(Clone)]
struct State {
    data: Vec<Vec<Cell>>,
    player: Player,
    solved: bool,
    goals_left: i32
}

impl State {
    fn inspect(&mut self, x1: usize, y1: usize, x2: usize, y2: usize,
            next: bool) -> bool {
        let cell = self.data[y1][x1];
        if cell.is_free(self.solved) {
            return true
        }
        if !next { return false }
        let ncell = self.data[y2][x2];
        if cell.is_movable() && ncell.is_free(self.solved) {
            if cell.is_crate() {
                /* Move but can't succeed */
                self.data[y2][x2].kind = CellType::Crate;
                self.data[y1][x1].kind = CellType::Empty;
            } else {
                if self.data[y2][x2].is_goal() {
                    /* Block on goal -> success */
                    self.goals_left = self.goals_left - 1
                }
                self.data[y2][x2].kind = CellType::Block;

                if self.data[y1][x1].is_goal() {
                    /* Block removed from goal -> failure */
                    self.goals_left = self.goals_left + 1
                }
                self.data[y1][x1].kind = CellType::Empty
            }
            /* finally, let's move */
            return true
        }
        return false;
    }
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    lines: Vec<String>, /* currently usued, will be used for reset */
    max_undo: usize,
    cell_size: u32,
    states: Vec<State>
}

impl Map {
    fn load(lines: &Vec<String>) -> Result<(i32, i32, State), String> {
        let mut iter = lines.into_iter();
        let width = match iter.next() {
            Some(l) => read_int(l)?,
            None => return Err(format!("Invalid map format\n"))
        };
        let height = match iter.next() {
            Some(l) => read_int(l)?,
            None => return Err(format!("Invalid map format\n"))
        };
        let mut map = Vec::new();
        let mut x = 0;
        let mut y = 0;
        let mut start = false;
        let mut exit_cell = false;
        let mut num_blocks = 0;
        let mut num_goals = 0;

        for j in 0..height {
            let line: Vec<char> = match iter.next() {
                Some(l) => l.chars().collect(),
                None => return Err(format!("Invalid map format\n"))
            };
            let mut row = Vec::new();
            for i in 0..width {
                let c = line[i as usize];
                row.push(
                    match c {
                        's' => {
                            if !start {
                                start = !start;
                                x = i;
                                y= j;
                                Cell::non_goal(CellType::Empty)
                            } else {
                                return Err(format!("Multiple start points"))
                            }
                        },
                        '.' => Cell::non_goal(CellType::Wall),
                        ' ' => Cell::non_goal(CellType::Empty),
                        'g' => {
                            num_goals = num_goals + 1;
                            Cell { kind: CellType::Empty, goal: true }
                        },
                        'b' => {
                            num_blocks = num_blocks + 1;
                            Cell::non_goal(CellType::Block)
                        },
                        'c' => Cell::non_goal(CellType::Crate),
                        'x' => {
                            if !exit_cell {
                                exit_cell = !exit_cell;
                                Cell::non_goal(CellType::Exit)
                            } else {
                                return Err(format!("Multiple exit points"))
                            }
                        },
                        _  => return Err(format!("Invalid map: {}", c))
                    }
                )
            }
            map.push(row);
        }
        if !start {
            return Err(format!("Missing start point"))
        } else if !exit_cell {
            return Err(format!("Missing exit point"))
        } else if num_goals <= 0 {
            return Err(format!("Not enough goals"))
        } else if num_goals != num_blocks {
            return Err(format!("Block/Goal mismatch"))
        }
        let state = State {
            data: map,
            player: Player { x: x, y: y },
            solved: false,
            goals_left: num_goals
        };
        Ok((width, height, state))
    }

    pub fn new(path: &str, cell_size: u32, max_undo: usize) -> Result<Map, String> {

        let f = match File::open(path) {
            Ok(f) => f,
            Err(e) =>
                return Err(format!("Cannot open '{}': {}\n", path, e))
        };
        let buf = BufReader::new(f);
        let mut lines = Vec::new();
        for l in buf.lines() {
            match l {
                Ok(l) => lines.push(l),
                Err(e) =>
                    return Err(format!("Invalid line in '{}': {}\n", path, e))
            }
        }
        let (width, height, state) = match Map::load(&lines) {
            Ok(d) => d,
            Err(e) => return Err(e)
        };
        let mut states = Vec::new();
        states.push(state);
        Ok(Map {
            width: width, height: height, states: states,
            cell_size: cell_size, max_undo: max_undo,
            lines: lines
        })
    }

    fn get_state(&mut self) -> &mut State {
        let len = self.states.len();
        &mut self.states[len - 1]
    }

    fn get_state_ro(&self) -> &State {
        let len = self.states.len();
        &self.states[len - 1]
    }

    pub fn update(&mut self, dir: Direction) -> bool {
        /* copy current state */
        let len = self.states.len();
        let mut state = self.states[len - 1].clone();

        let w: usize = self.width as usize;
        let h: usize = self.height as usize;
        let x: usize = state.player.x as usize;
        let y: usize = state.player.y as usize;
        let moved = {
            match dir {
                Direction::Up => if y > 0 {
                    let moved = state.inspect(x, y - 1, x, y - 2, y > 1);
                    if moved { state.player.y = state.player.y - 1 }
                    moved
                } else { false },
                Direction::Down => if y < h - 1 {
                    let moved = state.inspect(x, y + 1, x, y + 2, y < h - 2);
                    if moved { state.player.y = state.player.y + 1 }
                    moved
                } else { false },
                Direction::Left => if x > 0 {
                    let moved = state.inspect(x - 1, y, x - 2, y, x > 1);
                    if moved { state.player.x = state.player.x - 1 }
                    moved
                } else { false },
                Direction::Right => if x < w - 1 {
                    let moved = state.inspect(x + 1, y, x + 2, y, x < w - 2);
                    if moved { state.player.x = state.player.x + 1 }
                    moved
                } else { false }
            }
        };

        /* If we moved, update the undo stack */
        if moved {
            if len >= self.max_undo {
                self.states.remove(0);
            }
            self.states.push(state);
        }

        let curr_state = self.get_state();
        if curr_state.goals_left == 0 {
            curr_state.solved = true;
            let fx : usize = curr_state.player.x as usize;
            let fy : usize = curr_state.player.y as usize;
            if curr_state.data[fy][fx].is_exit() {
                return true
            }
        } else {
            curr_state.solved = false
        }
        return false
    }

    pub fn render(&self, canvas: & mut sdl2::render::WindowCanvas) {
        let cs : i32 = self.cell_size as i32;
        let state = self.get_state_ro();
        for j in 0..self.height {
            for i in 0..self.width {
                let cell = &state.data[j as usize][i as usize];
                canvas.set_draw_color(cell.color(state.goals_left));
                canvas.fill_rect(Rect::new(i * cs, j * cs,
                                           self.cell_size, self.cell_size)).unwrap();
            }
        }
        /* Draw player */
        canvas.set_draw_color(Color::RGB(255, 51, 51));
        canvas.fill_rect(Rect::new(state.player.x * cs,
                                   state.player.y * cs,
                                   self.cell_size, self.cell_size)).unwrap();
    }

    pub fn undo(&mut self) {
        if self.states.len() > 1 {
            self.states.pop();
        }
    }

    pub fn reset(&mut self) {
        let state = match Map::load(&self.lines) {
            Ok((_, _, s)) => s,
            Err(e) => panic!("Map reset should not fail: {}\n", e)
        };
        self.states.clear();
        self.states.push(state);
    }
}

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::fmt;
use std::io;

fn read_line() -> Result<String, String> {
    let mut line = "".to_string();
    match io::stdin().read_line(&mut line) {
        Ok(_) => (), /* disregard the returned len */
        Err(e) => return Err(format!("Can't read stdin: {}", e))
    }
    let len_withoutcrlf = line.trim_right().len();
    line.truncate(len_withoutcrlf);
    Ok(line)
}

fn read_int() -> Result<i32, String> {
    let line = read_line()?;
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
enum Cell {
    Wall,
    Crate,
    Empty,
    Goal,
    Block,
    Exit,
    Success
}

impl Cell {
    fn color(&self, goals_left: i32) -> Color {
        match *self {
            Cell::Wall => Color::RGB(96, 96, 96),
            Cell::Empty => Color::RGB(192, 192, 192),
            Cell::Goal => Color::RGB(255, 255, 51),
            Cell::Block => Color::RGB(102, 51, 0),
            Cell::Success => Color::RGB(103, 240, 139),
            Cell::Crate => Color::RGB(255, 128, 0),
            Cell::Exit => if goals_left > 0 {
                Color::RGB(0, 0, 0)
            } else {
                Color::RGB(255, 255, 255)
            }
        }
    }
}


impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Wall => write!(f, "."),
            Cell::Empty => write!(f, " "),
            Cell::Goal => write!(f, "g"),
            Cell::Block => write!(f, "b"),
            Cell::Exit => write!(f, "x"),
            Cell::Success => write!(f, "!"),
            Cell::Crate => write!(f, "c"),
        }
    }
}

impl Cell {
    fn is_free(&self, solved: bool) -> bool {
        match *self {
            Cell::Wall => false,
            Cell::Exit => solved,
            Cell::Success => false,
            Cell::Block => false,
            Cell::Goal => true,
            Cell::Empty => true,
            Cell::Crate => false,
        }
    }

    fn is_movable(&self) -> bool {
        match *self {
            Cell::Block => true,
            Cell::Success => true,
            Cell::Crate => true,
            _ => false
        }
    }

    fn is_crate(&self) -> bool {
        match *self {
            Cell::Crate => true,
            _ => false
        }
    }

    fn is_goal(&self) -> bool {
        match *self {
            Cell::Goal => true,
            _ => false
        }
    }

    fn is_success(&self) -> bool {
        match *self {
            Cell::Success => true,
            _ => false
        }
    }

    fn is_exit(&self) -> bool {
        match *self {
            Cell::Exit => true,
            _ => false
        }
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
    fn move_up_down<F1, F2>(&mut self, x: usize, y: usize, next: bool,
                            f1: F1, f2: F2) -> bool
    where F1: Fn(usize, usize) -> usize,
          F2: Fn(i32, i32) -> i32 {
        let cell = self.data[f1(y, 1)][x];
        if cell.is_free(self.solved) {
            self.player.y = f2(self.player.y , 1);
            return true
        }
        if !next { return false }
        let ncell = self.data[f1(y, 2)][x];
        if cell.is_movable() && ncell.is_free(self.solved) {
            if cell.is_crate() {
                /* Move but can't succeed */
                self.data[f1(y, 2)][x] = Cell::Crate;
                self.data[f1(y, 1)][x] = Cell::Empty;
            } else {
                if self.data[f1(y, 2)][x].is_goal() {
                    /* Block on goal -> success */
                    self.data[f1(y, 2)][x] = Cell::Success;
                    self.goals_left = self.goals_left - 1
                } else {
                    self.data[f1(y, 2)][x] = Cell::Block
                }

                if self.data[f1(y, 1)][x].is_success() {
                    self.data[f1(y, 1)][x] = Cell::Goal;
                    self.goals_left = self.goals_left + 1
                } else {
                    self.data[f1(y, 1)][x] = Cell::Empty
                }
            }
            /* finally, let's move */
            self.player.y = f2(self.player.y, 1);
            return true
        }
        return false;
    }

    fn move_left_right<F1, F2>(&mut self, x: usize, y: usize, next: bool,
                               f1: F1, f2: F2) -> bool
    where F1: Fn(usize, usize) -> usize,
          F2: Fn(i32, i32) -> i32 {
        let cell = self.data[y][f1(x, 1)];
        if cell.is_free(self.solved) {
            self.player.x = f2(self.player.x , 1);
            return true
        }
        if !next { return false }
        let ncell = self.data[y][f1(x, 2)];
        if cell.is_movable() && ncell.is_free(self.solved) {
            if cell.is_crate() {
                /* Move but can't succeed */
                self.data[y][f1(x, 2)] = Cell::Crate;
                self.data[y][f1(x, 1)] = Cell::Empty;
            } else {
                if self.data[y][f1(x, 2)].is_goal() {
                    /* Block on goal -> success */
                    self.data[y][f1(x, 2)] = Cell::Success;
                    self.goals_left = self.goals_left - 1
                } else {
                    self.data[y][f1(x, 2)] = Cell::Block
                }

                if self.data[y][f1(x, 1)].is_success() {
                    self.data[y][f1(x, 1)] = Cell::Goal;
                    self.goals_left = self.goals_left + 1
                } else {
                    self.data[y][f1(x, 1)] = Cell::Empty
                }
            }
            /* finally, let's move */
            self.player.x = f2(self.player.x, 1);
            return true
        }
        return false
    }
}


pub struct Map {
    pub width: i32,
    pub height: i32,
    max_undo: usize,
    cell_size: u32,
    state: Vec<State>
}

impl Map {
    pub fn new(cell_size: u32, max_undo: usize) -> Result<Map, String> {
        let width = read_int()?;
        let height = read_int()?;
        let mut map = Vec::new();
        let mut x = 0;
        let mut y = 0;
        let mut start = false;
        let mut exit_cell = false;
        let mut goals_left = 0;

        for j in 0..height {
            let line: Vec<char> = {
                match read_line() {
                    Ok(d) => d.chars().collect(),
                    Err(e) => return Err(format!("read_line failure: {}", e))
                }
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
                                Cell::Empty
                            } else {
                                return Err(format!("Multiple start points"))
                            }
                        },
                        '.' => Cell::Wall,
                        ' ' => Cell::Empty,
                        'g' => {
                            goals_left = goals_left + 1;
                            Cell::Goal
                        },
                        'b' => Cell::Block,
                        'c' => Cell::Crate,
                        'x' => {
                            if !exit_cell {
                                exit_cell = !exit_cell;
                                Cell::Exit
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
            Err(format!("Missing start point"))
        } else if !exit_cell {
            Err(format!("Missing exit point"))
        } else if goals_left <= 0 {
            Err(format!("Not enough goals"))
        } else {
            let mut state = Vec::new();
            state.push(State { data: map, player: Player { x: x, y: y},
                        solved: false, goals_left: goals_left });
            Ok(Map {
                width: width, height: height, state: state,
                cell_size: cell_size, max_undo: max_undo
            })
        }
    }

    fn get_state(&mut self) -> &mut State {
        let len = self.state.len();
        &mut self.state[len - 1]
    }

    fn get_state_ro(&self) -> &State {
        let len = self.state.len();
        &self.state[len - 1]
    }

    pub fn update(&mut self, dir: Direction) -> bool {
        /* copy current state */
        let len = self.state.len();
        let mut state = self.state[len - 1].clone();

        let w: usize = self.width as usize;
        let h: usize = self.height as usize;
        let x: usize = state.player.x as usize;
        let y: usize = state.player.y as usize;
        let moved = {
            match dir {
                Direction::Up => if y > 0 {
                    state.move_up_down(x, y, y > 1, |x, y| x - y, |x, y| x - y)
                } else { false },
                Direction::Down => if y < h - 1 {
                    state.move_up_down(x, y, y < h - 2, |x, y| x + y, |x, y| x + y)
                } else { false },
                Direction::Left => if x > 0 {
                    state.move_left_right(x, y, x > 1, |x, y| x - y, |x, y| x - y)
                } else { false },
                Direction::Right => if x < w - 1 {
                    state.move_left_right(x, y, x < w - 2, |x, y| x + y, |x, y| x + y)
                } else { false }
            }
        };

        /* If we moved, update the undo stack */
        if moved {
            if len >= self.max_undo {
                self.state.remove(0);
            }
            self.state.push(state);
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

    #[allow(dead_code)]
    pub fn dump(&self) {
        let state = self.get_state_ro();
        for j in 0..self.height {
            for i in 0..self.width {
                print!("{}", state.data[j as usize][i as usize])
            }
            println!("")
        }
        println!("")
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
        if self.state.len() > 1 {
            self.state.pop();
        }
    }
}

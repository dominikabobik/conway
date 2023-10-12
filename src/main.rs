use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{fmt::{self, Display}, thread};
use std::io::stdout;
use std::time::Duration;

#[derive(Clone)]
struct Cell {
    alive: bool,
}

struct Board {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        Board {
            width,
            height,
            cells: vec![Cell { alive: false }; (width * height) as usize],
        }
    }

    pub fn init_state(&mut self, states: Vec<(u32, u32)>) -> Result<(), String> {
        if states.len() > (self.width * self.height) as usize {
            return Err("Too many states".to_string());
        }
        for state in states {
            let index = self.calculate_index(state.to_owned()).unwrap();
            self.cells[index].alive = true;
        }
        Ok(())
    }

    pub fn calculate_index(&self, position: (u32, u32)) -> Result<usize, String> {
        let index: usize = (position.0 * self.width + position.1) as usize;
        if index > self.cells.len() - 1 {
            return Err("Index out of bounds".to_string());
        }
        Ok(index)
    }

    pub fn tick(&mut self) {
        let mut new_board: Vec<Cell> = self.cells.clone();
        for i in 0..self.cells.len() {
            let cell: Cell = self.cells[i].clone();
            let alive_neighbours: i32 = Self::get_alive_neighbours(&self, i as i32);

            if alive_neighbours < 2 && cell.alive {
                // die
                new_board[i].alive = false;
            } else if (alive_neighbours == 2 || alive_neighbours == 3) && cell.alive {
                // live
                new_board[i].alive = true;
            } else if alive_neighbours == 3 && !cell.alive {
                // become alive
                new_board[i].alive = true;
            } else if alive_neighbours > 3 && cell.alive {
                // die
                new_board[i].alive = false;
            }
        }
        self.cells = new_board;
    }

    fn get_alive_neighbours(&self, index: i32) -> i32 {
        let mut alive_nighbours_count: i32 = 0;
        // top left
        let top_left_index: i32 = index - (self.width as i32) - 1;
        if top_left_index >= 0 {
            if self.cells[top_left_index as usize].alive {
                alive_nighbours_count = alive_nighbours_count + 1;
            }
        }
        // top
        let top_index = index - (self.width as i32);
        if top_index >= 0 {
            if self.cells[top_index as usize].alive {
                alive_nighbours_count = alive_nighbours_count + 1;
            }
        }
        // top right
        let top_right_index: i32 = index - (self.width as i32) + 1;
        if top_right_index >= 0 {
            if self.cells[top_right_index as usize].alive {
                alive_nighbours_count = alive_nighbours_count + 1;
            }
        }
        // right
        let right_index: i32 = index + 1;
        if right_index < self.cells.len() as i32{
            if self.cells[right_index as usize].alive {
                alive_nighbours_count = alive_nighbours_count + 1;
            }
        }
        // bottom right
        let bottom_right_index: i32 = index + (self.width as i32) + 1;
        if bottom_right_index < self.cells.len() as i32{
            if self.cells[bottom_right_index as usize].alive {
                alive_nighbours_count = alive_nighbours_count + 1;
            }
        }
        // bottom
        let bottom_index: i32 = index + (self.width as i32);
        if bottom_index < self.cells.len() as i32{
            if self.cells[bottom_index as usize].alive {
                alive_nighbours_count = alive_nighbours_count + 1;
            }
        }
        // bottom left
        let bottom_left_index: i32 = index + (self.width as i32) - 1;
        if bottom_left_index < self.cells.len() as i32 && bottom_left_index >= 0{
            if self.cells[bottom_left_index as usize].alive {
                alive_nighbours_count = alive_nighbours_count + 1;
            }
        }
        // left
        let left_index: i32 = index - 1;
        if left_index >= 0 {
            if self.cells[left_index as usize].alive {
                alive_nighbours_count = alive_nighbours_count + 1;
            }
        }
        return alive_nighbours_count;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.width {
            for x in 0..self.height {
                if self.cells[self.calculate_index((x, y)).unwrap()].alive {
                    write!(f, "🍀")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut board: Board = Board::new(30, 30);
    board
        .init_state(vec![(0, 2), (2, 2), (1, 1), (4, 4), (23, 24), (24, 23), (24, 24), (22, 23), (23, 23),(25,25), (25, 24), (22,25)])
        .unwrap();

    println!("{}", board);
    thread::sleep(Duration::from_secs(3));

    execute!(
        stdout(),
        EnterAlternateScreen,
        SetForegroundColor(Color::Magenta),
        Hide
    ).unwrap();

    loop {
        if poll(Duration::from_millis(400)).unwrap() {
            match read().unwrap() {
                Event::Key(_) => break,
                _ => {}
            }
        } else {
            execute!(
                stdout(),
                Clear(ClearType::All),
                MoveTo(0, 0),
                Print(&board),
                Print("Press enter to exit...")
            )
            .unwrap();
            board.tick();
        }

    }
    execute!(stdout(), ResetColor, Show, LeaveAlternateScreen).unwrap();
}

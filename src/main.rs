use std::{error::Error, fmt::{Display, self}};


#[derive(Clone)]
struct Cell {
    alive: bool
}

struct Board {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

impl Board {
    pub fn new(width: u32, height: u32) -> Board{
        Board {
            width: width,
            height: height,
            cells: vec![Cell {alive: false}; (width*height) as usize]
        }
    }

    pub fn init_state(&mut self, states: Vec<(u32, u32)>) -> Result<(), String>
    {
        if states.len() > (self.width * self.height) as usize
        {
            return Err("Too many states".to_string())
        }
        for state in states
        {
            let index = self.calculate_index(state.to_owned()).unwrap();
            self.cells[index].alive = true;
        };
        Ok(())
    }

    pub fn calculate_index(&self, position: (u32, u32)) -> Result<usize, String>
    {
        let index: usize = (position.0*self.width + position.1) as usize;
        if index > self.cells.len() - 1
        {
            return Err("Index out of bounds".to_string());
        }
        Ok(index)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.width
        {
            for x in 0..self.height
            {
                if self.cells[self.calculate_index((x, y)).unwrap()].alive
                {
                    write!(f, "O")?;
                }
                else {
                    write!(f, "X")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
fn main() {
    println!("Hello, world!");
    let mut board: Board = Board::new(3, 3);
    board.init_state(vec![(0,2), (2,2), (1,1)]).unwrap();
    print!("{}", board);
}

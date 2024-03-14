use std::{fmt::Display, rc::Rc};
mod lifecell;
mod fluid;
use lifecell::LiveCell;
use fluid::FluidCell;

trait AutomataCell where 
    Self: Clone + Eq + PartialEq
{
    fn step(&self, n: &[&Self]) -> Self;
}


#[derive(Clone, Debug)]
struct Board<Cell> where
    Cell: AutomataCell
{
    width : usize,
    height: usize,
    current: usize,
    boards: [Rc<[Cell]>; 2],
}

impl<Cell> Board<Cell> where
    Cell: AutomataCell
{
    pub fn init_with<F>(width: usize, height: usize, f: F) -> Self
    where
        F: Fn() -> Cell
    {
        let size = width * height;
        let mut bufa = Vec::new();
        bufa.resize_with(size, f);
        let bufb = bufa.clone();
        Self{
            current: 0,
            width, height,
            boards : [
                bufa.into(),
                bufb.into(),
            ]
        }
    }

    pub fn get_at(board: &Rc<[Cell]>, w: usize, h: usize, width: usize) -> &Cell{
        &board[w + h * width]
    }

    pub fn step(&mut self) {
        let curr_board = self.boards[(self.current + 0) % 2].clone();
        let next_board = &mut self.boards[(self.current + 1) % 2];

        for index in 0..self.height * self.width{
            let w = index % self.width;
            let h = index / self.width;
            // returns the neighbors of the cell at (w, h)
            // if it is in the border it will wrap around
            let get = |x: isize, y: isize| {
                let x = (x + w as isize).rem_euclid(self.width as isize);
                let y = (y + h as isize).rem_euclid(self.height as isize);
                Self::get_at(&curr_board, x as usize, y as usize, self.width)
            };
            let neightbors = [
                get(-1, -1), get(0, -1), get(1, -1),
                get(-1,  0), get(1,  0),
                get(-1,  1), get(0,  1), get(1,  1),
            ];
            Rc::get_mut(next_board).unwrap()[index] = curr_board[index].step(&neightbors);
        }

        self.current = (self.current + 1) % 2;
    }

    fn stable (&self) -> bool {
        self.boards[0] == self.boards[1]
    }

    #[allow(dead_code)]
    fn set_at(&mut self, w: usize, h: usize, cell: Cell){
        Rc::get_mut(&mut self.boards[self.current]).unwrap()[w + h * self.width] = cell;
    }
}

impl<Cell> Display for Board<Cell> where
    Cell: AutomataCell + Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for h in 0..self.height{
            for w in 0..self.width{
                write!(f, "{:3} ", self.boards[self.current][w + h * self.width])?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\x1B[{}A", self.height)?;
        Ok(())
    }
}

const SIZE: usize = 16;
fn main() {
    let mut rand_board = Board::init_with(SIZE, SIZE, FluidCell::random);
    loop {
        print!("{rand_board}");
        rand_board.step();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        if rand_board.stable() { break; }
    }

    println!("reached stable board!");

}

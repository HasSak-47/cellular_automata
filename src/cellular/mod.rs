pub mod wacky_cell;
pub mod life_cell;

pub trait Cell{
    fn update(&self, neightbors: [&Self; 8]) -> Self;
    // fn is(&self, is_kind: Self) -> bool;
    fn random() -> Self;
}


#[derive(Clone, Copy)]
pub struct Board<CellType, const WIDTH: usize, const HEIGHT: usize>{
    width: isize,
    height: isize,
    cells: [[CellType; HEIGHT]; WIDTH],
}

impl<CellType: Default + Copy + Cell, const WIDTH: usize, const HEIGHT: usize> Board<CellType, WIDTH, HEIGHT>{
    pub fn width(&self) -> isize  {self.width}
    pub fn height(&self) -> isize {self.height}
    pub fn new() -> Self {
        Board{
            cells: [[CellType::default(); HEIGHT]; WIDTH],
            height: HEIGHT as isize,
            width : WIDTH as isize,
        }
    }

    pub fn random() -> Self {
        let mut b = Self::new();

        for i in 0..b.width{
            for j in 0..b.height{
                *b.get_mut(i, j) = CellType::random();
            }
        }

        b
    }

    fn id(&self, x: isize, y: isize) -> (usize, usize) {
        let modx = x % self.width;
        let x = if modx < 0  {self.width + modx} else {modx};
        let mody = y % self.height;
        let y = if mody < 0 {self.height + mody} else {mody};

        (x as usize, y as usize)
    }

    pub fn get(&self, x: isize, y: isize) -> &CellType{
        let a = self.id(x, y);
        &self.cells[a.0][a.1]

    }

    pub fn update_cell(&self, x: isize, y: isize) -> CellType{
        // disguisting
        let neightbors = [
            self.get(x - 1, y - 1),
            self.get(x + 0, y - 1),
            self.get(x + 1, y - 1),
            self.get(x - 1, y + 0),
            // self.get(x + 0, y + 0),
            self.get(x + 1, y + 0),
            self.get(x - 1, y + 1),
            self.get(x + 0, y + 1),
            self.get(x + 1, y + 1),
        ];

        self.get(x, y).update(neightbors)
    }

    #[inline]
    pub fn get_mut(&mut self, x: isize, y: isize) -> &mut CellType{
        let a = self.id(x, y);
        &mut self.cells[a.0][a.1]
    }

    pub fn update(&mut self){
        let mut copy = self.clone();
        for i in 0..self.width{
            for j in 0..self.height {
                *copy.get_mut(i, j) = self.update_cell(i, j); 
            }
        }

        for i in 0..self.width{
            for j in 0..self.height {
                let j = j as isize;
                let i = i as isize;
                *self.get_mut(i, j) = *copy.get_mut(i, j); 
            }
        }

    }
}

impl<CellType, const WIDHT: usize, const HEIGHT: usize> std::fmt::Display for Board<CellType, WIDHT, HEIGHT>
where
    CellType: std::fmt::Display + Cell + Default + Copy
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for i in 0..self.width{
            for j in 0..self.height{
                string.push_str(self.get(i, j).to_string().as_str());
            }
            string.push('\n');
        }
        string.pop();

        write!(f, "{}", string)
    }
}

pub use life_cell::*;

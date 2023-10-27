
#[derive(Clone, Debug, Default)]
struct Board<Cell>{
    width : usize,
    height: usize,
    boards: [Vec<Cell>; 2],
    current: usize,
}

trait AutomataCell where Self: Sized{
    fn random() -> Self;
    fn update(neightbors: [&Self; 9]) -> Self;
}

impl<Cell> Board<Cell>
where
    Cell: Default + Clone + Copy + AutomataCell
{
    fn __new<F: FnMut() -> Cell>(width: usize, height: usize, f: F) -> Self{
        let size = width * height;
        let mut boards = [ Vec::new(), Vec::new(), ];
        boards[0].resize_with(size, f);
        boards[1].resize_with(size, Cell::default);
        Board{width, height, boards, current: 0}
    }

    pub fn new_default(width: usize, height: usize) -> Self{
        Self::__new(width, height, Cell::default)
    }

    pub fn new_random(width: usize, height: usize) -> Self{
        Self::__new(width, height, Cell::random)
    }
    pub fn update(&mut self) {
        for i in 0..self.width * self.height{
            let x = (i % self.width) as isize;
            let y = (i / self.width) as isize;
            let neightbors = [
                &self.boards[self.current][self.get_uclid(x - 1, y - 1)],
                &self.boards[self.current][self.get_uclid(x - 1, y + 0)],
                &self.boards[self.current][self.get_uclid(x - 1, y + 1)],
                &self.boards[self.current][self.get_uclid(x + 0, y - 1)],
                &self.boards[self.current][self.get_uclid(x + 0, y + 0)],
                &self.boards[self.current][self.get_uclid(x + 0, y + 1)],
                &self.boards[self.current][self.get_uclid(x + 1, y - 1)],
                &self.boards[self.current][self.get_uclid(x + 1, y + 0)],
                &self.boards[self.current][self.get_uclid(x + 1, y + 1)],
            ];


            let index = self.get_uclid(x,y);
            self.boards[self.future_board()][index]= Cell::update(neightbors);
        }
        self.current = self.future_board();
    }
}

// util functions
impl<Cell> Board<Cell>{
    fn future_board(&self) -> usize { (self.current + 1) % 2 }

    fn get_uclid(&self, x: isize, y: isize) -> usize {
        (x + y * self.width as isize).rem_euclid((self.width * self.height) as isize) as usize
    }

    fn get_uclid_mut(&mut self, x: isize, y: isize) -> usize {
        (x + y * self.width as isize).rem_euclid((self.width * self.height) as isize) as usize
    }

    pub fn get(&self, x: isize, y: isize) -> &Cell {
        let current = &self.boards[self.current];
        &current[self.get_uclid(x, y)]
    }
}


impl<Cell> std::fmt::Display for Board<Cell>
where
    Cell: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height{
            for i in 0..self.width{
                write!(f, "{}", self.get(i as isize, j as isize))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum LifeCell{
    #[default]
    Dead=0,
    Alive=1,
}

impl From<&LifeCell> for usize{
    fn from(value: &LifeCell) -> Self {
        use LifeCell as LF;
        match value {LF::Dead => 0, LF::Alive => 1}
    }
}

impl AutomataCell for LifeCell{
    fn update(neightbors: [&Self; 9]) -> LifeCell{
        let mut total : usize = 0;
        for i in 0..9{
            let v : usize = neightbors[i].into();
            total += v;
        }
        if *neightbors[4] == LifeCell::Alive{
            if total < 3{
                LifeCell::Dead
            }
            else if total > 4{
                LifeCell::Dead
            }
            else {
                LifeCell::Alive
            }
        }
        else{
            if total == 3{
                LifeCell::Alive
            }
            else{
                LifeCell::Dead
            }
        }
    }

    fn random() -> Self {
        LifeCell::Alive
    }
}

impl std::fmt::Display for LifeCell{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LifeCell as LF;
        write!(f, "{}", match self {LF::Alive => "##", LF::Dead => "  "})
    }
}

fn main() {
    let mut b = Board::<LifeCell>::new_default(10, 10);
    b.boards[b.current][0] = LifeCell::Alive;
    b.boards[b.current][1] = LifeCell::Alive;
    b.boards[b.current][0 + 10] = LifeCell::Alive;
    b.boards[b.current][1 + 10] = LifeCell::Alive;

    println!("{b}");
    println!("-----------");
    b.update();
    println!("{b}");
}

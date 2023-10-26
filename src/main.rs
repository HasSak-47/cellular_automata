
#[derive(Clone, Debug, Default)]
struct Board<Cell>{
    width : usize,
    height: usize,
    boards: [Vec<Cell>; 2],
    current: usize,
}

trait LifeCell{
    fn random() -> Self;
    fn update(&mut self, neightbors: [&Self; 9]);
}

impl<Cell> Board<Cell>
where
    Cell: Default + Clone + LifeCell
{
    fn __new<F: FnMut() -> Cell>(width: usize, height: usize, f: F) -> Self{
        let mut boards = [
            Vec::with_capacity(width * height),
            Vec::with_capacity(width * height),
        ];
        boards[0].fill_with(f);
        boards[1].fill_with(Cell::default);
        Board{width, height, boards, current: 0}
    }

    pub fn new_default(width: usize, height: usize) -> Self{
        Self::__new(width, height, Cell::default)
    }

    pub fn new_random(width: usize, height: usize) -> Self{
        Self::__new(width, height, Cell::random)
    }

    fn future_board(&self) -> usize { (self.current + 1) % 2 }
    fn get(&self, x: isize, y: isize) -> &Cell { &self.boards[self.current][(x + y * self.width as isize).rem_euclid((self.width * self.height) as isize) as usize]}
    fn get_mut(&mut self, x: isize, y: isize) -> &mut Cell { &mut self.boards[self.current][(x + y * self.width as isize).rem_euclid((self.width * self.height) as isize) as usize]}

    pub fn update(&mut self) {

        for i in 0..self.width * self.height{
            let x = (i % self.width) as isize;
            let y = (i / self.width) as isize;
            let neightbors = [
                self.get(x -1, y -1), self.get(x +0, y -1), self.get(x +1,y -1),
                self.get(x -1, y +0), self.get(x +1, y +1), self.get(x +1,y +0),
                self.get(x -1, y +1), self.get(x +0, y +1), self.get(x +1,y +1),
            ];

            self.boards[self.future_board()][i].update(neightbors);
        }
    }
}


fn main() {

}

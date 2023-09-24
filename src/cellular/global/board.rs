
#[derive(Clone, Copy)]
enum TestCell{
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
struct Cell{
    unions: [Option<&'static Cell>; 4],
    ctype : TestCell,
}

#[derive(Clone, Copy)]
struct Clump{
}

#[derive(Clone)]
pub struct Board{
    width: isize,
    height: isize,
    cells: [[Cell; 16]; 16],
    clumps : Vec<Clump>,
}




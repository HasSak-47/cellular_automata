use tui::widgets::Widget;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LifeCell{
    Alive,
    #[default]
    Dead,
}

impl LifeCell{
    pub fn random() -> Self{
        Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board<T, const WIDTH: usize, const HEIGHT: usize>{
    board : [[T; WIDTH]; HEIGHT],
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Board<T, WIDTH, HEIGHT>
where
    T : Default + Copy,
{
    pub fn new() -> Self{
        Self{board: [[T::default(); WIDTH]; HEIGHT]}

    }
    pub fn random() -> Self{
        Self::new()
    }

    pub fn update(&mut self) {}
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Widget for &Board<T, WIDTH, HEIGHT>{
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
    }
}

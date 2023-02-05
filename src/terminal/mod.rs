use tui::buffer::Cell;
use tui::style::Color;

use crate::cellular::BasicCell;
use crate::cellular::Cell as OtherCell;

pub trait MakeBufferCell{
    fn make_buffer_cell(&self) -> Cell;
}

impl MakeBufferCell for BasicCell{
    fn make_buffer_cell(&self) -> Cell {
        let mut c = Cell::default();
        let color = if self.is(BasicCell::Alive) {Color::White} else {Color::Black};

        c.set_bg(color);
        c.set_fg(color);

        c
    }
}


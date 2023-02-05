pub use tui::buffer::Cell;
pub use tui::style::*;

pub trait MakeBufferCell{
    fn make_buffer_cell(&self) -> Cell;
}


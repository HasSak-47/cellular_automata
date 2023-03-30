use crate::terminal::{Color, MakeBufferCell, Cell as BufferCell};
use super::Cell;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum LifeCell{
    #[default]
    Dead,
    Alive
}

impl LifeCell{
    fn is_alive(&self) -> bool{
        match self {
            LifeCell::Dead => false,
            LifeCell::Alive => true,
        }
    }
}

impl MakeBufferCell for LifeCell{
    fn make_buffer_cell(&self) -> BufferCell {
        let mut c = BufferCell::default();
        let color = if self.is_alive() {Color::White} else {Color::Black};

        c.set_bg(color);
        c.set_fg(color);

        c
    }
}

impl std::fmt::Display for LifeCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         let p = ["  ", "$$"];
 
         let index: usize = match self {
             LifeCell::Dead => 0,
             LifeCell::Alive => 1,
         };
 
        write!(f, "{}", p[index])

    }
    
}

impl Cell for LifeCell{
    fn update(&self, neightbors: [&LifeCell; 8]) -> LifeCell{
        let mut live_neightbors = 0;
        for neightbor in neightbors{
            if neightbor.is_alive() {
                live_neightbors += 1;
            }
        }

        let mut new_cell = self.clone();
        if self.is_alive() {
            if live_neightbors < 2 || live_neightbors > 3 {new_cell = LifeCell::Dead;}
        }
        else{
            if live_neightbors == 3 {new_cell = LifeCell::Alive;}
        }

        new_cell
    }

    fn random() -> Self{
        let rand = rand::random::<i32>() % 2;
        match rand {
            x if x == 0 => Self::Dead, 
            _ => Self::Alive,
        }
    }
}

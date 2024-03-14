use std::fmt::Display;

use super::AutomataCell;
use rand::random;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FluidCell {
    Water,
    Air,
}

impl AutomataCell for FluidCell{
    fn step(&self, n: &[&Self]) -> Self {
        let upper = n[1];
        let under = n[7];
        use FluidCell as FC;

        match (upper,under){
            (FC::Water,FC::Water) => FC::Water,
            (FC::Water,FC::Air) => FC::Water,
            (FC::Air,FC::Water) => FC::Air,
            (FC::Air,FC::Air) => FC::Air,
            _ => self.clone(),
        }
    }
}

impl FluidCell{
    pub fn random() -> Self {
        match random::<bool>() {
            true => FluidCell::Water,
            false => FluidCell::Air,
        }
    }
}

impl Display for FluidCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // prints water as a white square
        write!(f, "{}", match self {
            FluidCell::Water => "■",
            FluidCell::Air => " ",
        })
    }
}

use std::fmt::Display;

use super::AutomataCell;
use rand::random;

#[derive(Clone, Eq, PartialEq, Default)]
pub enum LiveCell {
    Alive,
    #[default]
    Dead,
}

impl AutomataCell for LiveCell {
    fn step(&self, n: &[&Self]) -> Self {
        let alive = n.iter().filter(|x| ***x == LiveCell::Alive).count();
        match self {
            LiveCell::Alive => {
                if alive < 2 || alive > 3 {
                    LiveCell::Dead
                } else {
                    LiveCell::Alive
                }
            }
            LiveCell::Dead => {
                if alive == 3 {
                    LiveCell::Alive
                } else {
                    LiveCell::Dead
                }
            }
        }
    }
}

impl LiveCell{
    pub fn random() -> Self {
        match random::<bool>() {
            true => LiveCell::Alive,
            false => LiveCell::Dead,
        }
    }
}

impl Display for LiveCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // prints an alive cell as a white square
        write!(f, "{}", match self {
            LiveCell::Alive => "■",
            LiveCell::Dead => " ",
        })
    }
}

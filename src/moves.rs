use crate::board::{Position};

use std::convert::Into;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Submove {
    pub from: Position,
    pub to: Position,
}

impl Submove {
    pub fn new<T: Into<Position>>(from: T, to: T) -> Self {
        Submove {
            from: from.into(),
            to: to.into(),
        }
    }
}

impl Add for Submove {
    type Output = Submove;

    fn add(self, other: Submove) -> Submove {
        Submove {
            from: self.from + other.from,
            to: self.to + other.to,
        }
    }
}

impl Sub for Submove {
    type Output = Submove;

    fn sub(self, other: Submove) -> Submove {
        Submove {
            from: self.from - other.from,
            to: self.to - other.to,
        }
    }
}

pub type Moves = Vec<Submove>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submove_add() {
        let a = Submove::new((0, 0), (1, 1));
        let b = Submove::new((1, 1), (0, 0));
        let c = Submove::new((1, 1), (1, 1));
        assert_eq!(a + b, c);
    }

    #[test]
    fn submove_sub() {
        assert_eq!(
            Submove::new((2, 2), (2, 2)) - Submove::new((1, 1), (1, 1)),
            Submove::new((1, 1), (1, 1))
        );
    }
}

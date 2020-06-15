//! Management and matching of players

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn switch(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_white() {
        assert_eq!(Player::White.switch(), Player::Black);
    }

    #[test]
    fn switch_black() {
        assert_eq!(Player::Black.switch(), Player::White);
    }
}

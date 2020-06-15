//! # Notes
//! It may be nice to use a recursive `enum` structure for `Piece`, but but
//! [`box_syntax`](https://doc.rust-lang.org/unstable-book/language-features/box-syntax.html) and
//! [`box_patterns`](https://doc.rust-lang.org/unstable-book/language-features/box-patterns.html)
//! are still feature gated.

use crate::player::Player;

/// A `Piece` can either be a `Player` or a kinged `Player`
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Normal(Player),
    King(Player),
}

impl Piece {
    pub fn white() -> Self {
        Piece::Normal(Player::White)
    }
    pub fn black() -> Self {
        Piece::Normal(Player::Black)
    }
    pub fn white_king() -> Self {
        Piece::King(Player::White)
    }
    pub fn black_king() -> Self {
        Piece::King(Player::Black)
    }

    pub fn player(&self) -> Player {
        match *self {
            Piece::Normal(v) | Piece::King(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white() {
        assert_eq!(Piece::white(), Piece::Normal(Player::White));
    }

    #[test]
    fn black() {
        assert_eq!(Piece::black(), Piece::Normal(Player::Black));
    }

    #[test]
    fn black_king() {
        assert_eq!(Piece::black_king(), Piece::King(Player::Black));
    }

    #[test]
    fn white_king() {
        assert_eq!(Piece::white_king(), Piece::King(Player::White));
    }

    #[test]
    fn player() {
        let piece = Piece::King(Player::Black);
        assert_eq!(piece.player(), Player::Black);
    }
}

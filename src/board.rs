//! Board

use crate::moves::Submove;
use crate::piece::Piece;
use crate::player::Player;

use std::fmt;
use std::ops::{Add, Sub};

/// An American checkers game has a board width of 8.
pub const BOARD_WIDTH: usize = 8;

/// A `Point` can either be empty or contain a `Piece`
pub type Point = Option<Piece>;

/// An (x, y) coordinate representation of a position on the board
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position(x, y)
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position(self.0 - other.0, self.1 - other.1)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

impl From<(usize, usize)> for Position {
    fn from(t: (usize, usize)) -> Self {
        Position(t.0, t.1)
    }
}

impl From<(i32, i32)> for Position {
    fn from(t: (i32, i32)) -> Self {
        Position(t.0 as usize, t.1 as usize)
    }
}

impl From<(&i32, &i32)> for Position {
    fn from(t: (&i32, &i32)) -> Self {
        Position(*t.0 as usize, *t.1 as usize)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

type InternalBoard = [[Point; BOARD_WIDTH]; BOARD_WIDTH];

/// A board contains a two-dimensional vector of `Point`s
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub board: InternalBoard,
}

/// # Assumption
/// It is assumed that `Player::Black` is on the bottom and therefore occupying
/// starting indicies `[5, 6, 7]`. The
/// [`Board::norm()`](./struct.Board.html#method.norm) function rotates the
/// board in such a way that player is moving "up" the board (or negatively in
/// indicies).
impl Board {
    #[rustfmt::skip]
    pub fn new() -> Board {
        Board {
            board: [
                [ None,                 Some(Piece::white()), None,                 Some(Piece::white()), None,                 Some(Piece::white()), None,                 Some(Piece::white()) ],
                [ Some(Piece::white()), None,                 Some(Piece::white()), None,                 Some(Piece::white()), None,                 Some(Piece::white()), None ],
                [ None,                 Some(Piece::white()), None,                 Some(Piece::white()), None,                 Some(Piece::white()), None,                 Some(Piece::white()) ],
                [ None,                 None,                 None,                 None,                 None,                 None,                 None,                 None ],
                [ None,                 None,                 None,                 None,                 None,                 None,                 None,                 None ],
                [ Some(Piece::black()), None,                 Some(Piece::black()), None,                 Some(Piece::black()), None,                 Some(Piece::black()), None ],
                [ None,                 Some(Piece::black()), None,                 Some(Piece::black()), None,                 Some(Piece::black()), None,                 Some(Piece::black()) ],
                [ Some(Piece::black()), None,                 Some(Piece::black()), None,                 Some(Piece::black()), None,                 Some(Piece::black()), None ],
            ],
        }
    }

    /// Returns a board that is normalized from the
    /// [`Player`](../player/enum.Player.html)'s perspective.
    /// XXX: I'm not sure if this is changing the board in the right match or
    /// even if it ought to be mutable. This needs more thought.
    pub fn norm(&self, p: Player) -> Board {
        match p {
            Player::White => {
                let mut reversed_board = self.board;
                // Reverse order of rows
                reversed_board.reverse();
                // Reverse each row
                for (i, _) in reversed_board.clone().iter().enumerate() {
                    reversed_board[i].reverse();
                }
                Board {
                    board: reversed_board,
                }
            }
            _ => self.clone(),
        }
    }

    pub fn flip(&mut self) {
        let mut reversed_board = self.board;
        // Reverse order of rows
        reversed_board.reverse();
        // Reverse each row
        for (i, _) in reversed_board.clone().iter().enumerate() {
            reversed_board[i].reverse();
        }
        self.board = reversed_board;
    }

    pub fn at(&self, p: &Position) -> Point {
        self.board[p.0][p.1]
    }

    pub fn remove(&mut self, p: &Position) {
        self.board[p.0][p.1] = None
    }

    pub fn place(&mut self, x: Piece, p: &Position) {
        self.board[p.0][p.1] = Some(x)
    }

    /// Counts the number of pieces a player has on the board
    pub fn count_pieces(&self, p: Player) -> usize {
        self.board
            .iter()
            .flatten()
            .filter(|x| x.is_some())
            .filter(|x| x.unwrap().player() == p)
            .count()
    }

    /// TODO: This should be moved back into `Board` and replaced by a wrapper
    /// function.
    ///
    /// Validate a submove. This returns either `Ok(true)` or the first
    /// `Err(_)` to have occurred. Checking happens in the following manner.
    ///
    /// Each step of the validaiton is a closure so as to defer execution until a later time.
    ///
    /// Check that:
    /// 1. The piece being moved exists
    /// 2. The destination is empty
    /// 3. The piece being moved is owned by the player
    /// 4. The piece is moving forward if it is not kinged
    pub fn validate_submove(&self, s: &Submove, player: Player) -> Result<bool, String> {
        let piece_exists = |x: &Position| match self.norm(player).at(x).is_some() {
            false => Err(format!("No piece exists at {}.", x)),
            _ => Ok(true),
        };

        let dest_empty = |x: &Position| match self.norm(player).at(x).is_none() {
            false => Err(format!("Position {} is not empty.", x)),
            _ => Ok(true),
        };

        // `unwrap()` is safe because we know that the piece exists
        let owns_piece = |x: &Position| match self.norm(player).at(x).unwrap() {
            Piece::Normal(v) | Piece::King(v) => match v == player {
                false => Err(format!("Player {:?} does not own piece at {}.", player, x)),
                _ => Ok(true),
            },
        };

        let moving_forward = |s: &Submove| match self.norm(player).at(&s.from).unwrap() {
            Piece::Normal(_) => match s.from.0 > s.to.0 {
                false => Err(format!(
                    "Normal pieces must move forward: {} -> {}",
                    s.from, s.to
                )),
                _ => Ok(true),
            },
            _ => Ok(true),
        };

        // Reduce to `Ok(true)` or the first `Err`
        Ok(vec![
            piece_exists(&s.from)?,
            dest_empty(&s.to)?,
            owns_piece(&s.from)?,
            moving_forward(s)?,
        ]
        .iter()
        .all(|x| *x))
    }

    pub fn mutate(&mut self, s: &Submove) {
        let piece = self.at(&s.from).unwrap();
        self.remove(&s.from);
        self.remove(&s.to);
        self.place(piece, &s.to);
    }
}

/// A board can be specified by a series 'b', 'w', and '-' to specify black, white, and empty
/// pieces, respectively. Captial letters are used to denote kinged pieces.
impl From<&str> for Board {
    fn from(s: &str) -> Self {
        let mut board: InternalBoard = [[None; BOARD_WIDTH]; BOARD_WIDTH];
        for (i, c) in s.chars().enumerate() {
            let x = i / 8;
            let y = i % 8;
            match c {
                'w' | 'W' => board[x][y] = Some(Piece::white()),
                'b' | 'B' => board[x][y] = Some(Piece::black()),
                '-' | '·' => (),
                _ => panic!("Character '{}' is invalid.", c),
            }
        }
        Board { board }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_owned();
        for row in self.board.iter() {
            for tile in row.iter() {
                match tile {
                    None => s.push('·'),
                    Some(v) => {
                        s += match v {
                            Piece::Normal(Player::White) => "w",
                            Piece::Normal(Player::Black) => "b",
                            Piece::King(Player::White) => "W",
                            Piece::King(Player::Black) => "B",
                        }
                    }
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pass() {
        let s = "-w-w-w-w\
                 w-w-w-w-\
                 -w-w-w-w\
                 --------\
                 --------\
                 b-b-b-b-\
                 -b-b-b-b\
                 b-b-b-b-";
        let b = Board::from(s);
        assert_eq!(Board::new(), b);
    }

    #[test]
    #[should_panic]
    fn from_panic() {
        let s = "x";
        Board::from(s);
    }

    #[test]
    fn count_pieces() {
        assert_eq!(Board::new().count_pieces(Player::Black), 12);
    }

    #[test]
    fn norm() {
        let s = "-b-b-b-b\
                 b-b-b-b-\
                 -b-b-b-b\
                 --------\
                 --------\
                 w-w-w-w-\
                 -w-w-w-w\
                 w-w-w-w-";
        let b = Board::from(s);
        let norm = Board::new().norm(Player::White);
        assert_eq!(b, norm);
    }

    #[test]
    fn validate_submove_exists() {
        let b = Board::new();
        assert_eq!(
            b.validate_submove(&Submove::new((0, 0), (0, 0)), Player::Black),
            Err(String::from("No piece exists at (0, 0)."))
        );
    }

    #[test]
    fn validate_submove_empty() {
        let b = Board::new();
        assert_eq!(
            b.validate_submove(&Submove::new((6, 1), (6, 1)), Player::Black),
            Err(String::from("Position (6, 1) is not empty."))
        );
    }

    #[test]
    fn validate_submove_ok() {
        let b = Board::new();
        assert_eq!(
            b.validate_submove(&Submove::new((5, 0), (4, 1)), Player::Black),
            Ok(true)
        );
    }

    #[test]
    fn display() {
        let b = Board::new();
        println!("board::tests::display():\n{}", b);
    }

    #[test]
    fn position_add() {
        let a = Position(1, 0);
        let b = Position(1, 2);
        let c = Position(2, 2);
        assert_eq!(a + b, c);
    }
}

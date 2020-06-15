//! Management of game state

use crate::board::{Board, Position, BOARD_WIDTH};
use crate::moves::Submove;
use crate::piece::Piece;
use crate::player::Player;

pub const NAME: &str = "Rusted Checkers";

#[derive(Debug, PartialEq)]
pub struct Game {
    pub board: Board,
    pub player: Player,
    pub selected: Option<Position>,
    pub hilighted: Vec<Position>,
    pub nmoves: u8,
}

impl Game {
    #[allow(dead_code)]
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            player: Player::Black,
            selected: None,
            hilighted: Vec::new(),
            nmoves: 1,
        }
    }

    #[allow(dead_code)]
    fn validate_submove(&self, s: &Submove) -> Result<bool, String> {
        self.board.validate_submove(&s, self.player)
    }

    /// Checks the win condition for each player. This is to be run at the end of each turn.
    ///
    /// TODO: Add the case in which a player can make no possible moves.
    pub fn win_condition(&self) -> Option<Player> {
        if self.board.count_pieces(Player::Black) == 0 {
            Some(Player::White)
        } else if self.board.count_pieces(Player::White) == 0 {
            Some(Player::Black)
        } else {
            None
        }
    }

    /// Calculate all possible moves from position `p` for player `Game::Player`.
    pub fn gen_submoves(&mut self, p: Position) {
        // Calculate single space moves
        let directional_moves_from = |from: Position| match self.board.at(&p) {
            Some(Piece::King(_)) => {
                let neighbors = vec![(-1, -1), (-1, 1), (-1, 1), (1, 1)];
                neighbors
                    .iter()
                    .map(|(x, y)| (from.0 as i32 - x, from.1 as i32 - y))
                    .collect()
            }
            Some(Piece::Normal(_)) => {
                let neighbors = vec![(1, 1), (1, -1)];
                neighbors
                    .iter()
                    .map(|(x, y)| (from.0 as i32 - x, from.1 as i32 - y))
                    .collect()
            }
            _ => vec![],
        };

        // Filter out-of-bounds coordinates
        let filter_oob = |v: Vec<(i32, i32)>| {
            v.iter()
                .filter(|(x, y)| {
                    *x >= 0 && *x < BOARD_WIDTH as i32 && *y >= 0 && *y < BOARD_WIDTH as i32
                })
                .map(|(x, y)| (*x, *y))
                .collect()
        };

        // Generate initial possible moves
        let possible: Vec<(i32, i32)> = filter_oob(directional_moves_from(p));

        // Generate normal moves
        let mut normal_only: Vec<Position> = possible
            .iter()
            .filter(|&x| self.board.at(&Position::from(*x)).is_none())
            .map(|&x| Position::from(x))
            .collect();

        // Add normal moves to hilighted
        self.hilighted.append(&mut normal_only);

        // Get attacked pieces
        let attacked: Vec<(i32, i32)> = possible
            .iter()
            .filter(|&x| self.board.at(&Position::from(*x)).is_some())
            .filter(|&x| {
                self.board.at(&Position::from(*x)).unwrap().player() == self.player.switch()
            })
            .map(|&x| x)
            .collect();

        for &v in attacked.iter() {
            let diff = (v.0 as i32 - p.0 as i32, v.1 as i32 - p.1 as i32);
            let jump = Position::from((v.0 + diff.0, v.1 + diff.1));
            if !filter_oob(vec![(jump.0 as i32, jump.1 as i32)]).is_empty()
                && self.board.at(&jump).is_none()
            {
                self.hilighted.push(Position::from(jump));
            }
        }
    }

    pub fn select(&mut self, p: Position) {
        self.hilighted.clear();
        match self.board.at(&p) {
            Some(v) => {
                if v.player() == self.player {
                    self.selected = Some(p);
                    self.gen_submoves(p);
                }
            }
            _ => self.selected = None,
        }
    }

    /// # Steps
    /// 1. Validate submove
    /// 2. Mutate board
    /// 3. Change player
    pub fn do_submove(&mut self, s: &Submove) -> Result<bool, String> {
        // XXX: broken because of mutating board rotation
        //self.validate_submove(s)?;
        let piece = self.board.at(&s.from).unwrap();

        self.board.remove(&s.from);

        let diff = (
            s.to.0 as i32 - s.from.0 as i32,
            s.to.1 as i32 - s.from.1 as i32,
        );
        if diff.0.abs() > 1 || diff.1.abs() > 1 {
            // normalize diff
            let diff = (diff.0 / diff.0.abs(), diff.1 / diff.1.abs());
            // add diff to source
            let remove_pos = Position::from((s.from.0 as i32 + diff.0, s.from.1 as i32 + diff.1));
            self.board.remove(&remove_pos);
        } else {
            self.nmoves -= 1;
        }
        //check promotion. am not checking for piece type
        if((s.to.0 as i32) == 0){
            self.board.place(Piece::King(self.player), &s.to);
        }
        else{
            self.board.place(piece, &s.to);
        }

        if self.nmoves == 0 {
            self.player = self.player.switch();
            self.board.flip();
            self.nmoves = 1;
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_submove_move() {
        let mut g = Game::new();
        let submove = Submove::new((5, 0), (4, 1));
        let b = Board::from(
            "-b-b-b-b\
             b-b-b-b-\
             -b-b-b--\
             ------b-\
             --------\
             w-w-w-w-\
             -w-w-w-w\
             w-w-w-w-",
        );
        g.do_submove(&submove).unwrap();
        assert_eq!(
            g,
            Game {
                board: b,
                player: Player::White,
                selected: None,
                hilighted: Vec::new(),
                nmoves: 1,
            }
        );
    }

    #[test]
    fn validate_submove_exists() {
        let g = Game::new();
        assert_eq!(
            g.validate_submove(&Submove::new((0, 0), (0, 0))),
            Err(String::from("No piece exists at (0, 0)."))
        );
    }

    #[test]
    fn validate_submove_empty() {
        let g = Game::new();
        assert_eq!(
            g.validate_submove(&Submove::new((6, 1), (6, 1))),
            Err(String::from("Position (6, 1) is not empty."))
        );
    }

    #[test]
    fn validate_submove_ok() {
        let g = Game::new();
        assert_eq!(g.validate_submove(&Submove::new((7, 0), (0, 0))), Ok(true));
    }

    #[test]
    fn win_condition_black() {
        let mut g = Game::new();
        g.board = Board::from(
            "--------\
             --------\
             --------\
             --------\
             --------\
             b-b-b-b-\
             -b-b-b-b\
             b-b-b-b-");
        assert_eq!(g.win_condition(), Some(Player::Black));
    }

    #[test]
    fn win_condition_none() {
        let g = Game::new();
        assert_eq!(g.win_condition(), None);
    }

    #[test]
    fn gen_submoves_move() {
        let mut g = Game::new();
        g.gen_submoves(Position::new(5, 0));
        assert_eq!(g.hilighted, vec![Position::new(4, 1)]);
    }
}

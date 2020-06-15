pub mod board;
pub mod game;
pub mod moves;
pub mod networking;
pub mod piece;
pub mod player;

#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate bincode;

use docopt::Docopt;
use networking::{client, server};

const USAGE: &'static str = "
Rusted Checkers

Usage:
  rustedcheckers
  rustedcheckers server
  rustedcheckers client
  rustedcheckers (-h | --help)
  rustedcheckers --version

Options:
  -h --help     Show this screen.
  -v --version  Show version.
";


extern crate ncurses;

use crate::board::{Board, Position, BOARD_WIDTH};
use crate::game::Game;
use crate::moves::Submove;
use crate::piece::Piece;
use crate::player::Player;

use libc::{c_int, c_short};
use ncurses::constants::{COLOR_BLACK, COLOR_WHITE};
use ncurses::*;

use std::char;
use std::str;

#[derive(Debug, Deserialize)]
struct Args {
    cmd_server: bool,
    cmd_client: bool,
}

// Individual Colors
static COLOR_BACKGROUND: i16 = 230;
static COLOR_FOREGROUND: i16 = COLOR_BLACK;
static COLOR_RED: i16 = 9;
static COLOR_BLUE: i16 = 12;
static COLOR_HI: i16 = 3;
static COLOR_GREEN: i16 = 3;

// Color Pairs
static COLOR_PAIR_DEFAULT: i16 = 1;
static COLOR_PAIR_RED_ON_BLACK: i16 = 2;
static COLOR_PAIR_BLUE_ON_BLACK: i16 = 3;
static COLOR_PAIR_RED_ON_WHITE: i16 = 4;
static COLOR_PAIR_BLUE_ON_WHITE: i16 = 5;
static COLOR_PAIR_WHITE: i16 = 6;
static COLOR_PAIR_BLACK: i16 = 7;
static COLOR_PAIR_RED_ON_BKGD: i16 = 8;
static COLOR_PAIR_BLUE_ON_BKGD: i16 = 9;
static COLOR_PAIR_RED_HI: i16 = 10;
static COLOR_PAIR_BLUE_HI: i16 = 11;
static COLOR_PAIR_EMPTY_HI: i16 = 12;

// Window Positioning
static TITLE_POS: (i32, i32) = (0, 0);
static TITLE_H: i32 = 3;
static BOARD_POS: (i32, i32) = (TITLE_POS.0 + TITLE_H, 0);
static BOARD_H: i32 = BOARD_WIDTH as i32 + 2;
static BOARD_W: i32 = BOARD_WIDTH as i32 * 2 + 2;
static CAPTURE_POS: (i32, i32) = (BOARD_POS.0 + BOARD_H, 0);
static CAPTURE_H: i32 = 4;
static CAPTURE_W: i32 = 2 + 2 + 1 + 12 * 2;

/// Creates, refreshes, and returns a bordered window
fn create_win(height: i32, width: i32, start_y: i32, start_x: i32) -> WINDOW {
    let win = newwin(height, width, start_y, start_x);
    wbkgd(win, COLOR_PAIR(COLOR_PAIR_DEFAULT));
    box_(win, 0, 0);
    wrefresh(win);
    win
}

/// Destroys and refreshes a specified window
fn destroy_win(win: WINDOW) {
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}

fn draw_captured(win: WINDOW, board: &Board) {
    // Get number of captured pieces
    let w_cap = 12 - board.count_pieces(Player::White);
    let b_cap = 12 - board.count_pieces(Player::Black);

    // Title
    mvwaddstr(win, 0, 1, "┤Captured├");

    // Draw captured red pieces
    wcolor_set(win, COLOR_PAIR_RED_ON_BKGD);
    mvwaddstr(win, 1, 1, &w_cap.to_string());
    if w_cap > 0 {
        for x in 2..(w_cap + 2) {
            mvwaddstr(win, 1, x as i32 * 2, "⬤ ");
        }
    }

    // Draw captured blue pieces
    wcolor_set(win, COLOR_PAIR_BLUE_ON_BKGD);
    mvwaddstr(win, 2, 1, &b_cap.to_string());
    if b_cap > 0 {
        for y in 2..(b_cap + 2) {
            mvwaddstr(win, 2, y as i32 * 2, "⬤ ");
        }
    }

    wrefresh(win);
}

fn draw_board(win: WINDOW, game: &mut Game) {
    // Title
    mvwaddstr(win, 0, 1, "┤Board├");

    // Board
    for (i, x) in game.board.board.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            match y {
                Some(Piece::Normal(Player::White)) => {
                    if (i + j) % 2 == 1 {
                        wcolor_set(win, COLOR_PAIR_RED_ON_BLACK);
                    } else {
                        wcolor_set(win, COLOR_PAIR_RED_ON_WHITE);
                    }
                }
                Some(Piece::Normal(Player::Black)) => {
                    if (i + j) % 2 == 1 {
                        wcolor_set(win, COLOR_PAIR_BLUE_ON_BLACK);
                    } else {
                        wcolor_set(win, COLOR_PAIR_BLUE_ON_WHITE);
                    }
                }
                Some(Piece::King(Player::White)) => {
                    if (i + j) % 2 == 1 {
                        wcolor_set(win, COLOR_PAIR_RED_ON_BLACK);
                    } else {
                        wcolor_set(win, COLOR_PAIR_RED_ON_WHITE);
                    }
                }
                Some(Piece::King(Player::Black)) => {
                    if (i + j) % 2 == 1 {
                        wcolor_set(win, COLOR_PAIR_BLUE_ON_BLACK);
                    } else {
                        wcolor_set(win, COLOR_PAIR_BLUE_ON_WHITE);
                    }
                }
                _ => {
                    if (i + j) % 2 == 1 {
                        wcolor_set(win, COLOR_PAIR_BLACK);
                    } else {
                        wcolor_set(win, COLOR_PAIR_WHITE);
                    }
                }
            }

            match &game.selected {
                Some(p) => {
                    if Position::new(i, j) == *p {
                        match game.board.at(p) {
                            Some(Piece::Normal(Player::Black)) => {
                                wcolor_set(win, COLOR_PAIR_BLUE_HI)
                            }
                            Some(Piece::Normal(Player::White)) => {
                                wcolor_set(win, COLOR_PAIR_RED_HI)
                            }
                            Some(Piece::King(Player::Black)) => {
                                wcolor_set(win, COLOR_PAIR_BLUE_HI)
                            }
                            Some(Piece::King(Player::White)) => {
                                wcolor_set(win, COLOR_PAIR_RED_HI)
                            }
                            _ => 0,
                        };
                    }
                }
                _ => (),
            }

            if game.hilighted.contains(&Position::new(i, j)) {
                wcolor_set(win, COLOR_PAIR_EMPTY_HI);
                mvwaddstr(win, i as i32 + 1, j as i32 * 2 + 1, "  ");
            } else {
                   match y{
                            Some(Piece::King(_)) => {
                                mvwaddstr(win, i as i32 + 1, j as i32 * 2 + 1, "❤");
                            }
                            _ => {
                                mvwaddstr(win, i as i32 + 1, j as i32 * 2 + 1, "⬤ ");
                            },
                        };
            }
            wattroff(win, A_DIM());
        }
    }

    // Reset & Refresh
    wcolor_set(win, COLOR_PAIR_DEFAULT);
    wrefresh(win);
}

const fn new_mevent() -> MEVENT {
    MEVENT {
        id: 0 as c_short,
        x: 0 as c_int,
        y: 0 as c_int,
        z: 0 as c_int,
        bstate: 0 as u64,
    }
}

fn clearline(line: i32) {
    mv(line, 0);
    clrtoeol();
}

fn main() {
    // Unicode
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8");

    // ncurses init
    initscr();
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);

    // Get screen size
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    // Coloring
    start_color();
    init_pair(COLOR_PAIR_DEFAULT, COLOR_FOREGROUND, COLOR_BACKGROUND);
    init_pair(COLOR_PAIR_RED_ON_BLACK, COLOR_RED, COLOR_BLACK);
    init_pair(COLOR_PAIR_BLUE_ON_BLACK, COLOR_BLUE, COLOR_BLACK);
    init_pair(COLOR_PAIR_RED_ON_WHITE, COLOR_RED, COLOR_WHITE);
    init_pair(COLOR_PAIR_BLUE_ON_WHITE, COLOR_BLUE, COLOR_WHITE);
    init_pair(COLOR_PAIR_WHITE, COLOR_WHITE, COLOR_WHITE);
    init_pair(COLOR_PAIR_BLACK, COLOR_BLACK, COLOR_BLACK);
    init_pair(COLOR_PAIR_BLUE_ON_BKGD, COLOR_BLUE, COLOR_BACKGROUND);
    init_pair(COLOR_PAIR_RED_ON_BKGD, COLOR_RED, COLOR_BACKGROUND);
    init_pair(COLOR_PAIR_RED_HI, COLOR_RED, COLOR_HI);
    init_pair(COLOR_PAIR_BLUE_HI, COLOR_BLUE, COLOR_HI);
    init_pair(COLOR_PAIR_EMPTY_HI, COLOR_WHITE, COLOR_GREEN);
    bkgd(' ' as chtype | COLOR_PAIR(COLOR_PAIR_DEFAULT) as chtype);

    // Mouse
    mousemask(ALL_MOUSE_EVENTS as mmask_t, None);

    refresh();

    // Title
    mvaddstr(
        0,
        0,
        "╔═════════════════╗",
    );
    mvaddstr(1, 0, "║ Rusted Checkers ║");
    mvaddstr(
        2,
        0,
        "╚═════════════════╝",
    );

    //Create test board
    let mut game = Game::new();
    //game.board.mutate(&Submove::new((5, 0), (2, 1)));
    //game.board.mutate(&Submove::new((2, 1), (1, 2)));
    //game.board.mutate(&Submove::new((2, 3), (5, 2)));
    game.board.mutate(&Submove::new((2, 1), (4, 1)));
    //game.board.mutate(&Submove::new((1, 4), (4, 1)));

    // Create wide window for the board
    let board_win = create_win(BOARD_H, BOARD_W, BOARD_POS.0, BOARD_POS.1);
    draw_board(board_win, &mut game);

    // Create captured window
    let captured_win = create_win(CAPTURE_H, CAPTURE_W, CAPTURE_POS.0, CAPTURE_POS.1);
    draw_captured(captured_win, &game.board);

    refresh();

    // Input
    'main: loop {
        let ch = wget_wch(stdscr());
        match ch {
            Some(WchResult::KeyCode(KEY_MOUSE)) => {
                // Get stdscr coords
                let mut mevent = new_mevent();
                let err_code = getmouse(&mut mevent);
                if err_code == 0 {
                    mvaddstr(LINES() - 1, 0, format!("{:?}", mevent).as_str());
                } else {
                    panic!("Couldn't get mouse event.");
                }
                // Normalize to board_win
                let xs: &mut [i32] = &mut [mevent.x];
                let ys: &mut [i32] = &mut [mevent.y];
                let in_win = wmouse_trafo(board_win, ys, xs, false);
                if in_win {
                    let selected_pos =
                        Position::new((ys[0] - 1) as usize, ((xs[0] - 1) / 2) as usize);
                    if game.hilighted.contains(&selected_pos) {
                        let res = game.do_submove(&Submove {
                            from: game.selected.unwrap(),
                            to: selected_pos,
                        });
                        if res.is_err() {
                            mvaddstr(LINES() - 5, 0, res.err().unwrap().as_str());
                        } else {
                            game.hilighted.clear();
                            draw_board(board_win, &mut game);
                            draw_captured(captured_win, &game.board);
                        }
                    } else {
                        game.select(selected_pos);
                        // Select clicked piece
                        game.select(Position::new(
                            (ys[0] - 1) as usize,
                            ((xs[0] - 1) / 2) as usize,
                        ));
                        //game.selected = Some(Position::new(0, 1));
                        mvaddstr(
                            LINES() - 4,
                            0,
                            format!("Selected: {:?}", game.selected).as_str(),
                        );
                        draw_board(board_win, &mut game);
                    }
                }
            }
            Some(WchResult::KeyCode(_)) => {
                let line = LINES() - 3;
                clearline(line);
                mvaddstr(line, 0, "Keycode pressed.");
            }
            Some(WchResult::Char(c)) => {
                /* Enable attributes and output message. */
                addstr("\nKey pressed: ");
                attron(A_BOLD() | A_BLINK());
                addstr(format!("{}\n", char::from_u32(c as u32).expect("Invalid char")).as_ref());
                attroff(A_BOLD() | A_BLINK());
                match char::from_u32(c as u32).unwrap() {
                    'q' => break 'main,
                    _ => (),
                }
            }
            _ => (),
        }
    }

    destroy_win(board_win);
    endwin();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _main() {
        main();
    }
}

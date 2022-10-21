use super::board::*;
use super::super::enums::{piece_color::*, end_type::*};
use std::collections::HashMap;

pub type BitBoard = [u64; 12];
pub type BitBoardMove = (((u64, u64), (u64, u64)), BitBoard);

pub enum PieceNum {
    WhitePawn = 0,
    WhiteRook = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackRook = 7,
    BlackKnight = 8,
    BlackBishop = 9,
    BlackQueen = 10,
    BlackKing = 11
}

#[derive(Clone)]
pub struct Constants {
    pub start_board: BitBoard,
    pub row_and_column_mask: [u64; 64],
    pub diagonal_mask: [u64; 64],
    pub pawn_reach: [[u64; 64]; 2],
    pub rook_reach: [HashMap<u64, u64>; 64],
    pub knight_reach: [u64; 64],
    pub bishop_reach: [HashMap<u64, u64>; 64],
    pub king_reach: [u64; 64]
}

impl Constants {
    pub fn new() -> Self {
        Self {
            start_board: generate_start_board(),
            row_and_column_mask: generate_row_and_column_mask(),
            diagonal_mask: generate_diagonal_mask(),
            pawn_reach: generate_pawn_reach(),
            rook_reach: generate_rook_reach(),
            knight_reach: generate_knight_reach(),
            bishop_reach: generate_bishop_reach(),
            king_reach: generate_king_reach()
        }
    }
}

pub fn generate_start_board() -> BitBoard {
    let mut board = [0; 12];

    // White pawns
    for i in 0..8 {
        board[PieceNum::WhitePawn as usize] += 1 << pos_to_num(i, 1);
    }

    // White rooks
    board[PieceNum::WhiteRook as usize] += 1 << pos_to_num(0, 0);
    board[PieceNum::WhiteRook as usize] += 1 << pos_to_num(7, 0);

    // White knights
    board[PieceNum::WhiteKnight as usize] += 1 << pos_to_num(1, 0);
    board[PieceNum::WhiteKnight as usize] += 1 << pos_to_num(6, 0);

    // White bishops
    board[PieceNum::WhiteBishop as usize] += 1 << pos_to_num(2, 0);
    board[PieceNum::WhiteBishop as usize] += 1 << pos_to_num(5, 0);

    // White queen
    board[PieceNum::WhiteQueen as usize] += 1 << pos_to_num(3, 0);

    // White king
    board[PieceNum::WhiteKing as usize] += 1 << pos_to_num(4, 0);

    // Black pawns
    for i in 0..8 {
        board[PieceNum::BlackPawn as usize] += 1 << pos_to_num(i, 6);
    }

    // Black rooks
    board[PieceNum::BlackRook as usize] += 1 << pos_to_num(0, 7);
    board[PieceNum::BlackRook as usize] += 1 << pos_to_num(7, 7);

    // Black knights
    board[PieceNum::BlackKnight as usize] += 1 << pos_to_num(1, 7);
    board[PieceNum::BlackKnight as usize] += 1 << pos_to_num(6, 7);

    // Black bishops
    board[PieceNum::BlackBishop as usize] += 1 << pos_to_num(2, 7);
    board[PieceNum::BlackBishop as usize] += 1 << pos_to_num(5, 7);

    // Black queen
    board[PieceNum::BlackQueen as usize] += 1 << pos_to_num(3, 7);

    // Black king
    board[PieceNum::BlackKing as usize] += 1 << pos_to_num(4, 7);

    board
}

fn generate_row_and_column_mask() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut tmp = 0;

        // Up
        for j in number + 1..8 {
            tmp += 1 << pos_to_num(letter, j);
        }

        // Down
        for j in 0..number {
            tmp += 1 << pos_to_num(letter, j);
        }

        // Left
        for j in 0..letter {
            tmp += 1 << pos_to_num(j, number);
        }

        // Right
        for j in letter + 1..8 {
            tmp += 1 << pos_to_num(j, number);
        }

        possible_moves[i as usize] = tmp;
    }


    possible_moves
}

fn generate_diagonal_mask() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut tmp = 0;

        // Up-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        // Down-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        // Down-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        // Up-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        possible_moves[i as usize] = tmp;
    }


    possible_moves
}

// First = White, Second = Black
fn generate_pawn_reach() -> [[u64; 64]; 2] {
    let mut possible_moves = [[0; 64]; 2];
    let side_constants = [1, -1];

    for side in side_constants {
        for i in 0..64 {
            let (letter, number) = num_to_pos(i);
            let mut b = 0;

            // Right
            let new_letter = letter as i32 + 1;
            let new_number = number as i32 + side;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                b += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }

            // Left
            let new_letter = letter as i32 - 1;
            let new_number = number as i32 + side;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                b += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }

            if side == 1 {
                possible_moves[0][i as usize] = b;
            } else {
                possible_moves[1][i as usize] = b;
            }
        }
    }

    possible_moves
}

fn generate_rook_reach() -> [HashMap<u64, u64>; 64] {
    let mut possible_moves: [HashMap<u64, u64>; 64] = [(); 64].map(|_| HashMap::with_capacity(16384));

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut possibilities = Vec::with_capacity(16384);
        possibilities.push(0);
        let mut points = Vec::with_capacity(14);

        // Up
        for j in number + 1..8 {
            points.push(pos_to_num(letter, j));
        }

        // Down
        for j in 0..number {
            points.push(pos_to_num(letter, j));
        }

        // Left
        for j in 0..letter {
            points.push(pos_to_num(j, number));
        }

        // Right
        for j in letter + 1..8 {
            points.push(pos_to_num(j, number));
        }

        let mut board = 0;
        let mut count = 0;
        while count < points.len() {
            let num_pos = points[count];
            if board & (1 << num_pos) == 0 {
                board += 1 << num_pos;
                for l in 0..count {
                    let num_pos = points[l];
                    board -= 1 << num_pos;
                }
                possibilities.push(board);
                count = 0;
            } else {
                count += 1;
            }
        }

        for p in possibilities {
            let mut board = 0;

            // Up
            for j in number + 1..8 {
                let pos = 1 << pos_to_num(letter, j);
                board += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Down
            for j in (0..number).rev() {
                let pos = 1 << pos_to_num(letter, j);
                board += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Left
            for j in (0..letter).rev() {
                let pos = 1 << pos_to_num(j, number);
                board += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Right
            for j in letter + 1..8 {
                let pos = 1 << pos_to_num(j, number);
                board += pos;
                if p & pos == pos {
                    break;
                }
            }

            possible_moves[i as usize].insert(p, board);
        }
    }

    possible_moves
}

fn generate_knight_reach() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    // Clockwise starting from up-right
    let position_diffs = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut board = 0;

        for diffs in position_diffs {
            let (letter_diff, number_diff) = diffs;
            let new_letter = letter as i32 + letter_diff;
            let new_number = number as i32 + number_diff;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                board += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }
        }

        possible_moves[i as usize] = board;
    }

    possible_moves
}

fn generate_bishop_reach() -> [HashMap<u64, u64>; 64] {
    let mut possible_moves: [HashMap<u64, u64>; 64] = [(); 64].map(|_| HashMap::with_capacity(16384));

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut possibilities = Vec::with_capacity(16384);
        possibilities.push(0);
        let mut points = Vec::with_capacity(14);

        // Up-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        // Down-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        // Down-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        // Up-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        let mut board = 0;
        let mut count = 0;
        while count < points.len() {
            let num_pos = points[count];
            if board & (1 << num_pos) == 0 {
                board += 1 << num_pos;
                for l in 0..count {
                    let num_pos = points[l];
                    board -= 1 << num_pos;
                }
                possibilities.push(board);
                count = 0;
            } else {
                count += 1;
            }
        }

        for p in possibilities {
            let mut b = 0;

            // Up-right
            for j in 1..7 {
                let new_letter = letter as i32 + j;
                let new_number = number as i32 + j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Down-right
            for j in 1..7 {
                let new_letter = letter as i32 + j;
                let new_number = number as i32 - j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Down-left
            for j in 1..7 {
                let new_letter = letter as i32 - j;
                let new_number = number as i32 - j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Up-left
            for j in 1..7 {
                let new_letter = letter as i32 - j;
                let new_number = number as i32 + j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            possible_moves[i as usize].insert(p, b);
        }
    }

    possible_moves
}

fn generate_king_reach() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    // Clockwise starting from up-right
    let position_diffs = [(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut board = 0;

        for diffs in position_diffs {
            let (letter_diff, number_diff) = diffs;
            let new_letter = letter as i32 + letter_diff;
            let new_number = number as i32 + number_diff;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                board += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }
        }

        possible_moves[i as usize] = board;
    }

    possible_moves
}

pub fn print_bitboard(num: u64) {
    let board = format!("{:#066b}", num).chars().rev().collect::<String>();
    for i in (0..8).rev() {
        println!("{}", board[i*8..i*8+8].to_string());
    }
}

pub fn pos_to_num(letter: u64, number: u64) -> u64 {
    letter + (number << 3)
}

pub fn num_to_pos(num: u64) -> (u64, u64) {
    (num & 7, num >> 3)
}

fn get_full_color_board(board: &BitBoard, color: PieceColor) -> u64 {
    let mut possible_moves: u64 = 0;
    match color {
        PieceColor::White => {
            possible_moves |= board[PieceNum::WhitePawn as usize];
            possible_moves |= board[PieceNum::WhiteRook as usize];
            possible_moves |= board[PieceNum::WhiteKnight as usize];
            possible_moves |= board[PieceNum::WhiteBishop as usize];
            possible_moves |= board[PieceNum::WhiteQueen as usize];
            possible_moves |= board[PieceNum::WhiteKing as usize];
        },
        PieceColor::Black => {
            possible_moves |= board[PieceNum::BlackPawn as usize];
            possible_moves |= board[PieceNum::BlackRook as usize];
            possible_moves |= board[PieceNum::BlackKnight as usize];
            possible_moves |= board[PieceNum::BlackBishop as usize];
            possible_moves |= board[PieceNum::BlackQueen as usize];
            possible_moves |= board[PieceNum::BlackKing as usize];
        }
    }
    possible_moves
}

fn get_occupied_board(board: &BitBoard) -> u64 {
    get_full_color_board(board, PieceColor::White) | get_full_color_board(board, PieceColor::Black)
}

pub fn get_reach_board(board: &BitBoard, color: PieceColor, constants: &Constants) -> u64 {
    let occupied_board = get_occupied_board(board);

    let mut b = 0;

    match color {
        PieceColor::White => {
            for i in 0..64 {
                let num = 1 << i;
                let rows_and_columns = occupied_board & constants.row_and_column_mask[i];
                let diagonals = occupied_board & constants.diagonal_mask[i];

                if board[PieceNum::WhitePawn as usize] & num == num {
                    b |= constants.pawn_reach[0][i];
                } else if board[PieceNum::WhiteRook as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                } else if board[PieceNum::WhiteKnight as usize] & num == num {
                    b |= constants.knight_reach[i];
                } else if board[PieceNum::WhiteBishop as usize] & num == num {
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::WhiteQueen as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::WhiteKing as usize] & num == num {
                    b |= constants.king_reach[i];
                }
            }
        },
        PieceColor::Black => {
            for i in 0..64 {
                let num = 1 << i;
                let rows_and_columns = occupied_board & constants.row_and_column_mask[i];
                let diagonals = occupied_board & constants.diagonal_mask[i];

                if board[PieceNum::BlackPawn as usize] & num == num {
                    b |= constants.pawn_reach[1][i];
                } else if board[PieceNum::BlackRook as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                } else if board[PieceNum::BlackKnight as usize] & num == num {
                    b |= constants.knight_reach[i];
                } else if board[PieceNum::BlackBishop as usize] & num == num {
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::BlackQueen as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::BlackKing as usize] & num == num {
                    b |= constants.king_reach[i];
                }
            }
        }
    }

    b
}

fn is_in_check(board: &BitBoard, color: PieceColor, constants: &Constants) -> bool {
    let opposite_reach_board = get_reach_board(&board, color.opposite_color(), &constants);
    match color {
        PieceColor::White => {
            let king_num = board[PieceNum::WhiteKing as usize];
            opposite_reach_board & king_num == king_num
        },
        PieceColor::Black => {
            let king_num = board[PieceNum::BlackKing as usize];
            opposite_reach_board & king_num == king_num
        }
    }
}

pub fn generate_possible_moves(board: &BitBoard, prev_board: Option<&BitBoard>, color: PieceColor, constants: &Constants) -> Vec<BitBoardMove> {
    let mut possible_moves = Vec::new();

    let opposite_color = color.opposite_color();
    let occupied_board = get_occupied_board(&board);
    let own_pieces = get_full_color_board(&board, color);
    let opposite_pieces = get_full_color_board(&board, opposite_color);
    let reach_board = get_reach_board(&board, color, &constants);
    let opposite_reach_board = get_reach_board(&board, opposite_color, &constants);

    match color {
        PieceColor::White => {
            // Pawns
            let mut tmp = board[PieceNum::WhitePawn as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                if occupied_board & 1 << (i + 8) == 0 {
                    let mut new_board = board.clone();
                    new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                    if 55 <= i + 8 && i + 8 < 64 {
                        new_board[PieceNum::WhiteQueen as usize] += 1 << (i + 8);
                    } else {
                        new_board[PieceNum::WhitePawn as usize] += 1 << (i + 8);
                    }
                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 + 8)), new_board));

                    if 7 <= i && i < 16 && occupied_board & 1 << (i + 16) == 0 {
                        let mut new_board = board.clone();
                        new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                        new_board[PieceNum::WhitePawn as usize] += 1 << (i + 16);
                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 + 16)), new_board));
                    }
                }

                let mut possible_attacks = constants.pawn_reach[0][i as usize] & opposite_pieces;
                while possible_attacks != 0 {
                    let j = possible_attacks.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                    new_board[PieceNum::WhitePawn as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    possible_attacks -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Rooks
            let mut tmp = board[PieceNum::WhiteRook as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteRook as usize] -= 1 << i;
                    new_board[PieceNum::WhiteRook as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Knights
            let mut tmp = board[PieceNum::WhiteKnight as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.knight_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteKnight as usize] -= 1 << i;
                    new_board[PieceNum::WhiteKnight as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Bishops
            let mut tmp = board[PieceNum::WhiteBishop as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteBishop as usize] -= 1 << i;
                    new_board[PieceNum::WhiteBishop as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Queen
            let mut tmp = board[PieceNum::WhiteQueen as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable_straight = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let reachable_diagonals = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let reachable = reachable_straight | reachable_diagonals;
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteQueen as usize] -= 1 << i;
                    new_board[PieceNum::WhiteQueen as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // King
            let mut tmp = board[PieceNum::WhiteKing as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.king_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteKing as usize] -= 1 << i;
                    new_board[PieceNum::WhiteKing as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }
        },
        PieceColor::Black => {
            // Pawns
            let mut tmp = board[PieceNum::BlackPawn as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                if occupied_board & 1 << (i - 8) == 0 {
                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                    if 0 <= i - 8 && i - 8 < 8 {
                        new_board[PieceNum::BlackQueen as usize] += 1 << (i - 8);
                    } else {
                        new_board[PieceNum::BlackPawn as usize] += 1 << (i - 8);
                    }
                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 - 8)), new_board));

                    if 48 <= i && i < 56 && occupied_board & 1 << (i - 16) == 0 {
                        let mut new_board = board.clone();
                        new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                        new_board[PieceNum::BlackPawn as usize] += 1 << (i - 16);
                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 - 16)), new_board));
                    }
                }

                let mut possible_attacks = constants.pawn_reach[1][i as usize] & opposite_pieces;
                while possible_attacks != 0 {
                    let j = possible_attacks.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                    new_board[PieceNum::BlackPawn as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    possible_attacks -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Rooks
            let mut tmp = board[PieceNum::BlackRook as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackRook as usize] -= 1 << i;
                    new_board[PieceNum::BlackRook as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Knights
            let mut tmp = board[PieceNum::BlackKnight as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.knight_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackKnight as usize] -= 1 << i;
                    new_board[PieceNum::BlackKnight as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Bishops
            let mut tmp = board[PieceNum::BlackBishop as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackBishop as usize] -= 1 << i;
                    new_board[PieceNum::BlackBishop as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Queen
            let mut tmp = board[PieceNum::BlackQueen as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable_straight = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let reachable_diagonals = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let reachable = reachable_straight | reachable_diagonals;
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackQueen as usize] -= 1 << i;
                    new_board[PieceNum::BlackQueen as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // King
            let mut tmp = board[PieceNum::BlackKing as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.king_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackKing as usize] -= 1 << i;
                    new_board[PieceNum::BlackKing as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }
        }
    }

    // Validate
    let mut res = Vec::new();
    for mov in possible_moves {
        let b = mov.1;

        if !is_in_check(&b, color, &constants) {
            res.push(mov);
        }
    }

    res
}

fn get_piece_str(board: &BitBoard, letter: u64, number: u64) -> &str{
    let num = 1 << pos_to_num(letter, number);

    if board[PieceNum::WhitePawn as usize] & num == num {
        "\u{265F}"
    } else if board[PieceNum::WhiteRook as usize] & num == num {
        "\u{265C} "
    } else if board[PieceNum::WhiteKnight as usize] & num == num {
        "\u{265E} "
    } else if board[PieceNum::WhiteBishop as usize] & num == num {
        "\u{265D} "
    } else if board[PieceNum::WhiteQueen as usize] & num == num {
        "\u{265B} "
    } else if board[PieceNum::WhiteKing as usize] & num == num {
        "\u{265A} "
    } else if board[PieceNum::BlackPawn as usize] & num == num {
        "\u{2659} "
    } else if board[PieceNum::BlackRook as usize] & num == num {
        "\u{2656} "
    } else if board[PieceNum::BlackKnight as usize] & num == num {
        "\u{2658} "
    } else if board[PieceNum::BlackBishop as usize] & num == num {
        "\u{2657} "
    } else if board[PieceNum::BlackQueen as usize] & num == num {
        "\u{2655} "
    } else if board[PieceNum::BlackKing as usize] & num == num {
        "\u{2654} "
    } else {
        "  "
    }
}

pub fn get_bitboard_ascii(board: &BitBoard) -> String {
    let mut string = String::with_capacity(844);
    
    for i in (0..8).rev() {
        string.push_str("  +----+----+----+----+----+----+----+----+\n");
        if let Some(res) = std::char::from_digit(1 + i as u32, 10) {
            string.push(res);
            string.push(' ');
        }
        for j in 0..8 {
            string.push_str("| ");
            string.push_str(get_piece_str(&board, j as u64, i as u64));
            string.push(' ');
        }
        string.push_str("|\n");
    }
    string.push_str("  +----+----+----+----+----+----+----+----+\n");
    string.push_str("    A    B    C    D    E    F    G    H");

    string
}

pub fn board_to_bitboard(board: &ChessBoard) -> BitBoard{
    let mut res = [0; 12];
    for i in 0..8 {
        for j in 0..8 {
            if let Ok(Some(piece)) = board.get_piece(i, j) {
                match piece.color {
                    PieceColor::White => {
                        match piece.typ {
                            PieceType::Pawn => {
                                res[0] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Rook => {
                                res[1] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Knight => {
                                res[2] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Bishop => {
                                res[3] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Queen => {
                                res[4] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::King => {
                                res[5] += 1 << pos_to_num(i as u64, j as u64);
                            }
                        }
                    },
                    PieceColor::Black => {
                        match piece.typ {
                            PieceType::Pawn => {
                                res[6] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Rook => {
                                res[7] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Knight => {
                                res[8] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Bishop => {
                                res[9] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Queen => {
                                res[10] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::King => {
                                res[11] += 1 << pos_to_num(i as u64, j as u64);
                            }
                        }
                    }
                }
            }
        }
    }

    res
}

pub fn bitboard_check_game_end(bb: &BitBoard, prev_board: Option<&BitBoard>, turn: PieceColor, constants: &Constants) -> EndType {
    let opponent_color = turn.opposite_color();
    let possible_moves = generate_possible_moves(bb, prev_board, turn, &constants);

    if possible_moves.len() == 0 {
        let opponent_reach = get_reach_board(&bb, opponent_color, &constants);

        let checkmate = match turn {
            PieceColor::White => opponent_reach & bb[5] == bb[5],
            PieceColor::Black => opponent_reach & bb[11] == bb[11]
        };

        if checkmate {
            return EndType::Checkmate;
        } else {
            return EndType::Tie;
        }
    }

    EndType::NoEnd
}

mod evaluate;
mod generate_moves;
mod move_piece;

use evaluate::evaluate;
use generate_moves::generate_moves;
use move_piece::move_piece;
use std::{
    i32,
    io::{self, Write},
};

fn square_to_index(square: &str) -> u8 {
    let file = square.as_bytes()[0] - b'a';
    let rank = square.as_bytes()[1] - b'1';

    rank * 8 + file
}

fn index_to_square(index: u8) -> String {
    let file = (index % 8) + b'a';
    let rank = (index / 8) + b'1';

    format!("{}{}", file as char, rank as char)
}

fn in_check(boards: &[u64; 12], castling: &[bool; 4], side: &str) -> bool {
    let (king_board, other_side) = if side == "white" {
        (boards[5], "black")
    } else {
        (boards[11], "white")
    };

    let enemy_moves = generate_moves(boards, castling, other_side);

    for mv in enemy_moves {
        if mv.1 == king_board.trailing_zeros() as u8 {
            return true;
        }
    }

    false
}

pub fn generate_legal_moves(
    boards: &[u64; 12],
    castling: &[bool; 4],
    side: &str,
) -> Vec<(u8, u8, Option<char>)> {
    let mut legal_moves = Vec::new();
    let all_moves = generate_moves(boards, castling, side);

    for mv in all_moves {
        let mut new_boards = *boards;
        let mut new_castling = *castling;

        move_piece(mv, &mut new_boards, &mut new_castling, side);

        if !in_check(&new_boards, &new_castling, side) {
            legal_moves.push(mv);
        }
    }

    legal_moves
}

fn minimax(boards: &[u64; 12], castling: &[bool; 4], side: &str, depth: u8) -> i32 {
    if depth == 0 {
        return evaluate(boards);
    }

    let moves = generate_legal_moves(boards, castling, side);

    if side == "white" {
        let mut eval = i32::MIN;

        for mv in moves {
            let mut new_boards = *boards;
            let mut new_castling = *castling;

            move_piece(mv, &mut new_boards, &mut new_castling, side);

            let new_eval = minimax(&new_boards, &new_castling, "black", depth - 1);
            eval = eval.max(new_eval);
        }

        eval
    } else {
        let mut eval = i32::MAX;

        for mv in moves {
            let mut new_boards = *boards;
            let mut new_castling = *castling;

            move_piece(mv, &mut new_boards, &mut new_castling, side);

            let new_eval = minimax(&new_boards, &new_castling, "white", depth - 1);
            eval = eval.min(new_eval);
        }

        eval
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut boards_initialized = false;
    let mut boards: [u64; 12] = [0; 12];

    let mut side = "white";
    let mut castling = [true, true, true, true];

    for line in stdin.lines() {
        let command = line.unwrap();
        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts[0] {
            "uci" => {
                writeln!(stdout, "id name Ghoul").unwrap();
                writeln!(stdout, "id author Nate Davis").unwrap();
                writeln!(stdout, "uciok").unwrap();
            }
            "isready" => {
                writeln!(stdout, "readyok").unwrap();
            }
            "position" => {
                if boards_initialized {
                    let mv = parts.last().unwrap();

                    let from = square_to_index(&mv[0..2]);
                    let to = square_to_index(&mv[2..4]);
                    let promotion = mv.chars().nth(4);

                    move_piece((from, to, promotion), &mut boards, &mut castling, side);
                } else {
                    boards = [
                        0b11111111 << 8,
                        (1 << 1) | (1 << 6),
                        (1 << 2) | (1 << 5),
                        (1 << 0) | (1 << 7),
                        (1 << 3),
                        (1 << 4),
                        0b11111111 << 48,
                        (1 << 57) | (1 << 62),
                        (1 << 58) | (1 << 61),
                        (1 << 56) | (1 << 63),
                        (1 << 59),
                        (1 << 60),
                    ];

                    if parts.contains(&"moves") {
                        side = "black";
                    }

                    boards_initialized = true;
                }
            }
            "go" => {
                let depth = 5;
                let moves = generate_legal_moves(&boards, &castling, side);

                let mut best_score = if side == "white" { i32::MIN } else { i32::MAX };
                let mut best_move = (0u8, 0u8, None);

                for mv in moves {
                    let mut new_boards = boards;
                    let mut new_castling = castling;

                    move_piece(mv, &mut new_boards, &mut new_castling, side);

                    let other_side = if side == "white" { "black" } else { "white" };
                    let score = minimax(&new_boards, &castling, &other_side, depth - 1);

                    if (side == "white" && score > best_score)
                        || (side == "black" && score < best_score)
                    {
                        best_score = score;
                        best_move = mv;
                    }
                }

                let (from, to, promotion) = best_move;
                move_piece(best_move, &mut boards, &mut castling, side);

                let from = index_to_square(from);
                let to = index_to_square(to);

                if let Some(promotion) = promotion {
                    writeln!(stdout, "bestmove {}{}{}", from, to, promotion).unwrap();
                } else {
                    writeln!(stdout, "bestmove {}{}", from, to).unwrap();
                }
            }
            _ => {}
        }
    }
}

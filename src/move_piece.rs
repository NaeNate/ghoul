pub fn move_piece(
    mv: (u8, u8, Option<char>),
    boards: &mut [u64; 12],
    castling: &mut [bool; 4],
    side: &str,
) {
    let (from, to, promotion) = mv;

    let from_mask = 1 << from;
    let to_mask = 1 << to;

    let offset = if side == "white" { 0 } else { 6 };

    match from {
        7 => castling[0] = false,
        0 => castling[1] = false,
        63 => castling[2] = false,
        56 => castling[3] = false,
        4 => {
            castling[0] = false;
            castling[1] = false;
        }
        60 => {
            castling[2] = false;
            castling[3] = false;
        }
        _ => {}
    }

    if boards[5 + offset] & from_mask != 0 {
        let diff = (to as i8 - from as i8).abs();

        if diff == 2 {
            boards[5 + offset] &= !from_mask;
            boards[5 + offset] |= to_mask;

            if to > from {
                if side == "white" {
                    boards[3 + offset] &= !(1 << 7);
                    boards[3 + offset] |= 1 << 5;
                } else {
                    boards[3 + offset] &= !(1 << 63);
                    boards[3 + offset] |= 1 << 61;
                }
            } else {
                if side == "white" {
                    boards[3 + offset] &= !(1 << 0);
                    boards[3 + offset] |= 1 << 3;
                } else {
                    boards[3 + offset] &= !(1 << 56);
                    boards[3 + offset] |= 1 << 59;
                }
            }

            return;
        }
    }

    if let Some(piece) = promotion {
        boards[0 + offset] &= !from_mask;

        let piece_board = match piece {
            'q' => 4 + offset,
            'r' => 3 + offset,
            'b' => 2 + offset,
            'n' => 1 + offset,
            _ => 0,
        };

        boards[piece_board] |= to_mask;
    } else {
        for board in boards {
            if *board & from_mask != 0 {
                *board &= !from_mask;
                *board |= to_mask;
            } else {
                *board &= !to_mask;
            }
        }
    }
}

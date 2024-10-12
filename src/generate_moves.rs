pub fn generate_moves(
    boards: &[u64; 12],
    castling: &[bool; 4],
    side: &str,
) -> Vec<(u8, u8, Option<char>)> {
    let mut moves = Vec::new();

    let offset = if side == "white" { 0 } else { 6 };

    let friendly_pieces = boards[offset..offset + 6]
        .iter()
        .fold(0, |acc, board| acc | board);

    let enemy_pieces = boards[6 - offset..(6 - offset) + 6]
        .iter()
        .fold(0, |acc, &board| acc | board);

    let all_pieces = friendly_pieces | enemy_pieces;

    let mut pawns = boards[offset];

    let (forward, left, right, start_rank, promotion_rank) = if side == "white" {
        (8, 7, 9, 8..16, 56..64)
    } else {
        (-8, -9, -7, 48..56, 0..8)
    };

    while pawns != 0 {
        let index = pawns.trailing_zeros() as u8;

        let forward_move = (index as i8 + forward) as u8;

        if (all_pieces & (1 << forward_move)) == 0 {
            if promotion_rank.contains(&forward_move) {
                for promotion_piece in ['q', 'r', 'b', 'n'] {
                    moves.push((index, forward_move, Some(promotion_piece)));
                }
            } else {
                moves.push((index, forward_move, None));

                if start_rank.contains(&index) {
                    let double = (index as i8 + forward * 2) as u8;

                    if (all_pieces & (1 << double)) == 0 {
                        moves.push((index, double, None));
                    }
                }
            }
        }

        if index % 8 != 0 {
            let left_move = (index as i8 + left) as u8;

            if (enemy_pieces & (1 << left_move)) != 0 {
                if promotion_rank.contains(&forward_move) {
                    for promotion_piece in ['q', 'r', 'b', 'n'] {
                        moves.push((index, left_move, Some(promotion_piece)));
                    }
                } else {
                    moves.push((index, left_move, None));
                }
            }
        }

        if index % 8 != 7 {
            let right_move = (index as i8 + right) as u8;

            if (enemy_pieces & (1 << right_move)) != 0 {
                if promotion_rank.contains(&forward_move) {
                    for promotion_piece in ['q', 'r', 'b', 'n'] {
                        moves.push((index, right_move, Some(promotion_piece)));
                    }
                } else {
                    moves.push((index, right_move, None));
                }
            }
        }

        pawns &= pawns - 1;
    }

    let mut knights = boards[1 + offset];

    while knights != 0 {
        let index = knights.trailing_zeros() as u8;

        for direction in [6, 15, 17, 10, -10, -17, -15, -6] {
            let target = (index as i8 + direction) as u8;

            if target >= 64 {
                continue;
            }

            let start_file = index % 8;
            let end_file = target % 8;

            if (start_file as i8 - end_file as i8).abs() <= 2 {
                if (friendly_pieces & (1 << target)) == 0 {
                    moves.push((index, target, None));
                }
            }
        }

        knights &= knights - 1;
    }

    let mut bishops = boards[2 + offset];

    while bishops != 0 {
        let index = bishops.trailing_zeros() as u8;

        for direction in [-9, -7, 7, 9] {
            let mut target = index as i8;

            loop {
                target += direction;

                if target < 0 || target >= 64 {
                    break;
                }

                if (direction == -9 || direction == 7) && target % 8 == 7 {
                    break;
                }
                if (direction == -7 || direction == 9) && target % 8 == 0 {
                    break;
                }

                if (friendly_pieces & (1 << target)) != 0 {
                    break;
                }

                moves.push((index, target as u8, None));

                if (enemy_pieces & (1 << target)) != 0 {
                    break;
                }
            }
        }

        bishops &= bishops - 1;
    }

    let mut rooks = boards[3 + offset];

    while rooks != 0 {
        let index = rooks.trailing_zeros() as u8;

        for direction in [-8, -1, 1, 8] {
            let mut target = index as i8;

            loop {
                target += direction;

                if target < 0 || target >= 64 {
                    break;
                }

                if direction == 1 && target % 8 == 0 {
                    break;
                }

                if direction == -1 && target % 8 == 7 {
                    break;
                }

                if (friendly_pieces & (1 << target)) != 0 {
                    break;
                }

                moves.push((index, target as u8, None));

                if (enemy_pieces & (1 << target)) != 0 {
                    break;
                }
            }
        }

        rooks &= rooks - 1;
    }

    let mut queens = boards[4 + offset];

    while queens != 0 {
        let index = queens.trailing_zeros() as u8;

        for direction in [-9, -8, -7, -1, 1, 7, 8, 9] {
            let mut target = index as i8;

            loop {
                target += direction;

                if target < 0 || target >= 64 {
                    break;
                }

                if (direction == -9 || direction == 7) && target % 8 == 7 {
                    break;
                }

                if (direction == -7 || direction == 9) && target % 8 == 0 {
                    break;
                }

                if direction == 1 && target % 8 == 0 {
                    break;
                }

                if direction == -1 && target % 8 == 7 {
                    break;
                }

                if (friendly_pieces & (1 << target)) != 0 {
                    break;
                }

                moves.push((index, target as u8, None));

                if (enemy_pieces & (1 << target)) != 0 {
                    break;
                }
            }
        }

        queens &= queens - 1;
    }

    let mut kings = boards[5 + offset];

    while kings != 0 {
        let index = kings.trailing_zeros() as u8;

        let start_file = index % 8;

        for direction in [-9, -8, -7, -1, 1, 7, 8, 9] {
            let target = (index as i8 + direction) as u8;

            if target >= 64 {
                continue;
            }

            if (start_file as i8 - (target % 8) as i8).abs() > 1 {
                continue;
            }

            if (friendly_pieces & (1 << target)) == 0 {
                moves.push((index, target, None));
            }
        }

        if side == "white" {
            if castling[0]
                && (all_pieces & (1 << (index + 1)) == 0)
                && (all_pieces & (1 << (index + 2)) == 0)
            {
                moves.push((index, index + 2, None));
            }

            if castling[1]
                && (all_pieces & (1 << (index - 1)) == 0)
                && (all_pieces & (1 << (index - 2)) == 0)
                && (all_pieces & (1 << (index - 3)) == 0)
            {
                moves.push((index, index - 2, None));
            }
        } else {
            if castling[2]
                && (all_pieces & (1 << (index + 1)) == 0)
                && (all_pieces & (1 << (index + 2)) == 0)
            {
                moves.push((index, index + 2, None));
            }

            if castling[3]
                && (all_pieces & (1 << (index - 1)) == 0)
                && (all_pieces & (1 << (index - 2)) == 0)
                && (all_pieces & (1 << (index - 3)) == 0)
            {
                moves.push((index, index - 2, None));
            }
        }

        kings &= kings - 1;
    }

    // moves.retain(|m| m.1 < 64);
    moves
}

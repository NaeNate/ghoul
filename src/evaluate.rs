pub fn evaluate(boards: &[u64; 12]) -> i32 {
    let values = [1, 3, 3, 5, 9, 0];
    let mut score = 0;

    for i in 0..6 {
        let white_pieces = boards[i].count_ones() as i32;
        let black_pieces = boards[i + 6].count_ones() as i32;

        score += values[i] * (white_pieces - black_pieces);
    }

    score
}

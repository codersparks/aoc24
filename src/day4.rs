use ndarray::{Array, Array2, ArrayView1};


#[aoc(day4, part1)]
fn part_1(input: &str) -> u32 {

    count_word_in_board("XMAS", input) as u32
}

fn generate_board(input: &str) -> Array2<&u8> {
    let input = input.trim();
    let row_count = input.lines().count();
    let bytes = input.as_bytes();
    let row_length = (bytes.len()) / row_count;

    let a = Array::from_iter(input.lines().map(|l| l.as_bytes()).flatten());

    println!("Length: {}, Array: {:?}", a.len(), a);


    let board = a.into_shape_with_order((row_count, row_length)).unwrap();
    board
}

fn count_word_in_board(word: &str, input: &str) -> u32 {


    let board = generate_board(input);

    let row_count = check_rows(word, input);
    println!("Row count: {}", row_count);
    let column_count = check_columns(word, &board);
    println!("Column count: {}", column_count);
    // let diagonal_count = check_diagonals(word, board);
    // println!("Diagonal count: {}", diagonal_count);

    row_count
}

// fn check_diagonals(word: &str, board: &Array2<&u8>) -> usize {
//
//     let word_bytes = word.as_bytes();
//     let binding = word.chars().rev().collect::<String>();
//     let word_bytes_reversed = binding.as_bytes();
//
//     board.windows((word.len(), word.len())).into_iter().map(|window| {
//         let mut forward_diagonal_vec:Vec<&u8> = Vec::with_capacity(word.len());
//         let mut backward_diagonal_vec:Vec<&u8>= Vec::with_capacity(word.len());
//         let mut count = 0usize;
//
//         for i in 0..word.len() {
//             forward_diagonal_vec.push(&window.get((i, i)).unwrap());
//             backward_diagonal_vec.push(&window.get((i, word.len()-1-i)).unwrap());
//         }
//
//
//         let forward_diagonal = forward_diagonal_vec.as_slice();
//         let forward_match = check_slice_for_word(&word, &word_bytes_reversed, forward_diagonal);
//
//         if forward_match {
//             count += 1;
//         }
//
//         let backward_diagonal = backward_diagonal_vec.as_slice();
//         let backward_match = check_slice_for_word(&word, &word_bytes_reversed, backward_diagonal);
//         if backward_match {
//             count += 1;
//         }
//         count
//     }).sum::<usize>()
// }


fn check_array_view(word: &[u8], word_bytes_reversed: &[u8], view: &ArrayView1<&u8>) -> bool {
    let mut word_match = true;

    for i in 0..word.len() {
        word_match &= view[i] == &word[i] || view[i] == &word_bytes_reversed[i];
    }

    word_match
}

fn check_rows(word: &str, input: &str) -> u32 {
    let word_bytes = word.as_bytes();
    let binding = word.chars().rev().collect::<String>();
    let word_bytes_reversed = binding.as_bytes();
    input.lines().into_iter().map(|row| {
        let row_slice = row.as_bytes();
        let row_count = aoc_slices::count_sub_slice_u8(row_slice, word_bytes) + aoc_slices::count_sub_slice_u8(row_slice, word_bytes_reversed);
        println!("Row_count: {row_count}");
        row_count
    }).sum::<u32>()
}

fn check_columns(word: &str, board: &Array2<&u8>) -> u32 {
    let word_bytes = word.as_bytes();
    let binding = word.chars().rev().collect::<String>();
    let word_bytes_reversed = binding.as_bytes();
    board.columns().into_iter().map(|col| {
        let col_slice = col.as_slice().unwrap();

        let word_vec = word_bytes.to_vec().clone().into_iter().map(|b| &b).collect::<Vec<&u8>>();
        let word_ref_slice = word_vec.as_slice();

        let word_reversed_vec = word_bytes_reversed.to_vec().clone().into_iter().map(|b| &b).collect::<Vec<&u8>>();
        let word_reversed_ref_slice = word_reversed_vec.as_slice();

        let col_count = aoc_slices::count_sub_slice_ref_u8(col_slice, word_ref_slice) + aoc_slices::count_sub_slice_ref_u8(col_slice, word_reversed_ref_slice);
        println!("Col_count: {col_count}");
        col_count
    }).sum::<u32>()
}

fn check_columns_old(word: &[u8], word_bytes_reversed: &[u8], board: &Array2<&u8>) -> usize {
    board.columns().into_iter().map(|col| {
        col.windows(word.len()).into_iter().map(|window| {
            if check_array_view(word, word_bytes_reversed, &window) {
                1usize
            } else {
                0usize
            }
        }).sum::<usize>()
    }).sum()
}


fn check_slice_for_word(word: &&[u8], word_bytes_reversed: &&[u8], slice: &[&u8]) -> bool {
    let mut word_match = true;
    for i in 0..word.len() {
        word_match &= slice[i] == &word[i] || slice[i] == &word_bytes_reversed[i];
    }
    word_match
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_part_1_small() {
        let input = fs::read_to_string("test_input/day4_small.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 4);
    }
    #[test]
    fn test_part_1() {
        // let input = fs::read_to_string("input/2024/day4.txt").unwrap();
        let input = fs::read_to_string("test_input/day4.txt").unwrap();

        let output = part_1(&input);
        assert_eq!(output, 18);
    }

    #[test]
    fn test_part_1_rows() {
        let input = fs::read_to_string("test_input/day4_row.txt").unwrap();

        let output = part_1(&input);
        assert_eq!(output, 3);
    }

    #[test]
    fn test_part_1_cols() {
        let input = fs::read_to_string("test_input/day4_column.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 3);
    }

    #[test]
    fn test_part_1_diag() {
        let input = fs::read_to_string("test_input/day4_diag.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 2);
    }

    // #[test]
    // fn test_check_check_diagonals() {
    //
    //     let word_bytes = "XMAS".as_bytes();
    //     let binding = "XMAS".chars().rev().collect::<String>();
    //     let word_bytes_reversed = binding.as_bytes();
    //
    //     let input = fs::read_to_string("input/2024/day4.txt").unwrap();
    //     let board = generate_board(&input);
    //
    //     let output = check_diagonals(word_bytes, word_bytes_reversed, &board);
    //
    //     assert_eq!(output, 10);
    //
    //
    // }

    #[test]
    fn test_check_check_rows() {

        let word = "XMAS";


        // let input = fs::read_to_string("input/2024/day4.txt").unwrap();
        let input = fs::read_to_string("test_input/day4.txt").unwrap();

        let output = check_rows(word, input.as_str());

        assert_eq!(output, 5);


    }

    #[test]
    fn test_check_columns() {
        let word = "XMAS";



        let input = fs::read_to_string("test_input/day4.txt").unwrap();
        let board = generate_board(&input);
        let output = check_columns(word, &board);

        assert_eq!(output, 3)
    }
}
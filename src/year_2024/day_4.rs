use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 4 of 2024:
 * Ceres Search
 */
pub struct Day4_2024;

impl DayResult for Day4_2024 {
    fn print_day_result() {
        let input = fetch_input(4, 2024);
        println!("Result 1: {}", get_result_1(input));
        // println!("Result 2: {}", get_result_2(input.clone()));
    }
}

fn parse_grid(input: Vec<String>) -> Vec<Vec<char>> {
    input
        .iter()
        .flat_map(|line| line.split('\n'))
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect()
}

fn get_result_1(input: Vec<String>) -> i32 {
    let grid = parse_grid(input);
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),   // right
        (0, -1),  // left
        (-1, 0),  // up
        (1, 0),   // down
        (1, 1),   // diagonal down
        (-1, -1), // diagonal up left
        (1, -1),  // diagonal down left
        (-1, 1),  // diagonal down right
    ];

    let word = "XMAS";
    let row_count = grid.len();
    let col_count = grid[0].len();

    let search_grid = |row: usize, col: usize, row_delta: &i32, col_delta: &i32| -> bool {
        for (idx, current_char) in word.chars().enumerate() {
            let new_row = row as i32 + idx as i32 * row_delta;
            let new_col = col as i32 + idx as i32 * col_delta;

            if !(0..row_count as i32).contains(&new_row)
                || !(0..col_count as i32).contains(&new_col)
                || grid[new_row as usize][new_col as usize] != current_char
            {
                return false;
            }
        }

        true
    };

    let mut count = 0;

    for row in 0..row_count {
        for col in 0..col_count {
            count += directions
                .iter()
                .filter(|&&(row_delta, col_delta)| search_grid(row, col, &row_delta, &col_delta))
                .count() as i32;
        }
    }

    count
}

#[allow(unused)]
fn get_result_2(input: Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &[&str] = &["MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX"];

    static TEST_INPUT_2: &[&str] = &[".M.S......
        ..A..MSMS.
        .M.S.MAA..
        ..A.ASMSM.
        .M.S.M....
        ..........
        S.S.S.S.S.
        .A.A.A.A..
        M.M.M.M.M.
        ..........
        "];

    fn get_test_input(input: &[&str]) -> Vec<String> {
        input.iter().map(|&s| s.to_string()).collect()
    }

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(get_test_input(TEST_INPUT_1));
        assert_eq!(result, 18);
    }

    #[ignore]
    #[test]
    fn test_get_result_2() {
        let result = get_result_2(get_test_input(TEST_INPUT_2));
        assert_eq!(result, 9);
    }
}

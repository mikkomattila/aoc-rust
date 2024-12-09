use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 6 of 2024:
 * Guard Gallivant
 */
pub struct Day6_2024;

impl DayResult for Day6_2024 {
    fn print_day_result() {
        let input = fetch_input(6, 2024);
        println!("Result 1: {}", get_result_1(input.clone()));
        println!("Result 2: {}", get_result_2(input));
    }
}

fn parse_map(input: Vec<String>) -> Vec<Vec<char>> {
    input
        .iter()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect()
}

fn get_result_1(input: Vec<String>) -> i32 {
    const GUARD: char = '^';
    const OBSTRUCTION: char = '#';
    const MARK: char = 'X';

    let mut map = parse_map(input);
    let (start_row, start_col) = map
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|&c| c == GUARD).map(|col| (row, col)))
        .expect("No starting position found");

    map[start_row][start_col] = MARK;

    let get_next_direction = |dir: (isize, isize)| match dir {
        (-1, 0) => (0, 1),  // up to right
        (0, 1) => (1, 0),   // right to down
        (1, 0) => (0, -1),  // down to left
        (0, -1) => (-1, 0), // left to up
        _ => panic!("Invalid direction change"),
    };

    let mut current_direction = (-1, 0);
    let mut previous_position = (start_row, start_col);
    let mut current_row = start_row;
    let mut current_col = start_col;

    while current_row < map.len() && current_col < map[current_row].len() {
        if map[current_row][current_col] == OBSTRUCTION {
            current_row = previous_position.0;
            current_col = previous_position.1;
            current_direction = get_next_direction(current_direction);
        } else {
            map[current_row][current_col] = MARK;
        }

        previous_position = (current_row, current_col);
        current_row = (current_row as isize + current_direction.0) as usize;
        current_col = (current_col as isize + current_direction.1) as usize;

        let is_out_of_bounds = current_row >= map.len() || current_col >= map[current_row].len();
        if is_out_of_bounds {
            break;
        }
    }

    map.iter()
        .map(|line| line.iter().filter(|&&c| c == MARK).count())
        .sum::<usize>() as i32
}

#[allow(unused)]
fn get_result_2(input: Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &[&str] = &[
        "....#.....",
        ".........#",
        "..........",
        "..#.......",
        ".......#..",
        "..........",
        ".#..^.....",
        "........#.",
        "#.........",
        "......#...",
    ];

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.iter().map(|&s| s.to_string()).collect()
    }

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(get_test_input());
        assert_eq!(result, 41);
    }

    #[ignore = "todo"]
    #[test]
    fn test_get_result_2() {
        let result = get_result_2(get_test_input());
        assert_eq!(result, 6);
    }
}

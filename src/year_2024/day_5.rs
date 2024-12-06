use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 5 of 2024:
 * Print Queue
 */
pub struct Day5_2024;

impl DayResult for Day5_2024 {
    fn print_day_result() {
        let input = fetch_input(5, 2024);
        println!("Result 1: {}", get_result_1(input.clone()));
        println!("Result 2: {}", get_result_2(input));
    }
}

fn parse_input(input: Vec<String>) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let split_index: usize = input
        .iter()
        .position(|s| s.is_empty())
        .expect("No empty line found");

    let (rules_section, updates_section) = input.split_at(split_index);

    let page_ordering_rules = rules_section
        .iter()
        .map(|s| {
            let mut split = s.split('|');
            let first = split
                .next()
                .expect("Failed to get first element")
                .parse::<i32>()
                .expect("Failed to parse string to i32");
            let second = split
                .next()
                .expect("Failed to get second element")
                .parse::<i32>()
                .expect("Failed to parse string to i32");
            (first, second)
        })
        .collect();

    let updates = updates_section[1..]
        .iter()
        .map(|s| {
            s.split(',')
                .map(|s| s.parse::<i32>().expect("Failed to parse string to i32"))
                .collect()
        })
        .collect();

    (page_ordering_rules, updates)
}

fn get_result_1(input: Vec<String>) -> i32 {
    let mut sum = 0;
    let (page_ordering_rules, updates) = parse_input(input);

    for page_sequence in updates {
        let mut is_valid_sequence = true;

        for (i, &page_number) in page_sequence.iter().enumerate() {
            let remaining_pages = &page_sequence[i + 1..];
            let passed_pages = &page_sequence[..i];
            let allowed_pages = page_ordering_rules
                .iter()
                .filter(|&&(s, _)| s == page_number)
                .map(|&(_, b)| b)
                .collect::<Vec<i32>>();

            if !remaining_pages.iter().all(|&r| allowed_pages.contains(&r))
                || !passed_pages.iter().all(|&r| !allowed_pages.contains(&r))
            {
                is_valid_sequence = false;
                break;
            }
        }

        if is_valid_sequence {
            sum += page_sequence[page_sequence.len() / 2];
        }
    }

    sum
}

fn get_result_2(input: Vec<String>) -> i32 {
    let mut sum = 0;
    let (page_ordering_rules, updates) = parse_input(input);
    let mut invalid_page_sequences: Vec<Vec<i32>> = Vec::new();

    let get_allowed_pages = |page_number: i32| -> Vec<i32> {
        page_ordering_rules
            .iter()
            .filter(|&&(s, _)| s == page_number)
            .map(|&(_, b)| b)
            .collect::<Vec<i32>>()
    };

    for page_sequence in updates {
        for (i, &page_number) in page_sequence.iter().enumerate() {
            let remaining_pages = &page_sequence[i + 1..];
            let passed_pages = &page_sequence[..i];
            let allowed_pages = get_allowed_pages(page_number);

            if !remaining_pages.iter().all(|&r| allowed_pages.contains(&r))
                || !passed_pages.iter().all(|&r| !allowed_pages.contains(&r))
            {
                invalid_page_sequences.push(page_sequence.clone());
                break;
            }
        }
    }

    for mut sequence in invalid_page_sequences {
        let fixed_sequence: Vec<i32> = fix_invalid_sequence(&mut sequence[..], &get_allowed_pages);
        sum += fixed_sequence[fixed_sequence.len() / 2];
    }

    sum
}

fn fix_invalid_sequence(
    invalid_page_sequence: &mut [i32],
    get_allowed_pages: &dyn Fn(i32) -> Vec<i32>,
) -> Vec<i32> {
    for (i, &page_number) in invalid_page_sequence.iter().enumerate() {
        let passed_pages = &invalid_page_sequence[..i];
        let allowed_pages = get_allowed_pages(page_number);

        if passed_pages.iter().any(|&r| allowed_pages.contains(&r)) {
            for (j, &passed_page) in passed_pages.iter().enumerate() {
                if allowed_pages.contains(&passed_page) {
                    invalid_page_sequence[i] = passed_page;
                    invalid_page_sequence[j] = page_number;
                    return fix_invalid_sequence(invalid_page_sequence, get_allowed_pages);
                }
            }
            return fix_invalid_sequence(invalid_page_sequence, get_allowed_pages);
        }
    }
    invalid_page_sequence.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &[&str] = &[
        "47|53",
        "97|13",
        "97|61",
        "97|47",
        "75|29",
        "61|13",
        "75|53",
        "29|13",
        "97|29",
        "53|29",
        "61|53",
        "97|53",
        "61|29",
        "47|13",
        "75|47",
        "97|75",
        "47|61",
        "75|61",
        "47|29",
        "75|13",
        "53|13",
        "",
        "75,47,61,53,29",
        "97,61,53,29,13",
        "75,29,13",
        "75,97,47,61,53",
        "61,13,29",
        "97,13,75,29,47",
    ];

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.iter().map(|&s| s.to_string()).collect()
    }

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(get_test_input());
        assert_eq!(result, 143);
    }

    #[test]
    fn test_get_result_2() {
        let result = get_result_2(get_test_input());
        assert_eq!(result, 123);
    }
}

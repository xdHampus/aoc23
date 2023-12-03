advent_of_code::solution!(1);
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    const RADIX: u32 = 10;
    let mut res: u32 = 0;
    for line in input.lines() {
        for tchar in line.chars() {
            if tchar.is_digit(RADIX) {
                res += 10 * tchar.to_digit(RADIX).unwrap();
                break;
            }
        }
        for tchar in line.chars().rev() {
            if tchar.is_digit(RADIX) {
                res += tchar.to_digit(RADIX).unwrap();
                break;
            }
        }
    }

    return Some(res);
}

pub fn part_two(input: &str) -> Option<u32> {
    let helper_data: HelperData = HelperData::new();

    let mut res: u32 = 0;
    for line in input.lines() {
        res += extract_number(line, false, &helper_data);
        res += extract_number(line, true, &helper_data);
    }

    return Some(res);
}

fn extract_number(line: &str, reversed: bool, helper_data: &HelperData) -> u32 {
    let mut active_strings: Vec<String> = Vec::new();
    const RADIX: u32 = 10;
    let res_mult = if reversed { 1 } else { 10 };

    let full_numbers = if reversed {
        &helper_data.full_numbers_reversed
    } else {
        &helper_data.full_numbers
    };
    let incomplete_numbers = if reversed {
        &helper_data.incomplete_numbers_reversed
    } else {
        &helper_data.incomplete_numbers
    };

    let char_iterator: Box<dyn Iterator<Item = char>> = if reversed {
        Box::new(line.chars().rev())
    } else {
        Box::new(line.chars())
    };

    for tchar in char_iterator {
        if tchar.is_digit(RADIX) {
            return res_mult * tchar.to_digit(RADIX).unwrap();
        }

        active_strings.push(String::new());
        let mut remove_strings: Vec<usize> = Vec::new();
        for (index, tstr) in active_strings.iter_mut().enumerate() {
            tstr.push(tchar);
            if full_numbers.contains_key(tstr.as_str()) {
                return res_mult * full_numbers.get(tstr.as_str()).unwrap();
            } else if incomplete_numbers.contains(tstr.as_str()) {
                //do nothing
            } else {
                remove_strings.push(index);
            }
        }

        for index in remove_strings.iter().rev() {
            active_strings.remove(*index);
        }
    }

    return 0;
}

struct HelperData {
    full_numbers: HashMap<String, u32>,
    incomplete_numbers: HashSet<String>,
    full_numbers_reversed: HashMap<String, u32>,
    incomplete_numbers_reversed: HashSet<String>,
}

impl HelperData {
    fn new() -> HelperData {
        const NUMBERS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let full_numbers: HashMap<String, u32> = NUMBERS
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x.to_string(), i as u32 + 1))
            .collect();

        let full_numbers_reversed: HashMap<String, u32> = NUMBERS
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x.chars().rev().collect::<String>(), i as u32 + 1))
            .collect();

        let mut incomplete_numbers: HashSet<String> = HashSet::new();
        let mut incomplete_numbers_reversed: HashSet<String> = HashSet::new();

        for number in NUMBERS.iter() {
            let mut temp = String::new();
            for tchar in number.chars() {
                temp.push(tchar);
                incomplete_numbers.insert(temp.clone());
            }
            temp = String::new();
            for tchar in number.chars().rev() {
                temp.push(tchar);
                incomplete_numbers_reversed.insert(temp.clone());
            }
        }

        HelperData {
            full_numbers,
            incomplete_numbers,
            full_numbers_reversed,
            incomplete_numbers_reversed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}

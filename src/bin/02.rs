advent_of_code::solution!(2);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut color_limit: HashMap<&str, u32> = HashMap::new();
    color_limit.insert("red", 12);
    color_limit.insert("green", 13);
    color_limit.insert("blue", 14);

    let mut valid_ids: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut valid = true;
        let mut parsed_line = line.split(": ");
        let game_id: u32 = parsed_line
            .nth(0)
            .unwrap()
            .replace("Game ", "")
            .parse()
            .unwrap();
        let color_line = parsed_line.nth(0).unwrap().replace(";", ",");

        let mut color_entries: Vec<&str> = color_line.split(", ").collect();

        for color_entry in color_entries.iter_mut() {
            let color_entry_split: Vec<&str> = color_entry.split(" ").collect();
            let color_entry_count: u32 = color_entry_split[0].parse().unwrap();
            let color_entry_name: &str = color_entry_split[1];
            if color_entry_count > color_limit[color_entry_name] {
                valid = false;
                break;
            }
        }
        if valid {
            valid_ids.push(game_id);
        }
    }

    return Some(valid_ids.iter().sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut summed_product = 0;

    for line in input.lines() {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        let color_line = line.split(": ").nth(1).unwrap().replace(";", ",");
        let color_entries: Vec<&str> = color_line.split(", ").collect();

        for color_entry in color_entries.iter() {
            let color_entry_split: Vec<&str> = color_entry.split(" ").collect();
            let color_entry_count: u32 = color_entry_split[0].parse().unwrap();
            let color_entry_name: &str = color_entry_split[1];

            match color_entry_name {
                "red" => {
                    r = std::cmp::max(r, color_entry_count);
                }
                "green" => {
                    g = std::cmp::max(g, color_entry_count);
                }
                "blue" => {
                    b = std::cmp::max(b, color_entry_count);
                }
                _ => {}
            }
        }

        summed_product += r * g * b;
    }

    return Some(summed_product);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}

use std::{
    collections::HashMap,
    io::{self},
};

fn main() {
    let lines = io::stdin().lines();

    let total = get_lines_summary(lines.map(|line| line.unwrap()));

    dbg!(total);
}

fn get_sum_lines<I: Iterator<Item = String>>(lines: I) -> i32 {
    lines
        .take_while(|line| line.len() > 0)
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .sum()
}

fn get_lines_summary<I: Iterator<Item = String>>(lines: I) -> HashMap<i32, usize> {
    return lines
        .take_while(|line| line.len() > 0)
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .fold(HashMap::new(), |mut map, num| {
            *map.entry(num).or_insert(0) += 1;
            return map;
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sum_all_ints() {
        let test_data = ["1", "2", "3"].iter().map(|item| item.to_string());

        assert_eq!(get_sum_lines(test_data), 6);
    }

    #[test]
    fn should_ignore_after_empty() {
        let test_data = ["1", "2", "3", "", "4"].iter().map(|item| item.to_string());

        assert_eq!(get_sum_lines(test_data), 6);
    }

    #[test]
    fn should_ignore_non_ints() {
        let test_data = ["1", "2", "dhjds", "3"].iter().map(|item| item.to_string());

        assert_eq!(get_sum_lines(test_data), 6);
    }
}

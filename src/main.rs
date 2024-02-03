use std::io::{self};

fn main() {
    let lines = io::stdin().lines();

    let total = get_sum_lines(lines.map(|line| line.unwrap()));

    println!("total: {}", total);
}

fn get_sum_lines<I: Iterator<Item = String>>(lines: I) -> i32 {
    lines
        .take_while(|line| line.len() > 0)
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .sum()
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

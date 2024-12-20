use crate::day::{day01, day02, day03, day04, day05, day06};
use crate::solution::Solution;

pub fn get_lines(day: i32) -> Result<Vec<String>, String> {
    let filename = format!("inputs/day{:02}.txt", day);
    let file = match std::fs::read_to_string(filename) {
        Ok(f) => f,
        Err(e) => {
            return Err(format!("Failed to read file: {e}"));
        }
    };

    let lines = file.lines().map(|s| s.to_string()).collect();
    Ok(lines)
}

pub fn get_latest_day() -> Result<i32, String> {
    let mut latest_day = 0;
    while std::path::Path::new(&format!("inputs/day{:02}.txt", latest_day + 1)).exists() {
        latest_day += 1;
    }

    if latest_day == 0 {
        return Err("No input files for any days found".to_string());
    }
    Ok(latest_day)
}

pub fn solve(day: i32, lines: Vec<String>) -> Result<Solution, String> {
    match day {
        1 => Ok(day01::solve(lines)),
        2 => Ok(day02::solve(lines)),
        3 => Ok(day03::solve(lines)),
        4 => Ok(day04::solve(lines)),
        5 => Ok(day05::solve(lines)),
        6 => Ok(day06::solve(lines)),
        _ => Err(format!("Day {day} not implemented")),
    }
}

pub fn multiline_to_vec(s: &str) -> Vec<String> {
    s.lines().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiline_to_vec() {
        let test_input = r#"line1
line2"#;
        let expected = vec!["line1".to_string(), "line2".to_string()];
        assert_eq!(multiline_to_vec(test_input), expected);
    }
}

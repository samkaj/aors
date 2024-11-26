use crate::day::day01;

pub fn get_lines(day: i32) -> Result<Vec<String>, String> {
    let filename = format!("../inputs/day{:02}.txt", day);
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
    while std::path::Path::new(&format!("../inputs/day{:02}.txt", latest_day + 1)).exists() {
        latest_day += 1;
    }

    if latest_day == 0 {
        return Err("No input files for any days found".to_string());
    }

    Ok(latest_day)
}

pub fn solve(day: i32, lines: Vec<String>) -> Result<String, String> {
    match day {
        1 => Ok(day01::solve(lines)),
        _ => Err(format!("Day {day} not implemented")),
    }
}

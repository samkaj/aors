use crate::solution::Solution;

pub fn solve(lines: Vec<String>) -> Solution {
    Solution::new(part1(lines.clone()), part2(lines))
}

fn part1(lines: Vec<String>) -> String {
    let mut safe_count = 0;
    for line in lines {
        let levels = list_from_line(line);
        if level_is_safe(levels) {
            safe_count += 1;
        }
    }

    safe_count.to_string()
}

fn part2(_lines: Vec<String>) -> String {
    "".to_string()
}

fn list_from_line(line: String) -> Vec<i32> {
    line.split(" ")
        .into_iter()
        .map(|n| n.parse::<i32>().expect("a number"))
        .collect()
}

fn level_is_safe(list: Vec<i32>) -> bool {
    diff_ok(list.clone()) && (all_increasing(list.clone()) || all_decreasing(list.clone()))
}

fn all_increasing(list: Vec<i32>) -> bool {
    let mut orig = list.clone();
    orig.sort();
    orig == list
}

fn all_decreasing(list: Vec<i32>) -> bool {
    let mut orig = list.clone();
    orig.sort();
    orig.reverse();
    orig == list
}

fn diff_ok(list: Vec<i32>) -> bool {
    let mut iter = list.iter();
    let mut prev = iter.next().expect("empty list");
    for n in iter {
        let diff = i32::abs(n - prev);
        if diff == 0 || diff > 3 {
            return false;
        }
        prev = n;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve_part1() {
        let test_input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part1(lines), "2");
    }

    #[test]
    fn test_solve_part2() {
        let test_input = r#""#;
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part2(lines), "11");
    }
}

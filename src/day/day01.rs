use crate::solution::Solution;

pub fn solve(lines: Vec<String>) -> Solution {
    Solution::new(part1(lines.clone()), part2(lines))
}

fn part1(lines: Vec<String>) -> String {
    lines.join("\n")
}

fn part2(lines: Vec<String>) -> String {
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve_part1() {
        let test_input = r#""#;
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part1(lines), "Not implemented");
    }

    #[test]
    fn test_solve() {
        let test_input = r#""#;
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part2(lines), "Not implemented");
    }
}

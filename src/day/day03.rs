use regex::Regex;

use crate::solution::Solution;

struct Mul {
    lhs: i32,
    rhs: i32,
}

impl Mul {
    pub fn new(lhs: i32, rhs: i32) -> Mul {
        Mul { lhs, rhs }
    }

    pub fn mul(&self) -> i32 {
        self.lhs * self.rhs
    }
}

pub fn solve(lines: Vec<String>) -> Solution {
    Solution::new(part1(lines.clone()), part2(lines))
}

fn part1(lines: Vec<String>) -> String {
    let mut sum = 0;
    for line in lines {
        let pairs = extract_matches(line);
        let muls: Vec<Mul> = pairs.iter().map(|p| Mul::new(p.0, p.1)).collect();
        muls.iter().for_each(|m| sum += m.mul());
    }

    return sum.to_string();
}

fn part2(lines: Vec<String>) -> String {
    let line = get_enabled_parts(lines.join(""));
    let sum: i32 = part1(line).parse().unwrap_or(0);

    sum.to_string()
}

fn extract_matches(line: String) -> Vec<(i32, i32)> {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let regex = Regex::new(pattern).expect("invalid regex");

    let matches: Vec<(i32, i32)> = regex
        .captures_iter(line.as_str())
        .filter_map(|caps| {
            let n = caps.get(1)?.as_str().parse().ok()?;
            let m = caps.get(2)?.as_str().parse().ok()?;
            Some((n, m))
        })
        .collect();

    matches
}

fn get_enabled_parts(line: String) -> Vec<String> {
    let do_split: Vec<&str> = line.split("do()").collect();
    let mut enabled: Vec<String> = vec![];
    for dos in do_split.as_slice() {
        let foo: Vec<&str> = dos.split("don't()").collect();
        enabled.push(foo.get(0).unwrap().to_string());
    }

    enabled
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve_part1() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part1(lines), "161");
    }

    #[test]
    fn test_solve_part2() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part2(lines), "48");
    }
}

use crate::solution::Solution;
use std::iter::zip;

pub fn solve(lines: Vec<String>) -> Solution {
    Solution::new(part1(lines.clone()), part2(lines))
}

fn pairs(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in lines {
        let pair: Vec<&str> = line.rsplit("   ").collect();
        let l: i32 = pair
            .get(0)
            .expect("something")
            .parse()
            .expect("not a number");
        let r: i32 = pair
            .get(1)
            .expect("something")
            .parse()
            .expect("not a number");
        left.push(l);
        right.push(r);
    }

    (left, right)
}

fn part1(lines: Vec<String>) -> String {
    let (mut left, mut right) = pairs(lines);
    left.sort();
    right.sort();
    let mut sum = 0;
    zip(left, right).for_each(|a| sum += i32::abs(a.0 - a.1));

    sum.to_string()
}

fn part2(lines: Vec<String>) -> String {
    let (left, right) = pairs(lines);
    let sum: i32 = left
        .iter()
        .map(|x| x * right.iter().filter(|y| x == *y).count() as i32)
        .sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve_part1() {
        let test_input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part1(lines), "11");
    }

    #[test]
    fn test_solve_part2() {
        let test_input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part2(lines), "31");
    }
}

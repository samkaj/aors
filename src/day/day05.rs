use crate::solution::Solution;

pub fn solve(lines: Vec<String>) -> Solution {
    Solution::new(part1(lines.clone()), part2(lines))
}

struct Update {
    entries: Vec<i32>,
}

#[derive(Clone)]
struct Rule {
    left: i32,
    right: i32,
}

impl Update {
    pub fn preceds(&self, rule: Rule) -> Option<bool> {
        let n = rule.left;
        let m = rule.right;
        if let Some(first) = self.entries.iter().position(|&x| x == n) {
            if let Some(second) = self.entries.iter().position(|&x| x == m) {
                return Some(first < second);
            }
        }

        None
    }

    pub fn middle(&self) -> i32 {
        let mid = self.entries.len() / 2;
        *self.entries.get(mid).unwrap()
    }
}

fn part1(lines: Vec<String>) -> String {
    let rules = lines
        .iter()
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let rules: Vec<Rule> = rules
        .iter()
        .map(|s| {
            let lr: Vec<&str> = s.split('|').collect();
            let l: i32 = lr[0].parse().unwrap();
            let r: i32 = lr[1].parse().unwrap();
            Rule { left: l, right: r }
        })
        .collect();

    let updates = lines
        .iter()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .collect::<Vec<_>>();

    let updates: Vec<Update> = updates
        .iter()
        .map(|s| {
            let nums: Vec<i32> = s.split(",").map(|s| s.parse().unwrap()).collect::<Vec<_>>();
            Update { entries: nums }
        })
        .collect();

    let mut count = 0;

    for update in updates {
        let mut ok = true;
        for rule in &rules {
            let res = update.preceds(rule.clone());
            match res {
                Some(b) => {
                    if !b {
                        ok = false;
                    }
                }
                None => continue,
            }
        }
        if ok {
            count += update.middle();
        }
    }

    count.to_string()
}

fn part2(lines: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn p1() {
        let test_input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part1(lines), "143");
    }

    #[test]
    fn test_solve_part2() {
        let test_input = r#""#;
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part2(lines), "");
    }
}

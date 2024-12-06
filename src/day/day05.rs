use crate::solution::Solution;

pub fn solve(lines: Vec<String>) -> Solution {
    Solution::new(part1(lines.clone()), part2(lines))
}

#[derive(Clone)]
struct Update {
    entries: Vec<i32>,
}

#[derive(Clone)]
struct Rule {
    left: i32,
    right: i32,
}

impl Update {
    pub fn allowed(&self, rule: Rule) -> Option<bool> {
        let n = rule.left;
        let m = rule.right;
        if let Some(first) = self.entries.iter().position(|&x| x == n) {
            if let Some(second) = self.entries.iter().position(|&x| x == m) {
                return Some(first < second);
            }
        }

        None
    }

    pub fn reorder(&mut self, rule: Rule) {
        let n = rule.left;
        let m = rule.right;
        if let Some(first) = self.entries.iter().position(|&x| x == n) {
            if let Some(second) = self.entries.iter().position(|&x| x == m) {
                self.entries.swap(first, second);
            }
        }
    }

    pub fn middle(&self) -> i32 {
        self.entries[self.entries.len() / 2]
    }
}

fn get_rules(lines: Vec<String>) -> Vec<Rule> {
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
    rules
}

fn get_updates(lines: Vec<String>) -> Vec<Update> {
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
    updates
}

fn part1(lines: Vec<String>) -> String {
    let rules = get_rules(lines.clone());
    let updates = get_updates(lines);

    let mut count = 0;

    for update in updates {
        let mut ok = true;
        for rule in &rules {
            let res = update.allowed(rule.clone());
            match res {
                Some(b) => {
                    if !b {
                        ok = false;
                        break;
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
    let rules = get_rules(lines.clone());
    let updates = get_updates(lines);
    let mut count = 0;
    let mut bad_updates: Vec<Update> = vec![];
    for update in updates {
        for rule in &rules {
            let res = update.allowed(rule.clone());
            match res {
                Some(b) => {
                    if !b {
                        bad_updates.push(update.clone());
                        break;
                    }
                }
                None => continue,
            }
        }
    }

    for update in &mut bad_updates {
        let mut ok = false;
        while !ok {
            ok = true;
            for rule in &rules {
                if Some(false) == update.allowed(rule.clone()) {
                    update.reorder(rule.clone());
                    ok = false;
                }
            }
        }
        count += update.middle();
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve_part1() {
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
        assert_eq!(part2(lines), "123");
    }
}

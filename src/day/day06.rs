use crate::solution::Solution;
use std::collections::HashSet;

pub fn solve(lines: Vec<String>) -> Solution {
    Solution::new(part1(lines.clone()), part2(lines))
}

fn part1(lines: Vec<String>) -> String {
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let start_pos = matrix.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .enumerate()
            .find_map(|(x, &c)| if c == '^' { Some((x, y)) } else { None })
    });
    let start_pos = start_pos.unwrap();
    let mut pos = start_pos;
    let mut dir = (0, -1);
    let y_lim = matrix[0].len() as i32;
    let x_lim = matrix.len() as i32;
    let mut visited: HashSet<(usize, usize)> = HashSet::default();

    loop {
        visited.insert(pos);
        let next = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        if next.0 >= y_lim || next.0 < 0 || next.1 >= x_lim || next.1 < 0 {
            break;
        }

        // if there is something in front of you, turn right 90 degrees
        if matrix[next.1 as usize][next.0 as usize] == '#' {
            dir = match dir {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!("Invalid direction"),
            };
        } else {
            // otherwise, take a step forward
            pos = (next.0 as usize, next.1 as usize);
        }
    }

    visited.len().to_string()
}

fn part2(lines: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn day6() {
        let test_input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part1(lines), "41");
    }
}

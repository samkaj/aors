use crate::solution::Solution;

pub fn solve(lines: Vec<String>) -> Solution {
    Solution::new(part1(lines.clone()), part2(lines))
}

fn part1(lines: Vec<String>) -> String {
    let mut count = 0;
    let height = lines.len();
    for (i, line) in lines.clone().iter().enumerate() {
        for (j, char) in line.clone().chars().enumerate() {
            if char == 'X' {
                // XMAS
                if j + 3 < line.len() {
                    let right = &line[j..j + 4];
                    if right == "XMAS" {
                        count += 1;
                    }
                }

                // XMAS
                if j > 2 {
                    let left = &line[j - 3..j + 1];
                    if left == "SAMX" {
                        count += 1;
                    }
                }

                // X
                //  M
                //   A
                //    S
                if i + 3 < height && j + 3 < line.len() {
                    let diag_right = vec![
                        lines[i].chars().collect::<Vec<_>>()[j],
                        lines[i + 1].chars().collect::<Vec<_>>()[j + 1],
                        lines[i + 2].chars().collect::<Vec<_>>()[j + 2],
                        lines[i + 3].chars().collect::<Vec<_>>()[j + 3],
                    ];
                    let diag: String = diag_right.iter().collect();
                    if diag == "XMAS" {
                        count += 1;
                    }
                }

                // X
                // M
                // A
                // S
                if i + 3 < height {
                    let diag_right = vec![
                        lines[i].chars().collect::<Vec<_>>()[j],
                        lines[i + 1].chars().collect::<Vec<_>>()[j],
                        lines[i + 2].chars().collect::<Vec<_>>()[j],
                        lines[i + 3].chars().collect::<Vec<_>>()[j],
                    ];
                    let diag: String = diag_right.iter().collect();
                    if diag == "XMAS" {
                        count += 1;
                    }
                }

                // S
                // A
                // M
                // X
                if i > 2 {
                    let diag_right = vec![
                        lines[i].chars().collect::<Vec<_>>()[j],
                        lines[i - 1].chars().collect::<Vec<_>>()[j],
                        lines[i - 2].chars().collect::<Vec<_>>()[j],
                        lines[i - 3].chars().collect::<Vec<_>>()[j],
                    ];
                    let diag: String = diag_right.iter().collect();
                    if diag == "XMAS" {
                        count += 1;
                    }
                }

                // S
                //  A
                //   M
                //    X
                if i > 2 && j > 2 {
                    let diag_right = vec![
                        lines[i].chars().collect::<Vec<_>>()[j],
                        lines[i - 1].chars().collect::<Vec<_>>()[j - 1],
                        lines[i - 2].chars().collect::<Vec<_>>()[j - 2],
                        lines[i - 3].chars().collect::<Vec<_>>()[j - 3],
                    ];
                    let diag: String = diag_right.iter().collect();
                    if diag == "XMAS" {
                        count += 1;
                    }
                }

                //    X
                //   M
                //  A
                // S
                if i + 3 < height && j > 2 {
                    let diag_right = vec![
                        lines[i].chars().collect::<Vec<_>>()[j],
                        lines[i + 1].chars().collect::<Vec<_>>()[j - 1],
                        lines[i + 2].chars().collect::<Vec<_>>()[j - 2],
                        lines[i + 3].chars().collect::<Vec<_>>()[j - 3],
                    ];
                    let diag: String = diag_right.iter().collect();
                    if diag == "XMAS" {
                        count += 1;
                    }
                }

                //    S
                //   A
                //  M
                // X
                if i > 2 && j + 3 < line.len() {
                    let diag_right = vec![
                        lines[i].chars().collect::<Vec<_>>()[j],
                        lines[i - 1].chars().collect::<Vec<_>>()[j + 1],
                        lines[i - 2].chars().collect::<Vec<_>>()[j + 2],
                        lines[i - 3].chars().collect::<Vec<_>>()[j + 3],
                    ];
                    let diag: String = diag_right.iter().collect();
                    if diag == "XMAS" {
                        count += 1;
                    }
                }
            }
        }
    }

    return count.to_string();
}

fn part2(lines: Vec<String>) -> String {
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn p1() {
        let test_input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part1(lines), "18");
    }

    #[test]
    fn test_solve_part2() {
        let test_input = r#""#;
        let lines = util::multiline_to_vec(test_input);
        assert_eq!(part2(lines), "TODO");
    }
}

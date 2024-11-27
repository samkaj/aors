use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Solution {
    pub part1: String,
    pub part2: String,
}

impl Solution {
    pub fn new(part1: String, part2: String) -> Solution {
        Solution { part1, part2 }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Part 1: {}\nPart 2: {}", self.part1, self.part2)
    }
}

use regex::Regex;
use std::collections::VecDeque;
use std::fmt::Display;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let input = parse("input.txt")?;
    let solution = solve(input);
    println!("{solution}");
    Ok(())
}

fn parse(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    Ok(lines.flatten().collect())
}

#[derive(Debug, Clone)]
struct Stacks {
    stacks: Vec<VecDeque<char>>,
}

impl Stacks {
    fn with_capacity(count: usize) -> Self {
        Self {
            stacks: (0..count).map(|_| VecDeque::new()).collect(),
        }
    }

    /// Create a `Stacks` instance from a drawing description
    fn from_description<T: AsRef<str>>(stack_lines: &[T]) -> Self {
        let number_lines = stack_lines
            .iter()
            .next_back()
            .expect("should have more than one line");
        let number_stacks = number_lines.as_ref().split_whitespace().count();
        let mut stacks = Self::with_capacity(number_stacks);

        let stack_re = Regex::new(r"((?P<crate>\[[A-Z]\])|(\s{3}))\s?").unwrap();
        for line in &stack_lines[..stack_lines.len() - 1] {
            for (i, caps) in stack_re.captures_iter(line.as_ref()).enumerate() {
                if let Some(m) = caps.name("crate") {
                    let ch = m.as_str().chars().nth(1).expect("should be a character");
                    stacks.push_front(ch, i);
                }
            }
        }
        stacks
    }

    fn push_front(&mut self, c: char, to: usize) {
        assert!(c.is_uppercase());
        assert!(to < self.stacks.len());
        self.stacks[to].push_front(c);
    }

    /// Move `count` elements from the stack `from` to stack `dest`
    fn shift(&mut self, count: usize, from: usize, to: usize) {
        assert!(from < self.stacks.len());
        assert!(to < self.stacks.len());
        assert!(count <= self.stacks[from].len());
        for _ in 0..count {
            let value = self.stacks[from].pop_back().unwrap();
            self.stacks[to].push_back(value);
        }
    }

    fn message(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.iter().last().expect("should have a crate"))
            .collect()
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_len = self.stacks.iter().map(|s| s.len()).max().unwrap();
        for i in (0..max_len).rev() {
            f.write_fmt(format_args!("{:>3} ", i + 1))?;
            for s in self.stacks.iter() {
                if let Some(c) = s.get(i) {
                    f.write_fmt(format_args!("[{}]", c))?;
                } else {
                    f.write_str("   ")?;
                }
                f.write_str(" ")?;
            }
            f.write_str("\n")?;
        }
        f.write_str("    ")?;
        for i in 0..self.stacks.len() {
            f.write_fmt(format_args!("{:>3} ", i + 1))?;
        }
        Ok(())
    }
}

fn solve(input: Vec<String>) -> String {
    let mut lines = input.iter();
    let stack_lines: Vec<_> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
    let mut stacks = Stacks::from_description(&stack_lines);

    let instruction_lines = lines;
    // Parse the instructions
    let instruction_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for line in instruction_lines {
        let caps = instruction_re
            .captures_iter(line)
            .next()
            .expect("should match the line");
        let count: usize = caps
            .get(1)
            .expect("should have count")
            .as_str()
            .parse()
            .expect("should be a positive number");
        let orig: usize = caps
            .get(2)
            .expect("should have origin stack")
            .as_str()
            .parse()
            .expect("should be a positive number");
        let dest: usize = caps
            .get(3)
            .expect("should have destination stack")
            .as_str()
            .parse()
            .expect("should be a positive number");
        // Execute the instruction
        stacks.shift(count, orig - 1, dest - 1);
    }
    stacks.message()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, "CMZ");
    }
}

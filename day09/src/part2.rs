use std::collections::BTreeSet;
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

fn parse(path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut parsed = Vec::new();
    for line in lines.into_iter().flatten() {
        let words: Vec<String> = line.split_whitespace().map(String::from).collect();
        parsed.push(words);
    }

    Ok(parsed)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position(i64, i64);

impl Position {
    fn step(&self, direction: Direction) -> Self {
        let (x, y) = match direction {
            Direction::Up => (self.0, self.1 + 1),
            Direction::Right => (self.0 + 1, self.1),
            Direction::Down => (self.0, self.1 - 1),
            Direction::Left => (self.0 - 1, self.1),
        };
        Position(x, y)
    }

    fn follow(&self, other: &Self) -> Self {
        // Compute the new tail position
        let delta_x = other.0 - self.0;
        let delta_y = other.1 - self.1;

        let new_x = if delta_x.abs() > 0 && delta_y.abs() > 0 && (delta_x.abs() + delta_y.abs() > 2)
        {
            self.0 + delta_x.clamp(-1, 1)
        } else if delta_x > 1 {
            self.0 + 1
        } else if delta_x < -1 {
            self.0 - 1
        } else {
            self.0
        };

        let new_y = if delta_x.abs() > 0 && delta_y.abs() > 0 && (delta_x.abs() + delta_y.abs() > 2)
        {
            self.1 + delta_y.clamp(-1, 1)
        } else if delta_y > 1 {
            self.1 + 1
        } else if delta_y < -1 {
            self.1 - 1
        } else {
            self.1
        };

        Position(new_x, new_y)
    }
}

#[derive(Debug, Clone, Default)]
struct Rope {
    head: Position,
    knots: [Position; 9],
    tail_positions: BTreeSet<Position>,
}

impl Rope {
    fn step(&mut self, direction: Direction) {
        let head = &self.knots[0];
        self.head = self.head.step(direction);

        let mut prev = &self.head;
        for i in 0..self.knots.len() {
            let knot = self.knots[i];
            let new_knot = knot.follow(prev);
            self.knots[i] = new_knot;
            prev = &self.knots[i];
            if i == self.knots.len() - 1 {
                // We are in the tail
                self.tail_positions.insert(new_knot);
            }
        }
    }

    fn step_many(&mut self, direction: Direction, count: usize) {
        (0..count).for_each(|_| self.step(direction))
    }

    fn print_position(&mut self) {
        for j in (0..21).rev() {
            for i in 0..26 {
                let pos = Position(i, j);
                if self.head == pos {
                    print!("H");
                } else if let Some(i) = self.knots.iter().position(|p| *p == pos) {
                    print!("{}", i + 1);
                } else if i == 0 && j == 0 {
                    print!("s");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            c => Err(format!("invalid character {}", c)),
        }
    }
}

fn solve(input: Vec<Vec<String>>) -> u64 {
    let mut rope = Rope::default();
    for mut line in input {
        let count = line.pop().expect("a count").parse().expect("a valid count");
        let dir_char = line.pop().expect("a direction");
        let direction: Direction = (&dir_char[..]).try_into().expect("a valid character");
        rope.step_many(direction, count);
    }

    rope.tail_positions.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 1)
    }

    #[test]
    fn test_sample2() {
        let input = parse("sample2.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 36)
    }
}

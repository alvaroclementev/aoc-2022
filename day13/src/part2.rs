use itertools::{EitherOrBoth, Itertools};
use nom::{branch::alt, combinator::map, multi::separated_list0, sequence::delimited, IResult};
use std::{
    fs::File,
    io::{self, BufRead},
};

// NOTE: Taking the chance to learn how to use `nom` and parser combinators
// with a simple example

fn main() -> io::Result<()> {
    let input = lines("input.txt")?;
    let solution = solve(input);
    println!("{solution}");
    Ok(())
}

fn lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    Ok(lines.into_iter().flatten().collect())
}

fn _line_words(path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut parsed = Vec::new();
    for line in lines.into_iter().flatten() {
        let words: Vec<String> = line.split_whitespace().map(String::from).collect();
        parsed.push(words);
    }

    Ok(parsed)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketValue {
    Value(i32),
    List(Vec<PacketValue>),
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketValue::Value(left), PacketValue::Value(right)) => left.cmp(right),
            (PacketValue::List(left), PacketValue::List(right)) => {
                // Compare lists
                for pair in left.iter().zip_longest(right.iter()) {
                    match pair {
                        EitherOrBoth::Both(l, r) => match l.cmp(r) {
                            std::cmp::Ordering::Equal => continue,
                            cmp => return cmp,
                        },
                        EitherOrBoth::Left(..) => return std::cmp::Ordering::Greater,
                        EitherOrBoth::Right(..) => return std::cmp::Ordering::Less,
                    }
                }
                std::cmp::Ordering::Equal
            }
            (left @ PacketValue::List(..), PacketValue::Value(right)) => {
                left.cmp(&PacketValue::List(vec![PacketValue::Value(*right)]))
            }
            (PacketValue::Value(left), right @ PacketValue::List(..)) => {
                let left = PacketValue::List(vec![PacketValue::Value(*left)]);
                left.cmp(right)
            }
        }
    }
}

impl std::fmt::Display for PacketValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketValue::List(vec) => {
                write!(f, "[")?;
                for (i, e) in vec.iter().enumerate() {
                    e.fmt(f)?;
                    if i < vec.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?;
            }
            PacketValue::Value(value) => {
                write!(f, "{}", value)?;
            }
        }
        Ok(())
    }
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn value(input: &str) -> IResult<&str, PacketValue> {
    map(nom::character::complete::i32, PacketValue::Value)(input)
}

/// Parser for a PacketValue::List
fn list(input: &str) -> IResult<&str, PacketValue> {
    map(
        delimited(
            nom::character::complete::char('['),
            separated_list0(nom::character::complete::char(','), alt((value, list))),
            nom::character::complete::char(']'),
        ),
        PacketValue::List,
    )(input)
}

fn parse_packet(input: &str) -> PacketValue {
    match list(input) {
        Ok((rest, packet)) => {
            assert!(rest.is_empty(), "the input was not fully parsed");
            packet
        }
        Err(e) => {
            println!("Error parsing packet: {e}");
            panic!();
        }
    }
}

fn solve(input: Vec<String>) -> u64 {
    // Group the packets into pairs
    let mut collected = input
        .iter()
        .filter(|l| !l.is_empty())
        .map(|line| parse_packet(line))
        .collect::<Vec<_>>();

    // Insert divided packets
    let first_divider = parse_packet("[[2]]");
    let second_divider = parse_packet("[[6]]");
    collected.push(first_divider.clone());
    collected.push(second_divider.clone());

    // Sort the vecotr
    collected.sort();

    // Find the first and and second dividesr
    let first = collected.iter().position(|p| p == &first_divider).unwrap() + 1;
    let second = collected.iter().position(|p| p == &second_divider).unwrap() + 1;

    // Sort them
    (first * second) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = lines("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 140);
    }
}

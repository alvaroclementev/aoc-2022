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

#[derive(Debug, PartialEq, Eq)]
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
    input
        .iter()
        .filter(|l| !l.is_empty())
        .tuples()
        .enumerate()
        .filter_map(|(i, (left, right))| {
            let left = parse_packet(left);
            let right = parse_packet(right);
            if left <= right {
                Some((i + 1) as u64)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = lines("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 13);
    }

    #[test]
    fn test_parse_value() {
        assert_eq!(value("32"), Ok(("", PacketValue::Value(32))));
        assert_eq!(value("-32"), Ok(("", PacketValue::Value(-32))));
    }

    #[test]
    fn test_parse_empty_list() {
        assert_eq!(list("[]"), Ok(("", PacketValue::List(vec![]))));
    }

    #[test]
    fn test_parse_flat_list() {
        assert_eq!(
            list("[1,2,-3,4]"),
            Ok((
                "",
                PacketValue::List(vec![
                    PacketValue::Value(1),
                    PacketValue::Value(2),
                    PacketValue::Value(-3),
                    PacketValue::Value(4),
                ])
            ))
        );
    }

    #[test]
    fn test_parse_deep_list() {
        assert_eq!(
            list("[1,2,[-3],4]"),
            Ok((
                "",
                PacketValue::List(vec![
                    PacketValue::Value(1),
                    PacketValue::Value(2),
                    PacketValue::List(vec![PacketValue::Value(-3)]),
                    PacketValue::Value(4),
                ])
            ))
        );
    }

    #[test]
    fn test_parse_deeper_list() {
        assert_eq!(
            list("[1,2,[[-3]],4]"),
            Ok((
                "",
                PacketValue::List(vec![
                    PacketValue::Value(1),
                    PacketValue::Value(2),
                    PacketValue::List(vec![PacketValue::List(vec![PacketValue::Value(-3)])]),
                    PacketValue::Value(4),
                ])
            ))
        );
    }
}

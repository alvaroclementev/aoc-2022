use regex::Regex;
use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

// NOTE(alvaro): It would be nice to try to use `nom` (parser combinators)
// to parse this input
#[derive(Debug)]
enum Operand {
    Old,
    Value(u64),
}

impl Operand {
    fn value(&self, old: u64) -> u64 {
        match self {
            Operand::Old => old,
            Operand::Value(value) => *value,
        }
    }
}

impl FromStr for Operand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            num if num.chars().all(|c| c.is_numeric()) => Ok(Operand::Value(num.parse().unwrap())),
            _ => Err("Invalid operand"),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Add(left, right) => left.value(old) + right.value(old),
            Operation::Mul(left, right) => left.value(old) * right.value(old),
        }
    }
}

fn main() -> io::Result<()> {
    let input = parse("input.txt")?;
    let solution = solve(input);
    println!("{solution}");
    Ok(())
}

fn parse(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    Ok(lines.into_iter().flatten().collect())
}

// NOTE(alvaro): We could use `lazy_static` to make these global constants
fn parse_monkey_line(line: &str) -> usize {
    let re = Regex::new(r"^\s*Monkey (\d+):\s*$").unwrap();
    let cap = re.captures_iter(line).next().expect("monkey line to match");
    cap[1].parse().unwrap()
}

fn parse_items_line(line: &str) -> VecDeque<u64> {
    let items_start: usize = line.find(':').expect("line to have :") + 1;
    let line = &line[items_start..];
    let item_re = Regex::new(r"\s*(\d+)\s*,?").unwrap();
    item_re
        .captures_iter(line)
        .map(|cap| cap[1].parse().unwrap())
        .collect()
}

fn parse_operation_line(line: &str) -> Operation {
    let re = Regex::new(r"^\s*Operation:\s*new\s*=\s*(old|\d+)\s*(\+|\*)\s*(old|\d+)\s*$").unwrap();
    let cap = re.captures_iter(line).next().unwrap();
    let (left, op, right) = (&cap[1], &cap[2], &cap[3]);
    let left = left.parse().unwrap();
    let right = right.parse().unwrap();
    match op {
        "+" => Operation::Add(left, right),
        "*" => Operation::Mul(left, right),
        _ => unreachable!(),
    }
}

fn parse_test_lines<'a>(line: impl IntoIterator<Item = &'a str>) -> (u64, usize, usize) {
    // Parse the first test line
    let re = Regex::new(r"^\s*Test:\s*divisible\s+by\s+(\d+)\s*$").unwrap();
    let mut line_iter = line.into_iter();
    let first_line = line_iter.next().unwrap();
    let test_cap = re
        .captures_iter(first_line)
        .next()
        .expect("monkey line to match");
    let test_value = test_cap[1].parse().unwrap();
    // Parse true / flase
    let true_false_re =
        Regex::new(r"^\s*If\s+(?:true|false)\s*:\s*throw\s+to\s+monkey\s+(\d+)\s*$").unwrap();
    let true_line = line_iter.next().unwrap();
    let true_monkey = true_false_re
        .captures_iter(true_line)
        .next()
        .expect("true line to match")[1]
        .parse()
        .unwrap();
    let false_line = line_iter.next().unwrap();
    let false_monkey = true_false_re
        .captures_iter(false_line)
        .next()
        .expect("false line to match")[1]
        .parse()
        .unwrap();
    (test_value, true_monkey, false_monkey)
}

#[derive(Debug)]
struct MonkeyInstructions {
    number: usize,
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
}

impl MonkeyInstructions {
    fn new(
        number: usize,
        items: VecDeque<u64>,
        operation: Operation,
        test: u64,
        if_true: usize,
        if_false: usize,
    ) -> Self {
        Self {
            number,
            items,
            operation,
            test,
            if_true,
            if_false,
        }
    }
}

fn solve(input: Vec<String>) -> u64 {
    // TODO(alvaro): Can we use `group_by()` for this?
    // Prepare the text to be parsed
    let mut instructions = vec![vec![]];
    for line in input.into_iter() {
        let line = line.trim_start().to_owned();
        if line.is_empty() {
            instructions.push(Vec::new());
        } else {
            instructions.last_mut().unwrap().push(line);
        }
    }

    let mut monkeys = vec![];
    // Parse the Monkey instructions
    for turn in instructions {
        let mut turn_iter = turn.iter().map(|s| &s[..]);
        // Parse the monkey line
        let monkey_n = parse_monkey_line(turn_iter.next().unwrap());
        let items = parse_items_line(turn_iter.next().unwrap());
        let operation = parse_operation_line(turn_iter.next().unwrap());
        let (test, if_true, if_false) = parse_test_lines(&mut turn_iter);
        monkeys.push(MonkeyInstructions::new(
            monkey_n, items, operation, test, if_true, if_false,
        ));
    }

    // A temporary insertion vector used to please the borrow checker
    let mut insertions = Vec::new();

    // Use the MCM to maintain the number in a reduced range while keeping
    // the properties for the modulo operation
    let mcm: u64 = monkeys.iter().map(|m| m.test).product();

    let monkey_count = monkeys.len();
    let mut monkey_inspections: Vec<u64> = (0..monkey_count).map(|_| 0).collect();
    // Execute the rounds
    for _ in 1..=10_000 {
        for i in 0..monkey_count {
            let monkey = monkeys.get_mut(i).unwrap();
            while let Some(item) = monkey.items.pop_front() {
                let new_value = monkey.operation.apply(item) % mcm;
                let target_monkey_n = if new_value % monkey.test == 0 {
                    // Maintain the number in a controlled range
                    monkey.if_true
                } else {
                    monkey.if_false
                };
                insertions.push((new_value, target_monkey_n));

                // Count the inspection
                *monkey_inspections.get_mut(i).unwrap() += 1;
            }

            // NOTE: this is done in a seprate loop so that we don't need to
            // take 2 mutable references to the monkeys Vec at the same time,
            // which is not allowed. We could do in 1 loop with `split_at_mut`,
            // but the logic is more complicated.

            // Actually perform the insertions, draining the vector
            for (item, to) in insertions.drain(..) {
                let target_monkey = monkeys
                    .get_mut(to)
                    .expect("the target monkey number to exist");
                target_monkey.items.push_back(item);
            }
        }
    }

    // Monkey Business
    monkey_inspections.sort();
    monkey_inspections.into_iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 2713310158)
    }
}

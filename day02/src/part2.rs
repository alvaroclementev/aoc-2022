use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Clone, Copy)]
enum Pick {
    Rock,
    Paper,
    Scissor,
}

impl Pick {
    fn score(&self) -> u64 {
        match self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scissor => 3,
        }
    }

    fn for_outcome(other_pick: &Pick, outcome: &Outcome) -> Self {
        match (other_pick, outcome) {
            (Pick::Rock, Outcome::Lose) => Pick::Scissor,
            (Pick::Rock, Outcome::Win) => Pick::Paper,
            (Pick::Paper, Outcome::Lose) => Pick::Rock,
            (Pick::Paper, Outcome::Win) => Pick::Scissor,
            (Pick::Scissor, Outcome::Lose) => Pick::Paper,
            (Pick::Scissor, Outcome::Win) => Pick::Rock,
            (pick, _) => *pick,
        }
    }
}

impl From<&str> for Pick {
    fn from(pick_str: &str) -> Self {
        match pick_str {
            "A" | "X" => Pick::Rock,
            "B" | "Y" => Pick::Paper,
            "C" | "Z" => Pick::Scissor,
            _ => panic!("invalid pick {}", pick_str),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn from_game(pick: &Pick, other_pick: &Pick) -> Self {
        match (pick, other_pick) {
            (Pick::Rock, Pick::Paper) => Outcome::Lose,
            (Pick::Rock, Pick::Scissor) => Outcome::Win,
            (Pick::Paper, Pick::Rock) => Outcome::Win,
            (Pick::Paper, Pick::Scissor) => Outcome::Lose,
            (Pick::Scissor, Pick::Rock) => Outcome::Lose,
            (Pick::Scissor, Pick::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }

    fn score_from_game(pick: &Pick, other_pick: &Pick) -> u64 {
        let outcome = Self::from_game(pick, other_pick);
        outcome.score() + pick.score()
    }
}

impl From<&str> for Outcome {
    fn from(outcome_str: &str) -> Self {
        match outcome_str {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("invalid outcome_str {}", outcome_str),
        }
    }
}

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

fn solve(input: Vec<Vec<String>>) -> u64 {
    input
        .into_iter()
        .map(|line| {
            let other = Pick::from(line[0].as_ref());
            let outcome = Outcome::from(line[1].as_ref());
            let pick = Pick::for_outcome(&other, &outcome);
            Outcome::score_from_game(&pick, &other)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 12)
    }
}

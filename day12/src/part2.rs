use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

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

type Position = (usize, usize);

#[derive(Debug)]
struct ShortestPath {
    position: Position,
    from: Position,
    steps: usize,
}

impl ShortestPath {
    fn new(position: Position, from: Position, steps: usize) -> Self {
        Self {
            position,
            from,
            steps,
        }
    }
}

#[derive(Debug)]
struct Map {
    heights: Vec<Vec<u8>>,
    start: Position,
    end: Position,
}

impl Map {
    fn new(heights: Vec<Vec<u8>>, start: Position, end: Position) -> Self {
        Self {
            heights,
            start,
            end,
        }
    }

    fn from_input(input: Vec<String>) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);

        let heights: Vec<Vec<u8>> = input
            .iter()
            .enumerate()
            .map(|(j, l)| {
                l.chars()
                    .into_iter()
                    .enumerate()
                    .map(|(i, c)| match c {
                        'S' => {
                            start = (i, j);
                            0
                        }
                        'E' => {
                            end = (i, j);
                            b'z' - b'a'
                        }
                        c @ 'a'..='z' => c as u8 - b'a',
                        _ => panic!("Invalid character found"),
                    })
                    .collect()
            })
            .collect();

        Self::new(heights, start, end)
    }

    fn hill_climb(&self, start: Position) -> u64 {
        // Begin from `start` and hill climb until we get to the end
        let mut shortest: HashMap<Position, ShortestPath> = HashMap::new();

        // NOTE: I think We could make this faster using a greedy algorithm
        // with early stop (see `std::collections::BinaryHeap`)
        // Depth-first search
        // A candidate is a tuple (pos, from, steps)
        let mut candidates = Vec::from([ShortestPath::new(start, start, 0)]);
        while let Some(path) = candidates.pop() {
            let position = path.position;
            // Check if there's already a shortest path for this
            if shortest
                .get(&path.position)
                .filter(|sp| sp.steps <= path.steps)
                .is_some()
            {
                // This is a worse path, so stop exploring it
                continue;
            } else {
                // This is the best path to the solution, so record it
                shortest.insert(position, path);
            }
            let path = shortest.get_mut(&position).unwrap();

            // Explore this path
            let right = (position.0 as isize + 1, position.1 as isize);
            let bottom = (position.0 as isize, position.1 as isize + 1);
            let left = (position.0 as isize - 1, position.1 as isize);
            let top = (position.0 as isize, position.1 as isize - 1);

            let cur_height = self.height_at(position).unwrap();
            [right, bottom, left, top]
                .into_iter()
                .map(|pos| (pos, self.height_at_relative(pos)))
                .filter(|(_, height)| {
                    height
                        .filter(|h| (*h as i32 - cur_height as i32) <= 1)
                        .is_some()
                })
                // Add the candidates to the queue
                // Now we can prioritize the direction through which we want to
                // go, for now we try to climb as much of the hill as we can
                // at the beginning
                .sorted_by_key(|x| x.1.unwrap())
                .rev()
                .for_each(|(pos, _)| {
                    candidates.push(ShortestPath::new(
                        (pos.0 as usize, pos.1 as usize),
                        path.position,
                        path.steps + 1,
                    ))
                });
        }

        // Return the best path to the end
        shortest
            .get(&self.end)
            .map(|p| p.steps as u64)
            .unwrap_or(u64::MAX)
    }

    fn height_at(&self, position: Position) -> Option<u8> {
        let (i, j) = position;
        if j < self.heights.len() && i < self.heights[0].len() {
            Some(self.heights[j][i])
        } else {
            None
        }
    }

    fn height_at_relative(&self, position: (isize, isize)) -> Option<u8> {
        if position.0 >= 0 && position.1 >= 0 {
            self.height_at((position.0 as usize, position.1 as usize))
        } else {
            None
        }
    }
}

fn solve(input: Vec<String>) -> u64 {
    let map = Map::from_input(input);
    map.heights
        .iter()
        .enumerate()
        .flat_map(|(j, l)| l.iter().enumerate().map(move |(i, h)| ((i, j), h)))
        .filter(|(_, h)| **h == 0)
        .map(|(pos, _)| pos)
        .map(|pos| map.hill_climb(pos))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = lines("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 29)
    }
}

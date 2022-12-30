use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Copy, Clone)]
enum Inst {
    Noop,
    Addx(i64),
}

impl Inst {
    fn ticks(&self) -> u8 {
        match self {
            Inst::Noop => 1,
            Inst::Addx(_) => 2,
        }
    }
}

impl TryFrom<&str> for Inst {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        let inst = parts.next().ok_or("no instruction found")?;
        match inst {
            "noop" => Ok(Inst::Noop),
            "addx" => {
                let imm = parts.next().ok_or("no immediate found")?;
                let imm = imm
                    .parse()
                    .map_err(|_| "immediate is not a valid integer")?;
                Ok(Inst::Addx(imm))
            }
            _ => Err("found invalid command"),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Cpu {
    inst: Vec<Inst>,
    cycle: usize,
    /// Points to the _next_ instruction to execute
    pc: usize,
    x: i64,
    /// Number of ticks spent in the current instruction
    inst_ticks: u8,
}

impl Cpu {
    fn from_instructions(inst: Vec<Inst>) -> Self {
        Self {
            inst,
            cycle: 1,
            pc: 0,
            x: 1,
            inst_ticks: 0,
        }
    }

    /// Run a cycle of the clock. Returns `true` if the program is finished
    fn tick(&mut self) -> bool {
        // Fetch the current instruction
        let Some(inst) = self.inst.get(self.pc) else {
            // We are done!
            return true;
        };

        self.inst_ticks += 1;
        if self.inst_ticks < inst.ticks() {
            self.cycle += 1;
            return false;
        }

        // Ready to actually execute the instruction
        match inst {
            Inst::Noop => {}
            Inst::Addx(imm) => {
                self.x += imm;
            }
        };

        // We executed a new cycle and a new instruction
        self.inst_ticks = 0;
        self.cycle += 1;
        self.pc += 1;
        false
    }

    fn tick_n(&mut self, n: usize) {
        for _ in 0..n {
            let done = self.tick();
            if done {
                break;
            }
        }
    }

    fn signal_strength(&self) -> i64 {
        self.cycle as i64 * self.x
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
    let lines = io::BufReader::new(file).lines().flatten().collect();
    Ok(lines)
}

fn solve(input: Vec<String>) -> i64 {
    let inst = input
        .into_iter()
        .flat_map(|line| (&line[..]).try_into())
        .collect();
    let mut cpu = Cpu::from_instructions(inst);

    // NOTE(alvaro): We start with 19 since we want the value BEFORE executing
    // cycle 20, not after
    // Start the CPU
    cpu.tick_n(19);
    (20..=220)
        .step_by(40)
        .map(|_| {
            let strength = cpu.signal_strength();
            cpu.tick_n(40);
            strength
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
        assert_eq!(solution, 13140)
    }
}

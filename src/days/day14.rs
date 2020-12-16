use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
struct Bitmask {
    include: u64,
    exclude: u64,
    floating: Vec<u64>,
}

impl Bitmask {
    fn new(include: u64, exclude: u64, floating: u64) -> Bitmask {
        // Get each "X" in the mask as an individual bit
        // ex: X01X -> [1000, 0001]
        let mut bits = Vec::new();
        for dig in 0..36 {
            let is_floating = (floating >> dig) & 1 == 1;

            if is_floating {
                bits.push(floating & (1 << dig));
                bits.push(0);
            }
        }
        let floating = bits
            .iter()
            .combinations(bits.len() / 2)
            .map(|bits| bits.iter().fold(0, |acc, bit| acc | **bit))
            .collect();

        Bitmask {
            include,
            exclude,
            floating,
        }
    }

    const fn empty() -> Bitmask {
        Bitmask {
            include: 0,
            exclude: 1,
            floating: Vec::new(),
        }
    }

    fn apply(&self, other: u64) -> u64 {
        let mut result = other;

        result |= self.include;
        result &= self.exclude;

        result
    }

    fn apply_v2(&self, other: usize) -> Vec<usize> {
        let other = other as u64;
        let other = other | self.include;

        self.floating
            .iter()
            .map(|xor| (other ^ xor) as usize)
            .collect()
    }
}

impl FromStr for Bitmask {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let include = s.replace("X", "0");
        let exclude = s.replace("X", "1");
        let floating = s.replace("1", "0").replace("X", "1");

        let include = u64::from_str_radix(&include, 2).map_err(|_| "Unable to parse u64")?;
        let exclude = u64::from_str_radix(&exclude, 2).map_err(|_| "Unable to parse u64")?;
        let floating = u64::from_str_radix(&floating, 2).map_err(|_| "Unable to parse u64")?;

        Ok(Bitmask::new(include, exclude, floating))
    }
}

#[derive(Clone)]
enum Operation {
    Mask(Bitmask),
    Mem(usize, u64),
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MASK_RE: Regex =
                Regex::new(r"mask = ([0-1|X]+)").expect("Failed to initialize regex");
            static ref MEM_RE: Regex =
                Regex::new(r"mem\[(\d+)\] = (\d+)").expect("Failed to initialize regex");
        }

        let op = s.split(" ").next().ok_or("Unable to parse number")?;

        if op == "mask" {
            let caps = MASK_RE.captures(s).ok_or("Unable to parse input")?;
            let mask = caps.get(1).ok_or("Unable to get mask")?.as_str();
            return Ok(Operation::Mask(Bitmask::from_str(mask)?));
        }

        let caps = MEM_RE.captures(s).ok_or("Unable to parse input")?;

        let index = caps.get(1).ok_or("Unable to get mask")?.as_str();
        let index = index
            .parse::<usize>()
            .map_err(|_| "Unable to convert to u64")?;

        let value = caps.get(2).ok_or("Unable to get mask")?.as_str();
        let value = value
            .parse::<u64>()
            .map_err(|_| "Unable to convert to u64")?;

        Ok(Operation::Mem(index, value))
    }
}

#[aoc_generator(day14)]
fn generator(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| Operation::from_str(line).unwrap())
        .collect()
}

const DEFAULT_MASK: Bitmask = Bitmask::empty();

#[aoc(day14, part1)]
fn part1(input: &[Operation]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = DEFAULT_MASK;

    input.iter().for_each(|op| match op {
        Operation::Mask(new_mask) => {
            mask = new_mask.clone();
        }
        Operation::Mem(i, val) => {
            let val = mask.apply(*val);

            mem.insert(i, val);
        }
    });

    mem.values().sum()
}

#[aoc(day14, part2)]
fn part2(input: &[Operation]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = DEFAULT_MASK;

    input.iter().for_each(|op| match op {
        Operation::Mask(new_mask) => {
            mask = new_mask.clone();
        }
        Operation::Mem(i, val) => {
            let indicies = mask.apply_v2(*i);

            indicies.iter().for_each(|i| {
                mem.insert(*i, *val);
            });
        }
    });

    mem.values().sum()
}

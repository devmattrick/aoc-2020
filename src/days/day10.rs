use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;

use crate::aoc_test;

#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<u16> {
  input.lines().map(|line| line.parse::<u16>().expect("Unable to parse input")).collect()
}

#[aoc(day10, part1)]
fn part1(input: &[u16]) -> u16 {
  let mut input = Vec::from(input);
  input.sort();

  let mut one_diffs = 0;
  let mut three_diffs = 1; // Start at 1, because final adapter -> device will always have a difference of 3

  let mut prev = 0;
  input.iter().for_each(|n| {
    match n - prev {
      1 => one_diffs += 1,
      2 => (),
      3 => three_diffs += 1,
      _ => panic!("Difference not allowed!")
    }

    prev = *n;
  });

  one_diffs * three_diffs
}

// Made into a struct to make memoization easier
struct Part2Solver {
  max: u16,
  values: Vec<u16>,
  cache: HashMap<usize, u128>,
}

impl Part2Solver {
  fn new(values: &[u16]) -> Part2Solver {
    // "Clean up" the input vec to make things easier
    let mut values = Vec::from(values);
    values.push(0); // Add 0 for the "charging port"
    values.sort();
    let max = values.last().expect("Unable to get last element") + 3;

    Part2Solver {
      max,
      values,
      cache: HashMap::new(),
    }
  }

  fn solve(&mut self) -> u128 {
    self.solve_inner(0)
  }

  fn solve_inner(&mut self, i: usize) -> u128 {
    if self.cache.contains_key(&i) {
      return *self.cache.get(&i).expect("Unable to get value");
    }

    let val = self.values[i];

    // Base case: the current value + 3 is the max value
    if self.max - val == 3 {
      return 1;
    }


    // Recursive case: for all "next" values between i + 1 to i + 3 (excluding any that are out of bounds)
    let mut answers = 0;

    let cloned = self.values.clone();
    let iter = cloned.iter().enumerate();
    let nexts = iter.skip(i + 1).take(3);

    for (next_i, next_val) in nexts {
      // If the next value is within a range of 3 of the current value
      if next_val - val <= 3 {
        // Find the number of answers from there
        answers += self.solve_inner(next_i);
      }
    }

    self.cache.insert(i, answers);

    answers
  }
}

#[aoc(day10, part2)]
fn part2(input: &[u16]) -> u128 {
  Part2Solver::new(input).solve()
}

aoc_test! {
  input = "
    28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3
  ";

  part1 = "220";
  part2 = "19208"
}

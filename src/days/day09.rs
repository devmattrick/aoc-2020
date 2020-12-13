use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use std::collections::VecDeque;
use std::iter::FromIterator;

use crate::aoc_test;

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().expect("Unable to parse int"))
        .collect()
}

const BUFFER_SIZE: usize = 25;

#[aoc(day9, part1)]
fn part1(input: &[u64]) -> u64 {
    let input = input.iter();
    let mut queue = VecDeque::from_iter(input.clone().take(BUFFER_SIZE).map(|n| *n));
    let input = input.skip(25);

    for num in input {
        let mut found = false;

        for combination in queue.iter().tuple_combinations().map(|(a, b)| *a + *b) {
            if *num == combination {
                found = true;
                break;
            }
        }

        if !found {
            return *num;
        }

        queue.pop_front();
        queue.push_back(*num);
    }

    panic!("Unable to find incorrect line");
}

#[aoc(day9, part2)]
fn part2(input: &[u64]) -> u64 {
    let invalid = part1(input);
    let len = input.len();

    for i in 2..len {
        for window in input.windows(i) {
            let sum: u64 = window.iter().sum();

            if sum == invalid {
                let mut window: Vec<u64> = window.iter().map(|n| *n).collect();
                window.sort();

                return window.first().expect("Could not get first element")
                    + window.last().expect("Could not get last element");
            }
        }
    }

    panic!("Unable to find combination");
}

// TODO Write tests
aoc_test! {
  input = "
    1
    2
    3
    4
    5
    6
    7
    8
    9
    10
    11
    12
    13
    14
    15
    16
    17
    18
    19
    20
    21
    22
    23
    24
    25
    26
    49
    100
    124
  ";

  part1 = "100";
  part2 = "74";
}

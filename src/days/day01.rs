use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use crate::aoc_test;

const TARGET: u32 = 2020;

#[aoc_generator(day1)]
fn generator(input: &str) -> Vec<u32> {
    let input = input.trim();

    input.lines().map(|x| x.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    for x in input {
        for y in input {
            if x + y == TARGET {
                return x * y;
            }
        }
    }

    return 0;
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    for x in input {
        for y in input {
            for z in input {
                if x + y + z == TARGET {
                    return x * y * z;
                }
            }
        }
    }

    return 0;
}

aoc_test! {
  input = "
    1721
    979
    366
    299
    675
    1456
  ";

  part1 = "514579";
  part2 = "241861950";
}

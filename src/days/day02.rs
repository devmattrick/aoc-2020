use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use regex::{Regex, Match};

use crate::aoc_test;

#[derive(Debug)]
struct PasswordPolicy {
  min: u8,
  max: u8,
  letter: char,
  password: String,
}

#[aoc_generator(day2)]
fn generator(input: &str) -> Vec<PasswordPolicy> {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").expect("Failed to initialize regex");
  }
  let input = input.trim();

  input.lines().map(|x| {
    let caps = RE.captures(x).expect("Unable to parse input");

    let min = parse_number(caps.get(1));
    let max = parse_number(caps.get(2));
    let letter = parse_char(caps.get(3));
    let password = parse_string(caps.get(4));

    PasswordPolicy {
      min,
      max,
      letter,
      password,
    }
  }).collect()
}

fn parse_number(input: Option<Match<>>) -> u8 {
  input.expect("Missing input value").as_str().parse::<u8>().expect("Unable to parse number")
}

fn parse_char(input: Option<Match<>>) -> char {
  input.expect("Missing input value").as_str().chars().next().expect("Unable to get char")
}

fn parse_string(input:  Option<Match<>>) -> String {
  String::from(input.expect("Missing input value").as_str())
}

#[aoc(day2, part1)]
fn part1(policies: &[PasswordPolicy]) -> u16 {
  let mut valid = 0;

  for policy in policies {
    let matches = policy.password.matches(policy.letter).count();
    let matches = matches as u8;

    if matches >= policy.min && matches <= policy.max {
      valid += 1;
    }
  }

  valid
}

#[aoc(day2, part2)]
fn part2(policies: &[PasswordPolicy]) -> u16 {
  let mut valid = 0;

  for policy in policies {
    let min_result = char_at(&policy.password, policy.min as usize) == policy.letter;
    let max_result = char_at(&policy.password, policy.max as usize) == policy.letter;

    if min_result != max_result {
      valid += 1;
    }
  }

  valid
}

fn char_at(s: &String, i: usize) -> char {
  s.chars().nth(i - 1).expect("Index out of bounds")
}

aoc_test!{
  input = "1-3 a: abcde
           1-3 b: cdefg
           2-9 c: ccccccccc";

  part1 = "2";
  part2 = "1";
}

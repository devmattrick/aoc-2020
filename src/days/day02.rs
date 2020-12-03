#[derive(Debug)]
struct PasswordPolicy {
  min: u8,
  max: u8,
  letter: char,
  password: String,
}

fn part1(policies: Vec<PasswordPolicy>) -> u16 {
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

fn part2(policies: Vec<PasswordPolicy>) -> u16 {
  let mut valid = 0;

  for policy in policies {
    let min_result = char_at(&policy.password, policy.min as usize) != policy.letter;
    let max_result = char_at(&policy.password, policy.max as usize) != policy.letter;

    if min_result != max_result {
      valid += 1;
    }
  }

  valid
}

fn char_at(s: &String, i: usize) -> char {
  s.chars().nth(i - 1).expect("Index out of bounds")
}

#[cfg(test)]
mod tests {
  use std::fs;
  use regex::{Regex, Match};
  use super::*;

  fn read_input() -> Vec<PasswordPolicy> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").expect("Failed to initialize regex");
    }

    let input = fs::read_to_string("input/02.txt").expect("Failed to read file");
    let input = input.trim_end();

    input.lines().map(|x| {
      let caps = RE.captures(x).expect("Unable to parse input");

      let min = parse_number(caps.get(1));
      let max = parse_number(caps.get(2));

      PasswordPolicy {
        min,
        max,
        letter: parse_char(caps.get(3)),
        password: parse_string(caps.get(4)),
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

  #[test]
  fn test_part1() {
    println!("Part 1 Answer: {}", part1(read_input()));
  }

  #[test]
  fn test_part2() {
    println!("Part 2 Answer: {}", part2(read_input()));
  }
}

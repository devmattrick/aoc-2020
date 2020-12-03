const TARGET: u32 = 2020;

fn part1(input: Vec<u32>) -> u32 {
    for x in &input {
        for y in &input {
            if x + y == TARGET {
                return x * y;
            }
        }
    }

    return 0;
}

fn part2(input: Vec<u32>) -> u32 {
    for x in &input {
        for y in &input {
            for z in &input {
                if x + y + z == TARGET {
                    return x * y * z;
                }
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
  use std::fs;
  use super::*;

  fn read_input() -> Vec<u32> {
    let input = fs::read_to_string("input/01.txt").expect("Failed to read file");
    let input = input.trim_end();

    input.lines().map(|x| x.parse::<u32>().unwrap()).collect()
  }

  #[test]
  fn test_day01_part1() {
    let answer = part1(read_input());

    println!("Part 1 Answer: {}", answer);
  }

  #[test]
  fn test_day01_part2() {
    let answer = part2(read_input());

    println!("Part 2 Answer: {}", answer);
  }
}

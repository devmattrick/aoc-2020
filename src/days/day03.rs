use std::cell::RefCell;

struct BoolWrapGrid {
  data: RefCell<Vec<bool>>,
  width: usize,
  height: usize,
}

impl BoolWrapGrid {
  fn new(width: usize, height: usize) -> BoolWrapGrid {
    BoolWrapGrid {
      data: RefCell::new(vec![false; width * height]),
      width,
      height,
    }
  }

  fn get(&self, x: usize, y: usize) -> bool {
    *self.data.borrow().get(self.index(x, y)).expect("Index out of bounds")
  }

  fn set(&self, x: usize, y: usize, value: bool) {
    self.data.borrow_mut().insert(self.index(x, y), value);
  }

  fn index(&self, x: usize, y: usize) -> usize {
    (x % self.width) + self.width * (y % self.height)
  }
}

fn part1(input: BoolWrapGrid) -> u32 {
  num_trees(&input, 3, 1)
}

fn part2(input: BoolWrapGrid) -> u32 {
  num_trees(&input, 1, 1) * num_trees(&input, 3, 1) * num_trees(&input, 5, 1) * num_trees(&input, 7, 1) * num_trees(&input, 1, 2)
}

fn num_trees(input: &BoolWrapGrid, step_x: usize, step_y: usize) -> u32 {
  let mut trees = 0;

  let mut x = step_x;
  let mut y = step_y;

  while y < input.height {
    if input.get(x, y) {
      trees += 1;
    }

    x += step_x;
    y += step_y;
  }

  trees
}

#[cfg(test)]
mod tests {
  use std::fs;
  use super::*;

  fn read_input() -> BoolWrapGrid {
    let input = fs::read_to_string("input/03.txt").expect("Failed to read file");
    let input: Vec<&str> = input.trim_end().lines().collect();

    let width = input.get(0).expect("Missing first element").len();
    let height = input.len();

    let grid = BoolWrapGrid::new(width, height);

    for (y, line) in input.iter().enumerate() {
      for (x, tile) in line.chars().enumerate() {
        if tile == '#' {
          grid.set(x, y, true);
        }
      }
    }

    grid
  }

  #[test]
  fn test_day03_part1() {
    let answer = part1(read_input());

    println!("Part 1 Answer: {}", answer);
  }

  #[test]
  fn test_day03_part2() {
    let answer = part2(read_input());

    println!("Part 2 Answer: {}", answer);
  }
}

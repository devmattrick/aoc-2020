#[derive(Copy, Clone)]
enum Tile {
  Tree,
  Empty,
}

struct WrapGrid {
  data: Vec<Tile>,
  width: usize,
  height: usize,
}

impl WrapGrid {
  fn get(&self, x: usize, y: usize) -> Tile {
    *self.data.get(self.index(x, y)).expect("Index out of bounds")
  }

  fn index(&self, x: usize, y: usize) -> usize {
    (x % self.width) + self.width * (y % self.height)
  }
}

fn part1(input: WrapGrid) -> u32 {
  num_trees(&input, 3, 1)
}

fn part2(input: WrapGrid) -> u32 {
  num_trees(&input, 1, 1) * num_trees(&input, 3, 1) * num_trees(&input, 5, 1) * num_trees(&input, 7, 1) * num_trees(&input, 1, 2)
}

fn num_trees(input: &WrapGrid, step_x: usize, step_y: usize) -> u32 {
  let mut trees = 0;

  let mut x = step_x;
  let mut y = step_y;

  while y < input.height {
    match input.get(x, y) {
      Tile::Tree => trees += 1,
      _ => (),
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

  fn read_input() -> WrapGrid {
    let input = fs::read_to_string("input/03.txt").expect("Failed to read file");
    let input: Vec<&str> = input.trim_end().lines().collect();

    let width = input.get(0).expect("Missing first element").len();
    let height = input.len();

    let mut data = Vec::new();

    for tile in input.join("").chars() {
      data.push(match tile {
        '#' => Tile::Tree,
        '.' => Tile::Empty,
        _ => panic!("Unrecognized character"),
      })
    }

    WrapGrid {
      data,
      width,
      height,
    }
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

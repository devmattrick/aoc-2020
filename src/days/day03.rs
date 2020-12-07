use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use crate::aoc_test;

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

#[aoc_generator(day3)]
fn generator(input: &str) -> WrapGrid {
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

#[aoc(day3, part1)]
fn part1(input: &WrapGrid) -> u32 {
  num_trees(input, 3, 1)
}

#[aoc(day3, part2)]
fn part2(input: &WrapGrid) -> u32 {
  num_trees(input, 1, 1) * num_trees(input, 3, 1) * num_trees(input, 5, 1) * num_trees(input, 7, 1) * num_trees(input, 1, 2)
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

aoc_test! {
  input = "
    ..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#
  ";

  part1 = "7";
  part2 = "336";
}

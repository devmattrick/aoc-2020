use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::str::FromStr;
use std::mem;
use std::fmt;

#[derive(Copy, Clone, Debug)]
enum Seat {
  Occupied,
  Empty,
  Floor,
}

impl FromStr for Seat {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "L" => Ok(Self::Empty),
      "#" => Ok(Self::Occupied),
      "." => Ok(Self::Floor),
      _ => Err("Invalid input string"),
    }
  }
}

const DIRECTIONS: [(i32, i32); 8] = [
  (-1, -1), ( 0, -1), ( 1, -1),
  (-1,  0),           ( 1,  0),
  (-1,  1), ( 0,  1), ( 1,  1),
];

#[derive(Clone)]
struct Grid {
  data: Vec<Vec<Seat>>,
}

impl Grid {
  fn get(&self, x: i32, y: i32) -> Seat {
    self.data[x as usize][y as usize]
  }

  fn in_bounds(&self, x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < self.data.len() as i32 && y < self.data[0].len() as i32
  }

  fn run<F: Fn(&mut Self) -> bool>(&mut self, sim: F) -> bool {
    sim(self)
  }

  fn occupied(&self) -> u32 {
    self.data.iter().flatten().filter(|seat| match seat {
      Seat::Occupied => true,
      _ => false,
    }).count() as u32
  }

  fn adj(&self, x: i32, y: i32) -> i32 {
    let mut count = 0;

    for (dx, dy) in &DIRECTIONS {
      let x = x + dx;
      let y = y + dy;

      if !self.in_bounds(x, y) {
        continue;
      }

      match self.get(x, y) {
        Seat::Occupied => count += 1,
        _ => (),
      }
    }

    count
  }

  fn los(&self, x: i32, y: i32) -> i32 {
    let mut count = 0;

    for (dx, dy) in &DIRECTIONS {
      let mut x = x + dx;
      let mut y = y + dy;

      while self.in_bounds(x, y) {
        match self.get(x, y) {
          Seat::Occupied => {
            count += 1;
            break;
          },
          Seat::Empty => break,
          _ => (),
        }

        x += dx;
        y += dy;
      }
    }

    count
  }
}

impl FromStr for Grid {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let data = s.lines()
      .map(|line| {
        line.chars().map(|c| Seat::from_str(&*c.to_string()).unwrap()).collect::<Vec<Seat>>()
      })
      .collect::<Vec<Vec<Seat>>>();

    Ok(Grid {
      data
    })
  }
}

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      for x in 0..self.data.len() {
        for y in 0..self.data[0].len() {
        write!(f, "{}", match self.data[x][y] {
          Seat::Occupied => "#",
          Seat::Empty => "L",
          Seat::Floor => ".",
        })?;
      }

      writeln!(f, "")?;
    }

    Ok(())
  }
}

#[aoc_generator(day11)]
fn generator(input: &str) -> Grid {
  Grid::from_str(input).expect("Unable to parse input")
}

#[aoc(day11, part1)]
fn part1(input: &Grid) -> u32 {
  let mut input = (*input).clone();

  let sim = |grid: &mut Grid| {
    let mut modified = false;
    let mut new_data = grid.data.clone();

    for x in 0..grid.data.len() {
      for y in 0..grid.data[0].len() {
        let seat = grid.data[x][y];
        let adj = grid.adj(x as i32, y as i32);

        match seat {
          Seat::Occupied => {
            if adj >= 4 {
              modified = true;
              new_data[x][y] = Seat::Empty;
            }
          },
          Seat::Empty => {
            if adj == 0 {
              modified = true;
              new_data[x][y] = Seat::Occupied;
            }
          },
          _ => (),
        }
      }
    }

    mem::swap(&mut grid.data, &mut new_data);
    modified
  };

  while input.run(sim) {}

  input.occupied()
}

#[aoc(day11, part2)]
fn part2(input: &Grid) -> u32 {
  let mut input = (*input).clone();

  let sim = |grid: &mut Grid| {
    let mut modified = false;
    let mut new_data = grid.data.clone();

    for x in 0..grid.data.len() {
      for y in 0..grid.data[0].len() {
        let seat = grid.data[x][y];
        let adj = grid.los(x as i32, y as i32);

        match seat {
          Seat::Occupied => {
            if adj >= 5 {
              modified = true;
              new_data[x][y] = Seat::Empty;
            }
          },
          Seat::Empty => {
            if adj == 0 {
              modified = true;
              new_data[x][y] = Seat::Occupied;
            }
          },
          _ => (),
        }
      }
    }

    mem::swap(&mut grid.data, &mut new_data);
    modified
  };

  while input.run(sim) {}

  input.occupied()
}

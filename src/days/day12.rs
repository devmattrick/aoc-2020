use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::convert::TryFrom;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(Angle),
    Right(Angle),
    Forward(i32),
}

impl FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let act = chars.next().ok_or("Unable to get first char")?;
        let arg = chars
            .as_str()
            .parse::<i32>()
            .map_err(|_| "Unable to parse arg")?;

        match act {
            'N' => Ok(Action::North(arg)),
            'S' => Ok(Action::South(arg)),
            'E' => Ok(Action::East(arg)),
            'W' => Ok(Action::West(arg)),
            'L' => Ok(Action::Left(Angle::try_from(arg)?)),
            'R' => Ok(Action::Right(Angle::try_from(arg)?)),
            'F' => Ok(Action::Forward(arg)),
            _ => Err("Invalid action"),
        }
    }
}

#[derive(Copy, Clone)]
enum Angle {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Angle {
    fn as_i32(&self) -> i32 {
        match self {
            Angle::Deg0 => 0,
            Angle::Deg90 => 90,
            Angle::Deg180 => 180,
            Angle::Deg270 => 270,
        }
    }
}

impl TryFrom<i32> for Angle {
    type Error = &'static str;

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        let mut n = n % 360;
        if n < 0 {
            n += 360;
        }

        match n {
            0 => Ok(Angle::Deg0),
            90 => Ok(Angle::Deg90),
            180 => Ok(Angle::Deg180),
            270 => Ok(Angle::Deg270),
            _ => Err("Invalid angle"),
        }
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let new_angle = self.as_i32() + other.as_i32();

        Self::try_from(new_angle).expect("Unable to add angles")
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let new_angle = self.as_i32() - other.as_i32();

        Self::try_from(new_angle).expect("Unable to add angles")
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, other: Self) {
        let new_angle = self.as_i32() + other.as_i32();

        *self = Self::try_from(new_angle).expect("Unable to add angles");
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, other: Self) {
        let new_angle = self.as_i32() - other.as_i32();

        *self = Self::try_from(new_angle).expect("Unable to add angles");
    }
}

#[aoc_generator(day12)]
fn generator(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| Action::from_str(line).expect("Unable to parse action"))
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &[Action]) -> i32 {
    let mut x = 0;
    let mut y = 0;

    let mut angle = Angle::Deg0;
    for action in input {
        match action {
            Action::North(step) => y += step,
            Action::South(step) => y -= step,
            Action::East(step) => x += step,
            Action::West(step) => x -= step,
            Action::Left(d_angle) => angle += *d_angle,
            Action::Right(d_angle) => angle -= *d_angle,
            Action::Forward(step) => match angle {
                Angle::Deg0 => x += step,
                Angle::Deg90 => y += step,
                Angle::Deg180 => x -= step,
                Angle::Deg270 => y -= step,
            },
        }
    }

    return x.abs() + y.abs();
}

#[aoc(day12, part2)]
fn part2(input: &[Action]) -> i32 {
    let mut way_x = 10;
    let mut way_y = 1;
    let mut ship_x = 0;
    let mut ship_y = 0;

    for action in input {
        match action {
            Action::North(step) => way_y += step,
            Action::South(step) => way_y -= step,
            Action::East(step) => way_x += step,
            Action::West(step) => way_x -= step,
            Action::Left(d_angle) => {
                let tmp_x = way_x;

                match d_angle {
                    Angle::Deg90 => {
                        way_x = -way_y;
                        way_y = tmp_x;
                    }
                    Angle::Deg180 => {
                        way_x = -way_x;
                        way_y = -way_y;
                    }
                    Angle::Deg270 => {
                        way_x = way_y;
                        way_y = -tmp_x;
                    }
                    _ => (),
                }
            }
            Action::Right(d_angle) => {
                let tmp_x = way_x;

                match d_angle {
                    Angle::Deg90 => {
                        way_x = way_y;
                        way_y = -tmp_x;
                    }
                    Angle::Deg180 => {
                        way_x = -way_x;
                        way_y = -way_y;
                    }
                    Angle::Deg270 => {
                        way_x = -way_y;
                        way_y = tmp_x;
                    }
                    _ => (),
                }
            }
            Action::Forward(step) => {
                ship_x += way_x * step;
                ship_y += way_y * step;
            }
        }
    }

    return ship_x.abs() + ship_y.abs();
}

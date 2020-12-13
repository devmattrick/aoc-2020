use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::cmp::Ordering;

#[aoc_generator(day5)]
fn generator(input: &str) -> Vec<Vec<Direction>> {
    let input = input.lines();

    input
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'F' => Direction::Front,
                    'B' => Direction::Back,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unrecognized character: {}", c),
                })
                .collect()
        })
        .collect()
}

const MAX_Y: u8 = 127;
const MAX_X: u8 = 7;

enum Direction {
    Front,
    Back,
    Left,
    Right,
}

#[derive(Eq)]
struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    fn new(partitions: &[Direction]) -> Seat {
        let mut front = 0;
        let mut back = MAX_Y;
        let mut left = 0;
        let mut right = MAX_X;

        for partition in partitions {
            match partition {
                Direction::Front => back = (front + back) / 2,
                Direction::Back => front = ((front + back) / 2) + 1,
                Direction::Left => right = (left + right) / 2,
                Direction::Right => left = ((left + right) / 2) + 1,
            }
        }

        Seat {
            row: front,
            col: left,
        }
    }

    fn seat_id(&self) -> u16 {
        (self.row as u16 * 8) + self.col as u16
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seat_id().cmp(&other.seat_id())
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self.seat_id() == other.seat_id()
    }
}

#[aoc(day5, part1)]
fn part1(input: &[Vec<Direction>]) -> u16 {
    let mut seats = Vec::new();

    for seat in input {
        seats.push(Seat::new(seat));
    }

    seats
        .iter()
        .max()
        .expect("Unable to find max seat_id")
        .seat_id()
}

#[aoc(day5, part2)]
fn part2(input: &[Vec<Direction>]) -> u16 {
    let mut seats = Vec::new();

    for seat in input {
        seats.push(Seat::new(seat).seat_id());
    }

    seats.sort();

    *seats
        .iter()
        .enumerate()
        .find(|(i, &id)| match seats.get(i + 1) {
            Some(&next_id) => id != next_id - 1,
            None => false,
        })
        .expect("Unable to find valid seat value")
        .1
        + 1
}

// TODO Write tests

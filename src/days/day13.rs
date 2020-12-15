use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::str::FromStr;

enum Line {
    Bus(u64),
    None,
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "x" {
            return Ok(Line::None);
        }

        let n = s.parse::<u64>().map_err(|_| "Unable to parse number")?;
        Ok(Line::Bus(n))
    }
}

#[aoc_generator(day13)]
fn generator(input: &str) -> (u64, Vec<Line>) {
    let mut input = input.lines();

    let time = input
        .next()
        .expect("Unable to get first line")
        .parse::<u64>()
        .expect("Unable to parse number");
    let busses = input
        .next()
        .expect("Unable to get second line")
        .split(",")
        .map(|id| Line::from_str(id).expect("Unable to parse number"))
        .collect();

    (time, busses)
}

#[aoc(day13, part1)]
fn part1(input: &(u64, Vec<Line>)) -> u64 {
    let start_time = input.0;
    let busses = &(*input)
        .1
        .iter()
        // Remove all instances of "x"
        .filter_map(|id| match id {
            Line::Bus(bus_id) => Some(*bus_id),
            _ => None,
        })
        .collect::<Vec<u64>>();

    let mut time = start_time;
    loop {
        for bus in busses {
            if time % bus == 0 {
                return (time - start_time) * bus;
            }
        }

        time += 1;
    }
}

#[aoc(day13, part2)]
fn part2(input: &(u64, Vec<Line>)) -> u64 {
    let busses = (*input).1.iter().enumerate();

    let mut time: u64 = 0;
    let mut jump: u64 = 1;
    for (i, line) in busses {
        let i = i as u64;

        match line {
            Line::Bus(id) => {
                while (time + i) % id != 0 {
                    time += jump;
                }

                jump *= id;
            },
            _ => (),
        }
    }

    time
}

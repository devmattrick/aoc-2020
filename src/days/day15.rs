use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;

#[aoc_generator(day15)]
fn generator(input: &str) -> Vec<u32> {
    input.split(",").map(|s| s.parse::<u32>().expect("Unable to parse u32")).collect()
}

#[aoc(day15, part1)]
fn part1(input: &[u32]) -> u32 {
    let len = input.len();

    // Convert input into a vec of tuples like (val, i)
    let mut input = input.iter().enumerate().map(|(i, val)| (*val, i)).collect::<Vec<(u32, usize)>>();
    let mut used = HashMap::new();

    let last = input.remove(input.len() - 1).0;

    // Add the initial state to the map. The keys are the numbers and the values are the index they last appeared
    used.extend(input);

    let mut prev = last;
    for n in len - 1..=2018 {
        let search = used.get(&prev);

        //println!("{:#?}", used);

        let curr = match search {
            Some(i) => (n - i) as u32,
            None => 0,
        };

        used.insert(prev, n);
        prev = curr;
    }

    prev
}


#[aoc(day15, part2)]
fn part2(input: &[u32]) -> u32 {
    let len = input.len();

    // Convert input into a vec of tuples like (val, i)
    let mut input = input.iter().enumerate().map(|(i, val)| (*val, i)).collect::<Vec<(u32, usize)>>();
    let mut used = HashMap::new();

    let last = input.remove(input.len() - 1).0;

    // Add the initial state to the map. The keys are the numbers and the values are the index they last appeared
    used.extend(input);

    let mut prev = last;
    for n in len - 1..=29999998 {
        let search = used.get(&prev);

        //println!("{:#?}", used);

        let curr = match search {
            Some(i) => (n - i) as u32,
            None => 0,
        };

        used.insert(prev, n);
        prev = curr;
    }

    prev
}

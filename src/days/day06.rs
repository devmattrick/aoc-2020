use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use itertools::Itertools;

use crate::aoc_test;

#[aoc_generator(day6)]
fn generator(input: &str) -> Vec<String> {
    let input: Vec<&str> = input.lines().collect();

    // Separate groups into discrete entries
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in input {
        if line.is_empty() {
            groups.push(group.join("\n"));
            group = Vec::new();
            continue;
        }

        group.push(line);
    }

    // Push final group
    groups.push(group.join("\n"));

    groups
}

#[aoc(day6, part1)]
fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|group| group.lines().join("").chars().unique().count() as u32)
        .sum::<u32>()
}

#[aoc(day6, part2)]
fn part2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|group| {
            let group: Vec<&str> = group.lines().collect();
            let members = group.len();

            let first_person = group.get(0).expect("Empty group");
            let group_string = group.join("");

            let res = first_person
                .chars()
                .map(|c| group_string.matches(c).count())
                .filter(|&count| count == members)
                .collect::<Vec<usize>>()
                .len();

            res as u32
        })
        .sum::<u32>()
}

aoc_test! {
  input = "
    abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b
  ";

  part1 = "11";
  part2 = "6";
}

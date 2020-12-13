use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use pest::Parser;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{Directed, Direction, Graph};
use std::collections::{HashMap, HashSet};

use crate::aoc_test;

#[derive(Parser)]
#[grammar = "../resources/day07.pest"]
struct BagParser;

#[derive(Debug)]
struct BagContents {
    color: String,
    count: u8,
}

#[aoc_generator(day7)]
fn generator(input: &str) -> Graph<String, u8> {
    let input = input.lines();

    let mut bags: HashMap<String, Vec<BagContents>> = HashMap::new();

    // Its overkill using a parser generator for this but I'll be damned if I never use the stuff I learned in Compilers
    input.for_each(|line| {
        let parsed = BagParser::parse(Rule::input, line)
            .expect("Unable to parse rule")
            .next()
            .unwrap();

        for record in parsed.into_inner() {
            match record.as_rule() {
                Rule::rule => {
                    let mut inner_rules = record.into_inner();
                    let bag = inner_rules.next().unwrap().as_str();
                    let bag = bags.entry(bag.to_owned()).or_default();

                    let inner_bags = inner_rules.next().unwrap().into_inner();

                    inner_bags.for_each(|inner_bag| {
                        let mut inner_bag = inner_bag.into_inner();

                        let count = inner_bag
                            .next()
                            .unwrap()
                            .as_str()
                            .parse::<u8>()
                            .expect("Unable to parse string into u8");
                        let color = inner_bag.next().unwrap().as_str().to_owned();

                        bag.push(BagContents { color, count });
                    });
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
    });

    // Keep track of vertex indicies to make looking them up by name easier
    let mut verticies = HashMap::new();
    let mut graph = Graph::<String, u8, Directed>::new();

    // Add all keys into graph so we can add edges without worrying about them not being in the graph
    bags.keys().for_each(|key| {
        let vertex = graph.add_node(key.clone());
        verticies.insert(key, vertex);
    });

    bags.iter().for_each(|(key, value)| {
        let from = verticies.get(key).expect("Unable to find vertex id");

        value.iter().for_each(|bag| {
            let to = verticies.get(&bag.color).expect("Unable to find vertex id");

            graph.add_edge(*from, *to, bag.count.into());
        });
    });

    graph
}

#[aoc(day7, part1)]
fn part1(input: &Graph<String, u8>) -> usize {
    let target = input
        .node_indices()
        .filter(|node| input.node_weight(*node).unwrap() == "shiny gold")
        .next()
        .unwrap();

    let connected = find_connected(&input, target);

    connected.len()
}

fn find_connected<N, E>(graph: &Graph<N, E>, search: NodeIndex) -> HashSet<NodeIndex> {
    let mut set = HashSet::new();

    for connection in graph.neighbors_directed(search, Direction::Incoming) {
        set.insert(connection);
        set.extend(find_connected(graph, connection));
    }

    set
}

#[aoc(day7, part2)]
fn part2(input: &Graph<String, u8>) -> u32 {
    let target = input
        .node_indices()
        .filter(|node| input.node_weight(*node).unwrap() == "shiny gold")
        .next()
        .unwrap();

    count_weights(input, target) - 1
}

fn count_weights<N>(graph: &Graph<N, u8>, search: NodeIndex) -> u32 {
    let mut count = 1;

    for edge in graph.edges(search) {
        count += *edge.weight() as u32 * count_weights(graph, edge.target());
    }

    count
}

aoc_test! {
  input = "
    light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.
  ";

  part1 = "4";
  part2 = "32";
}

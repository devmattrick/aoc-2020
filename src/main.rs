#[macro_use] extern crate lazy_static;
#[allow(unused_imports)]
#[macro_use] extern crate aoc_runner_derive;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod days;
mod macros;

use aoc_runner_derive::aoc_main;

aoc_main! { year = 2020 }

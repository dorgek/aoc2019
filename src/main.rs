#[allow(dead_code)]

extern crate approx;

#[path = "day05/challenge.rs"] mod day05;
#[path = "day06/challenge.rs"] mod day06;
#[path = "day07/challenge.rs"] mod day07;
#[path = "day08/challenge.rs"] mod day08;
#[path = "day09/challenge.rs"] mod day09;
#[path = "day10/challenge.rs"] mod day10;
#[path = "day11/challenge.rs"] mod day11;
#[path = "day12/challenge.rs"] mod day12;
#[path = "day13/challenge.rs"] mod day13;
#[path = "day14/challenge.rs"] mod day14;
#[path = "day15/challenge.rs"] mod day15;
#[path = "day16/challenge.rs"] mod day16;
#[path = "day17/challenge.rs"] mod day17;
#[path = "day18/challenge.rs"] mod day18;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    day17::day_17( args );
}

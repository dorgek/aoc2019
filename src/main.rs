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
#[path = "day19/challenge.rs"] mod day19;
#[path = "day20/challenge.rs"] mod day20;
#[path = "day21/challenge.rs"] mod day21;
#[path = "day22/challenge.rs"] mod day22;
#[path = "day23/challenge.rs"] mod day23;
#[path = "day24/challenge.rs"] mod day24;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    day24::day_24( args );
}
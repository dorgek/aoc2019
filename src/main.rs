#[path = "day05/challenge.rs"] mod day05;
#[path = "day06/challenge.rs"] mod day06;
#[path = "day07/challenge.rs"] mod day07;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    day07::day_07( args );
}

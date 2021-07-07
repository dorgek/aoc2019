#[path = "day06/challenge.rs"] mod day06;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    day06::day_06( args );
}

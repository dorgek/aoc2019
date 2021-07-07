use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

fn calculate_fuel( mass: f64 ) -> f64 {
    return ( mass / 3.0 ).floor() - 2.0;
}

fn lines_from_file( filename: impl AsRef<Path> ) -> Vec<f64> {
    let file = File::open( filename ).expect( "no such file" );
    let buf = BufReader::new( file );

    return buf.lines()
        .map( |l| l.expect( "could not parse line" ).parse().unwrap() ) 
        .collect();
}

fn calculate_fuel_and_sum( masses: Vec<f64> ) -> f64 {
    let mut sum: f64 = 0.0;

    for mass in masses {
        sum += calculate_fuel( mass );
    }

    return sum;
}

fn calculate_fuel_mass( mass: f64 ) -> f64 {
    let mut calculate_fuel_val: f64 = calculate_fuel( mass );
    let mut total_fuel: f64 = 0.0;

    while calculate_fuel_val > 0.0 {
        total_fuel += calculate_fuel_val;
        calculate_fuel_val = calculate_fuel( calculate_fuel_val );
    }

    return total_fuel;
}

fn calculate_all_fuel_mass( masses: Vec<f64> ) -> f64 {
    let mut sum: f64 = 0.0;

    for mass in masses {
        sum += calculate_fuel_mass( mass );
    }

    return sum;
}

fn main() {
    let filename = "./day01/puzzleInput.txt";
    let lines = lines_from_file( filename );

    println!( "Part One: sum of masses {}", calculate_fuel_and_sum( lines.clone() ) );
    println!( "Part Two: sum of masses {}", calculate_all_fuel_mass( lines.clone() ) );
}
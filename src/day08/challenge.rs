use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

use ndarray::{
    Array2
};

static WIDTH: usize = 25;
static HEIGHT: usize = 6;

fn process_data( filename: impl AsRef<Path> ) -> Vec::<Array2<i64>> {
    let file = File::open( filename ).expect( "no such file" );
    let buf  = BufReader::new( file );
    let mut ret = Vec::<Array2<i64>>::new();
    let mut x = 0;
    let mut y = 0;

    for line in buf.lines().filter_map( |result| result.ok() ) {

        // layer
        let mut layer = Array2::<i64>::zeros( ( HEIGHT, WIDTH ) );

        // get every character
        for ( _i, c ) in line.chars().enumerate() {
            layer[[y, x]] = c.to_digit(10).unwrap() as i64;
            
            x += 1;

            if x == WIDTH { // TODO: workout a way to do it with the numeration
                x = 0;
                y += 1;
            }

            if y == HEIGHT {
                x = 0; 
                y = 0;

                ret.push( layer );
                layer = Array2::<i64>::zeros( ( HEIGHT, WIDTH ) );
            }
        }
    }

    return ret;
}

fn part_one( input_data: Vec<Array2<i64>> ) {
    let mut num_zeros = i64::max_value();
    let mut layer_num = 0;
    let mut i = 1;

    for layer in input_data.clone() {
        let count = layer.iter()
                        .filter( |x| **x == 0 )
                        .count();

        if count < num_zeros as usize {
            layer_num = i;
            num_zeros = count as i64;
        }

        i += 1;
    }

    let num_ones = input_data.clone()[layer_num-1]
            .iter()
            .filter( |x| **x == 1 )
            .count();

    let num_twos = input_data.clone()[layer_num-1]
            .iter()
            .filter( |x| **x == 2 )
            .count();

    println!( "Part One: {}", num_ones * num_twos );
}


fn part_two( input_data: Vec<Array2<i64>> ) {
    let mut image = Array2::<i64>::zeros( ( HEIGHT, WIDTH ) );

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut current_layer = 0;


            while input_data[current_layer][[y, x]] == 2 {
                current_layer += 1;
            }

            image[[y, x]] = input_data[current_layer][[y, x]];
        }
    }

    println!( "Part Two: " );

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            print!( "{}", get_pixel( x, y, image.clone() ) );
        }

        println!();
    }
}

fn get_pixel( x: usize, y: usize, image: Array2<i64> ) -> &'static str {
    return match image[[y,x]] {
        1 => "##", 
        _ => "  "
    };
}

#[allow(dead_code)]
pub fn day_08( args: Vec<String> ) {
    let input_data = process_data( args[1].clone() );
    part_one( input_data.clone() );
    part_two( input_data.clone() );
}
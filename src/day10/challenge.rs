use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

use ndarray::{
    Array2
};

use std::f64::consts::PI;

use approx::abs_diff_eq;

static WIDTH: usize = 33;
static HEIGHT: usize = 33;


fn process_data( filename: impl AsRef<Path> ) -> Array2<i64> {
    let file = File::open( filename ).expect( "no such file" );
    let buf  = BufReader::new( file );
    let mut ret = Array2::<i64>::zeros( ( HEIGHT, WIDTH ) );

    buf.lines()
        .enumerate()
        .for_each( |( y, l )| {
            l.expect( "could not parse line" )
                .chars()
                .enumerate()
                .for_each( |( x, c )| {
                    match c {
                        '.' => ret[[y, x]] = 0,
                        _ => ret[[y, x]] = 1
                    }
                })
        });


    return ret;
}

fn calculate_angle( current_location: ( f64, f64 ), asteroid_location: ( f64, f64 ) ) -> f64 {
    let angle = ( asteroid_location.1 - current_location.1 ).atan2( asteroid_location.0 - current_location.0 );

    if angle < 0.0 {
        return 2.0 * PI + angle;
    }

    return angle;
}

fn part_one( data: Array2<i64> ) {
    // get all coordiantes of a bomb
    let asteroids: Vec<( f64, f64 )> = data.indexed_iter()
        .filter( |( ( _y, _x ), val )| **val == 1 )
        .map( |( ( y, x ), _val )| ( x as f64, y as f64 ) )
        .collect();

    let total_num_asteroids = asteroids.len();
    let mut total_asteroids_visible = 0;
    let mut current_base_location: ( f64, f64 ) = ( 0.0, 0.0 );
        
    for ( i, asteroid ) in asteroids.iter().enumerate() {
        let slice = [&asteroids[0.. i], &asteroids[i + 1.. total_num_asteroids]].concat();
        let mut angles: Vec<f64> = vec![];
        let mut asteroid_count = 0;

        for viewable_ateroid in slice {
            let mut visible = true;
            let asteroid_angle = calculate_angle( *asteroid, viewable_ateroid );
            
            // check if asteroid is visible from current asteroid
            for angle in angles.clone() {
                if abs_diff_eq!( angle, asteroid_angle ) {
                    visible = false;
                }
            }

            if visible {
                asteroid_count += 1;
            }

            angles.push( asteroid_angle );
        }

        if total_asteroids_visible < asteroid_count {
            total_asteroids_visible = asteroid_count;
            current_base_location = *asteroid;
        }
    }

    println!( "Part one: x = {}, y = {}, asteroids = {}", current_base_location.0, current_base_location.1, total_asteroids_visible );
}

pub fn day_10( args: Vec<String> ) {
    let data = process_data( args[1].clone() );
    part_one( data );
}
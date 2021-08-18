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

use ordered_float::OrderedFloat;

use std::collections::{
    BTreeMap,
    HashSet
};

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
    let angle = 3.0 * PI - ( ( asteroid_location.0 - current_location.0 ).atan2( asteroid_location.1 - current_location.1 ) );

    if angle < 0.0 {
        return 2.0 * PI + angle;
    }

    return angle;
}

fn calculate_distance( current_location: ( f64, f64 ), asteroid_location: ( f64, f64 ) ) -> f64 {
    return ( ( asteroid_location.1 - current_location.1 ).powi( 2 ) + ( asteroid_location.0 - current_location.0 ).powi( 2 ) ).sqrt();
}

fn part_one( data: Array2<i64> ) -> ( f64, f64 ) {
    // get all coordiantes of asteroids
    let asteroids: Vec<( f64, f64 )> = data.indexed_iter()
        .filter( |( ( _y, _x ), val )| **val == 1 )
        .map( |( ( y, x ), _val )| ( x as f64, y as f64 ) )
        .collect();

    let total_num_asteroids = asteroids.len();
    let mut total_asteroids_visible = 0;
    let mut current_base_location: ( f64, f64 ) = ( 0.0, 0.0 );
        
    for ( i, asteroid ) in asteroids.iter().enumerate() {
        let slice = [&asteroids[0.. i], &asteroids[i + 1.. total_num_asteroids]].concat();
        let mut angles: HashSet<OrderedFloat<f64>> = HashSet::new(); 
        let mut asteroid_count = 0;

        for viewable_ateroid in slice {
            let mut visible = true;
            let asteroid_angle = OrderedFloat( calculate_angle( *asteroid, viewable_ateroid ) );
            
            // check if asteroid is visible from current asteroid
            for angle in angles.clone() {
                if asteroid_angle == angle {
                    visible = false;
                }
            }

            if visible {
                asteroid_count += 1;
            }

            angles.insert( asteroid_angle );
        }

        if total_asteroids_visible < asteroid_count {
            total_asteroids_visible = asteroid_count;
            current_base_location = *asteroid;
        }
    }

    println!( "Part one: x = {}, y = {}, asteroids = {}", current_base_location.0, current_base_location.1, total_asteroids_visible );

    return current_base_location;
}


fn part_two( data: Array2<i64>, base_location: ( f64, f64 ) ) {
    let asteroids: Vec<( f64, f64 )> = data.indexed_iter()
        .filter( |( ( y, x ), val )| **val == 1 || ( abs_diff_eq!( *y as f64, base_location.1 ) && abs_diff_eq!( *x as f64, base_location.0 ) ) )
        .map( |( ( y, x ), _val )| ( x as f64, y as f64 ) )
        .collect();

    let mut i = 0;
    let mut destroyed_asteroid: ( f64, f64 ) = ( 0.0, 0.0 );


    let mut visible_asteroids: BTreeMap<OrderedFloat<f64>, BTreeMap<OrderedFloat<f64>, (f64, f64)>> = BTreeMap::new();

    for ( _i, asteroid ) in asteroids.iter().enumerate() {
        let asteroid_angle = OrderedFloat( calculate_angle( base_location, *asteroid ) );
        let asteroid_distance = OrderedFloat( calculate_distance( base_location, *asteroid ) );

        if let Some( astroid_line ) = visible_asteroids.get_mut( &asteroid_angle ) {
            astroid_line.insert( asteroid_distance, *asteroid );
        } else {
            let mut new_line: BTreeMap::<OrderedFloat<f64>, (f64, f64)> = BTreeMap::new();
            new_line.insert( asteroid_distance, *asteroid );
            visible_asteroids.insert( asteroid_angle, new_line );
        }
    }

    while i < 200 {
        for ( key, _asteroid_line ) in visible_asteroids.clone() {
            if let Some( line ) = visible_asteroids.get_mut( &key ) {
                if let Some( ( distance_key, asteroid ) ) = line.clone().iter().next() {
                    destroyed_asteroid = asteroid.clone();
                    line.remove( &distance_key.clone() );
                }
            }

            i += 1;

            if i == 200 {
                break;
            }
        }
    }

    println!( "Part two: asteroid -> x: {}, y: {}, ans = {}", destroyed_asteroid.0, destroyed_asteroid.1, destroyed_asteroid.0 * 100.0 + destroyed_asteroid.1 );
}

#[allow(dead_code)]
pub fn day_10( args: Vec<String> ) {
    let data = process_data( args[1].clone() );
    let base_location = part_one( data.clone() );
    part_two( data.clone(), base_location );
}
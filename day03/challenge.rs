use std::cmp::min;
use std::cmp::max;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

fn process_input( filename: impl AsRef<Path> ) -> Vec<Vec<(i64, i64)>> {
    let file = File::open( filename ).expect( "no such file" );
    let buf = BufReader::new( file );

    return buf.lines()
        .map( |v| ( process_list( v.expect( "could not parse line" ) ) ) )
        .collect();
}

fn process_list( wire_path: String ) -> Vec<(i64, i64)> {
    
    let mut current_pos = ( 0, 0 );
    let mut ret = Vec::<(i64, i64)>::new();

    for wire_direction in wire_path.split( "," ) {

        let direction = wire_direction.chars()
                            .filter( |&x| x.is_alphabetic() )
                            .collect::<String>();

        let distance: i64 = wire_direction.chars()
                            .filter( |&x| x.is_digit(10) )
                            .collect::<String>()
                            .parse()
                            .unwrap();

        if direction == "R" {
            current_pos.0 += distance;
        } else if direction == "L" {
            current_pos.0 -= distance;
        } else if direction == "U" {
            current_pos.1 += distance;
        } else if direction == "D" {
            current_pos.1 -= distance;
        }

        ret.push( current_pos );
    }

    return ret;
}


fn calculate_collisions( wires: Vec<Vec<(i64, i64)>> ) -> i64 {
    let mut closest_distance_point: i64 = i64::max_value();
    let mut num_steps: i64 = i64::max_value();
    let len = wires.clone().len();
    let mut prev_wire_segment: (i64, i64);
    let mut wire_start: (i64, i64) = ( 0, 0 );

    for wire in 0..wires.len() {
        let wire_positions = wires.get( wire ).expect( "no" );
        
        let mut current_steps_one = 0;

        for wire_pos_iter in 0.. wire_positions.len() {
            let wire_end = *wire_positions.get( wire_pos_iter ).expect( "unknown error" );

            for i in wire + 1.. len {
                let mut current_steps_two = 0;
                let next_wire_positions = wires.get( i ).expect( "why" );
                prev_wire_segment = (0, 0);

                for j in 0.. next_wire_positions.len() {
                    let next_wire_segment = *next_wire_positions.get( j ).expect( "unknown error" );
                    let intersection = intersection( wire_start, wire_end, prev_wire_segment, next_wire_segment );
                    
                    match intersection {
                        Some(x) => {
                            // for part 1
                            let dist = x.0.abs() + x.1.abs();

                            if closest_distance_point > dist && wire_start.0 != 0 && wire_start.1 != 0 {
                                closest_distance_point = dist;
                            }

                            // // for part 2
                            // let new_dist = ( max( prev_wire_segment.0, x.0 ) - min( wire_start.0, x.0 ) ) + ( max( prev_wire_segment.1, x.1 ) - min( prev_wire_segment.1, x.1 ) );
                            // let new_dist_two = ( max( prev_wire_segment.0, x.0 ) - min( prev_wire_segment.0, x.0 ) ) + ( max( prev_wire_segment.1, x.1 ) - min( prev_wire_segment.1, x.1 ) );

                            // if current_steps_one + new_dist + new_dist_two + current_steps_two < num_steps && wire_start.0 != 0 && wire_start.1 != 0 {
                            //     num_steps = current_steps_one + new_dist + new_dist_two + current_steps_two;
                            // }

                            // current_steps_two += ( max( prev_wire_segment.0, next_wire_segment.0 ) - min( prev_wire_segment.0, next_wire_segment.0 ) ) + ( max( prev_wire_segment.1, next_wire_segment.1 ) - min( prev_wire_segment.1, next_wire_segment.1 ) );
                        },
                        None    => {
                            // current_steps_two += ( max( prev_wire_segment.0, next_wire_segment.0 ) - min( prev_wire_segment.0, next_wire_segment.0 ) ) + ( max( prev_wire_segment.1, next_wire_segment.1 ) - min( prev_wire_segment.1, next_wire_segment.1 ) );
                        }
                    };

                    prev_wire_segment = next_wire_segment;
                }

                current_steps_one += ( max( wire_start.0, wire_end.0 ) - min( wire_start.0, wire_end.0 ) ) + ( max( wire_start.1, wire_end.1 ) - min( wire_start.1, wire_end.1 ) );
            }

            wire_start = wire_end;
        }
    }

    return closest_distance_point;
    // return num_steps;
}

fn intersection( point_start: (i64, i64), point_end: (i64, i64), seg_one: (i64, i64), seg_two: (i64, i64) ) -> Option<(i64, i64)> {
    let min_seg_x = min( seg_one.0, seg_two.0 );
    let max_seg_x = max( seg_one.0, seg_two.0 );
    let min_seg_y = min( seg_one.1, seg_two.1 );
    let max_seg_y = max( seg_one.1, seg_two.1 );

    let min_point_x = min( point_start.0, point_end.0 );
    let max_point_x = max( point_start.0, point_end.0 );
    let min_point_y = min( point_start.1, point_end.1 );
    let max_point_y = max( point_start.1, point_end.1 );

    if point_start.0 != point_end.0 {
        if ( min_point_x..=max_point_x ).contains( &seg_one.0 ) && ( min_seg_y..=max_seg_y ).contains( &point_start.1 ) {
            return Some( ( seg_one.0, point_start.1 ) );
        }
    } else if point_start.1 != point_end.1 {
        if ( min_seg_x..=max_seg_x ).contains( &point_start.0 ) && ( min_point_y..=max_point_y ).contains( &seg_one.1 ) {
            return Some( ( point_start.0, seg_one.1 ) );
        }
    }

    None
}

fn main() {
    let filepath = "day03/testInput.txt";
    let wires = process_input( filepath );

    println!( "Answer: {}", calculate_collisions( wires ) );
}
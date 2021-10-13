mod cpu;
use self::cpu::{
    CPU,
    Computer
};

use std::collections::{
    HashMap
};

const DATA: &str = include_str!( "./puzzleInput.txt" );
const WIDTH: u64 = 50;
const HEIGHT: u64 = 50;
const SIZE: i64 = 100;

fn find_start_end( cpu: CPU, l: u64, r: u64, y: u64 ) -> ( i64, i64 ) {
    // find start of aoe
    let mut left: i64 = -1;
    let mut right: i64 = -1;

    for x in l.. r {
        let mut cpu_clone = cpu.clone();
        cpu_clone.set_inputs( vec![ y as i64, x as i64 ] );
        cpu_clone.execute_instructions();
        let result = cpu_clone.get_output_value();  

        if result == 1 {
            left = x as i64;
            break;
        }
    }

    for x in ( l.. r ).rev() {
        let mut cpu_clone = cpu.clone();
        cpu_clone.set_inputs( vec![ y as i64, x as i64 ] );
        cpu_clone.execute_instructions();
        let result = cpu_clone.get_output_value();

        if result == 1 {
            right = x as i64;
            break;
        }
    }

    return ( left, right );
}

fn process_input( input: &str ) -> HashMap<usize, i64> {
    return input.split( "," )
        .enumerate()
        .map( |(i, v)| (i, v.parse().unwrap()))
        .collect();
}

fn part_one( cpu_in: CPU ) {
    let mut cpu = cpu_in.clone();
    cpu.display_std_out( false );
    cpu.pause_execution_on_output( false );
    let mut num_affected = 0;
    let mut prev_coords: ( i64, i64 ) = ( 0, WIDTH as i64 );

    for i in 0.. HEIGHT {
        let coords = find_start_end( cpu.clone(), prev_coords.0 as u64, prev_coords.1 as u64, i );
        
        if coords.0 != -1 && coords.1 != -1 {
            num_affected += coords.1 - coords.0 + 1;

            if coords.0 - 10 < 0 {
                prev_coords.0 = 0;
            } else {
                prev_coords.0 = coords.0 - 10;
            }

            if coords.1 + 10 > WIDTH as i64 {
                prev_coords.1 = WIDTH as i64;
            } else {
                prev_coords.1 = coords.1 + 10;
            }

        }
    }

    println!( "Part one: num affected = {}", num_affected );
}

fn part_two( cpu_in: CPU ) {
    let mut cpu = cpu_in.clone();
    cpu.display_std_out( false );
    cpu.pause_execution_on_output( false );
    let mut count = 0;
    let mut bottom_corner: ( i64, i64 ) = ( 0, 0 );
    let mut top_row: ( i64, i64 ) = ( 0, 0 );
    let mut prev_coords: ( i64, i64 ) = ( 0, WIDTH as i64 );
    let mut i = 0;
    let mut i_temp = 0;
    let mut prev_coords_temp: ( i64, i64 ) = ( 0, WIDTH as i64 );

    while count != SIZE {
        let coords = find_start_end( cpu.clone(), prev_coords.0 as u64, prev_coords.1 as u64, i );

        if coords.0 != -1 && coords.1 != -1 {
            if coords.0 - 5 < 0 {
                prev_coords.0 = 0;
            } else {
                prev_coords.0 = coords.0 - 5;
            }

            prev_coords.1 = coords.1 + 5;

            if coords.1 - coords.0 + 1 >= SIZE {
                bottom_corner = ( coords.0, i as i64 );

                if count == 0 {
                    top_row = coords;
                    i_temp = i;
                    prev_coords_temp = prev_coords;
                } 
                count += 1;
                if !( top_row.1 - bottom_corner.0 + 1 >= SIZE ) {
                    if count > 0 {
                        i = i_temp;
                        prev_coords = prev_coords_temp;
                    }
                    count = 0;
                }


            } else {
                count = 0;
            }
        } else {
            count = 0;
        }

        i += 1;
    }

    println!( "Part two: top corner -> x * 10 000 + y = {}", bottom_corner.0 * 10_000 + ( bottom_corner.1 - SIZE + 1 ) );
}

pub fn day_19( _args: Vec< String > ) {
    let opcodes = process_input( DATA );
    let cpu = CPU::initialise( opcodes.clone() );

    part_one( cpu.clone() );
    part_two( cpu.clone() );
}

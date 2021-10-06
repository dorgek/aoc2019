mod cpu;
use self::cpu::{
    CPU,
    Computer
};

use std::collections::{
    HashMap,
    BTreeMap
};

const DATA: &str = include_str!( "./puzzleInput.txt" );

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64
}

fn process_input( input: &str ) -> HashMap<usize, i64> {
    return input.split( "," )
        .enumerate()
        .map( |(i, v)| (i, v.parse().unwrap()))
        .collect();
}

fn build_map( cpu: &mut CPU ) -> BTreeMap< Coordinate, u8 > {
    cpu.execute_instructions();
    let mut map: BTreeMap< Coordinate, u8 > = BTreeMap::new();
    let mut x = 0;
    let mut y = 0;

    while !cpu.has_finished() {
        let output = cpu.get_output_value() as u8;

        if output == 10 { 
            // new line
            x = 0;
            y += 1;
        } else {
            let coordiante: Coordinate = Coordinate { x: x, y: y };
            map.insert( coordiante, output );
            x += 1;
        }
    
        cpu.execute_instructions();
        print!( "{}", output as char );
    }

    return map;
}

fn check_point( coordiante: Coordinate, map: BTreeMap< Coordinate, u8 > ) -> bool {
    if let Some( value_check ) = map.get( &coordiante ) {
        if *value_check != 35 {
            return false;
        }
    }

    return true;
}

fn intersection( coordinate: Coordinate, map: BTreeMap< Coordinate, u8 > ) -> bool {
    let mut neighbours: Vec< Coordinate > = Vec::new();
    neighbours.push( Coordinate { x: coordinate.x, y: coordinate.y - 1 } );
    neighbours.push( Coordinate { x: coordinate.x - 1, y: coordinate.y } );
    neighbours.push( Coordinate { x: coordinate.x + 1, y: coordinate.y } );
    neighbours.push( Coordinate { x: coordinate.x, y: coordinate.y + 1 } );

    for neighbour in neighbours {
        if !check_point( neighbour, map.clone() ) {
            return false;
        }
    }

    return true;
}

fn find_intersections( map: BTreeMap< Coordinate, u8 > ) -> Vec< Coordinate > {
    let mut intersections: Vec< Coordinate > = Vec::new();

    for key in map.keys() {
        if intersection( *key, map.clone() ) {
            intersections.push( key.clone() );
        }
    }

    return intersections;
}

fn find_allignment_parameters( intersections: Vec< Coordinate > ) -> i64 {
    let mut sum = 0;

    for intersection in intersections {
        sum += intersection.x * intersection.y;
    }

    return sum;
}

fn part_one( mut cpu: CPU ) {
    cpu.display_std_out( false );
    cpu.pause_execution_on_output( true );

    let map = build_map( &mut cpu );
    let intersections = find_intersections( map.clone() );
    
    println!( "Part One: Allignment parameter = {}", find_allignment_parameters( intersections ) );
}

fn part_two( mut cpu: CPU ) {
    let input = "A,B,A,C,A,B,C,A,B,C\nR,8,R,10,R,10\nR,4,R,8,R,10,R,12\nR,12,R,4,L,12,L,12\nn\n";

    cpu.display_std_out( false );
    cpu.pause_execution_on_output( false );

    let mut input_instructions: Vec< i64 > = input.chars()
        .map( |c| c as u8 as i64 )
        .collect();

    input_instructions.reverse();

    cpu.set_inputs( input_instructions );

    while !cpu.has_finished() {
        cpu.execute_instructions();
    }

    println!( "Part two result: {}", cpu.get_output_value() );
}

#[allow(dead_code)]
pub fn day_17( _args: Vec<String> ) {
    let opcodes = process_input( DATA );
    let cpu = CPU::initialise( opcodes.clone() );
    let mut opcodes_two = opcodes.clone();
    opcodes_two.insert( 0, 2 );
    let cpu_two = CPU::initialise( opcodes_two );

    part_one( cpu );
    part_two( cpu_two );
}
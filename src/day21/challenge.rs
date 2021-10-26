mod cpu;
use self::cpu::{
    CPU,
    Computer
};

use std::collections::{
    HashMap
};

const DATA: &str = include_str!( "./puzzleInput.txt" );

fn process_input( input: &str ) -> HashMap<usize, i64> {
    return input.split( "," )
        .enumerate()
        .map( |(i, v)| (i, v.parse().unwrap()))
        .collect();
}

fn springdroid( cpu: &mut CPU, springscript: &str ) {

    let mut input_instructions: Vec< i64 > = springscript.chars()
        .map( |c| c as u8 as i64 )
        .collect();

    input_instructions.reverse();

    cpu.display_std_out( false );
    cpu.pause_execution_on_output( false );

    cpu.set_inputs( input_instructions );
    cpu.execute_instructions();

    println!( "Part one: hull damage = {}", cpu.get_output_value() );
}

fn part_one( cpu: &mut CPU ) {
    let springscript = "NOT C J\n\
                        AND D J\n\
                        NOT A T\n\
                        OR T J\n\
                        WALK\n";

    springdroid( cpu, springscript );
}

fn part_two( cpu: &mut CPU ) {
    let springscript = "NOT C J\n\
    NOT B T\n\
    OR T J\n\
    AND D J\n\
    AND H J\n\
    NOT A T\n\
    OR T J\n\
    RUN\n";

    springdroid( cpu, springscript );
}

pub fn day_21( _args: Vec< String > ) {
    let opcodes = process_input( DATA );
    let mut cpu = CPU::initialise( opcodes );

    part_one( &mut cpu.clone() );
    part_two( &mut cpu );
}
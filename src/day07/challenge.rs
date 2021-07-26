
mod cpu;
use self::cpu::{
    CPU,
    Computer
};
use std::collections::HashMap;
use itertools::Itertools;

fn process_input( input: &str ) -> HashMap<usize, i64> {
    return input.split( "," )
        .enumerate()
        .map( |(i, v)| (i, v.parse().unwrap()))
        .collect();
}

fn part_one( opcodes: HashMap<usize, i64> ) {
    let mut cpu: CPU;
    let mut next_input;
    let mut output = 0;
    let num_amplifiers = 5;

    for phase_setting in (0..=4).permutations(5) {
        let mut phase = phase_setting.clone();
        next_input = 0;

        for _ in 0..num_amplifiers {
            cpu = Computer::initialise( opcodes.clone() );
            cpu.display_std_out( false );
            cpu.set_inputs( vec![next_input, phase.pop().unwrap()] );
            cpu.execute_instructions();

            next_input = cpu.get_output_value();
        }

        if next_input > output {
            output = next_input;
        }
    }

    println!( "Part One: {}", output );
}

fn part_two( opcodes: HashMap<usize, i64> ) {
    let mut amplifiers: Vec<CPU>;
    let mut output = 0;
    let mut finished: bool;
    let mut next_input: i64;


    for phase_setting in (5..=9).permutations(5) {
        amplifiers = Vec::new();
        finished = false;
        let mut phase = phase_setting.clone();
        next_input = 0;

        for _ in 0..5 {
            let mut cpu: CPU = Computer::initialise( opcodes.clone() );
            cpu.display_std_out( false );
            cpu.pause_execution_on_output( true );
            amplifiers.push( cpu );
        }

        // first run
        for amplifier in amplifiers.iter_mut() {
            let inputs = vec![next_input, phase.pop().unwrap()];
            amplifier.set_inputs( inputs );
            amplifier.execute_instructions();

            next_input = amplifier.get_output_value();
        }

        while !finished {
    
            for amplifier in amplifiers.iter_mut() {
                let inputs = vec![next_input];
                amplifier.set_inputs( inputs );
                amplifier.execute_instructions();
    

                if amplifier.has_finished() {
                    finished = true;
                    break;
                } else {
                    next_input = amplifier.get_output_value();
                }
            }
    
            if output < next_input {
                output = next_input;
            }
        }
    }

    println!( "Part Two: {}", output );
}

#[allow(dead_code)]
pub fn day_07( _args: Vec<String> ) {
    let input = "3,8,1001,8,10,8,105,1,0,0,21,30,55,76,97,114,195,276,357,438,99999,3,9,102,3,9,9,4,9,99,3,9,1002,9,3,9,1001,9,5,9,1002,9,2,9,1001,9,2,9,102,2,9,9,4,9,99,3,9,1002,9,5,9,1001,9,2,9,102,5,9,9,1001,9,4,9,4,9,99,3,9,1001,9,4,9,102,5,9,9,101,4,9,9,1002,9,4,9,4,9,99,3,9,101,2,9,9,102,4,9,9,1001,9,5,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,99";
    let opcodes = process_input( input );

    part_one( opcodes.clone() );
    part_two( opcodes.clone() );
}


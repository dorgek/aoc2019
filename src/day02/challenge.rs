use std::collections::HashMap;
use std::process;


fn process_input( input: &str ) -> HashMap<usize, u64> {
    return input.split( "," )
        .enumerate()
        .map( |(i, v)| (i, v.parse().unwrap()))
        .collect();
}

fn process_opcodes( mut opcodes: HashMap<usize, u64> ) -> u64 {
    let mut idx = 0;
    let size = opcodes.len();

    while idx < size {
        let opcode = opcodes[&idx];

                
        if opcode == 99 {
            idx = size;
        } else {
            idx += 1;
            let pos = opcodes[&idx] as usize;
            let val = opcodes[&pos];
            let output: u64;

            idx += 1;
            let pos2 = opcodes[&idx] as usize;
            let val2 = opcodes[&pos2];

            if opcode == 1 {
                output = val + val2;
            } else {
                output = val * val2;
            }

            idx += 1;
            let output_save = opcodes[&idx] as usize;
            *opcodes.get_mut( &output_save ).unwrap() = output;

        }

        idx += 1;
    }

    return opcodes[&0];
}

fn main() {
    let input = "1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,9,1,19,1,5,19,23,2,9,23,27,1,27,5,31,2,31,13,35,1,35,9,39,1,39,10,43,2,43,9,47,1,47,5,51,2,13,51,55,1,9,55,59,1,5,59,63,2,6,63,67,1,5,67,71,1,6,71,75,2,9,75,79,1,79,13,83,1,83,13,87,1,87,5,91,1,6,91,95,2,95,13,99,2,13,99,103,1,5,103,107,1,107,10,111,1,111,13,115,1,10,115,119,1,9,119,123,2,6,123,127,1,5,127,131,2,6,131,135,1,135,2,139,1,139,9,0,99,2,14,0,0";
    let opcodes = process_input( input );
    let desired_output = 19690720;

    println!( "Part One: initial final state is {}", process_opcodes( opcodes ) );

    for i in 0..99 {
        for j in 0..99 {
            let current_opcodes = process_input( format!( "1,{},{},3,1,1,2,3,1,3,4,3,1,5,0,3,2,9,1,19,1,5,19,23,2,9,23,27,1,27,5,31,2,31,13,35,1,35,9,39,1,39,10,43,2,43,9,47,1,47,5,51,2,13,51,55,1,9,55,59,1,5,59,63,2,6,63,67,1,5,67,71,1,6,71,75,2,9,75,79,1,79,13,83,1,83,13,87,1,87,5,91,1,6,91,95,2,95,13,99,2,13,99,103,1,5,103,107,1,107,10,111,1,111,13,115,1,10,115,119,1,9,119,123,2,6,123,127,1,5,127,131,2,6,131,135,1,135,2,139,1,139,9,0,99,2,14,0,0", i, j ).as_str() );

            if process_opcodes( current_opcodes.clone() ) == desired_output {
                println!( "Part Two: position 0 = {}, noun = {} and verb = {}, 100 * noun + verb = {}", process_opcodes( current_opcodes.clone() ), i, j, 100 * i + j );
                process::exit(1);
            }
        }
    }
}
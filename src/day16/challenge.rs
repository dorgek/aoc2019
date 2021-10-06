use std::collections::HashMap;

const DATA: &str = include_str!( "./testInput.txt" );

fn process_input( input: &str ) -> Vec<u8> {
    return input.chars()
        .map( |c| c.to_string().parse::<u8>().unwrap() )
        .collect();
}

fn generate_pattern( idx: usize, signal_size: usize ) -> HashMap<usize, i8> {
    let base_pattern = vec![0, 1, 0, -1];
    let new_pattern: Vec<i8> =  base_pattern.into_iter()
                            .flat_map(|v| std::iter::repeat(v).take(idx + 1))
                            .collect();

    let mut repeat_num = ( signal_size * 2 ) / new_pattern.len();

    if repeat_num == 0 {
        repeat_num = 1;
    }

    let repeat = new_pattern.repeat( repeat_num );

    let pattern: HashMap<usize, i8> = repeat.iter()
        .take( signal_size + 1 )
        .enumerate()
        .filter( |( idx, v )| **v != 0 && *idx != 0 )
        .map( |( idx, v )| ( idx - 1, *v ) )
        .collect();

    return pattern;
}

fn calculate_fft( input_val: &mut Vec<u8> ) {
    let input_len = input_val.len();
    let original_signal = input_val.clone();

    for ( idx, _input_val_digit ) in input_val.clone().iter().enumerate() {
        let pattern = generate_pattern( idx, input_len );
        let mut val: i64 = 0;

        for (k, v) in pattern {
            val += ( original_signal[k] as i8 * v ) as i64;
        }

        val = val.abs();

        if val > 9 {
            val = val % 10;
        }

        input_val[idx] = val as u8;
    }
}

fn perform_fft_phases( input_val: &mut Vec<u8>, num_phases: u64 ) {
    for _i in 0.. num_phases {
        calculate_fft( input_val );
    }
}

fn fft_phases_shortcut( input_val: &mut Vec<u8>, num_phases: u64, offset: u32 ) {
    if offset < input_val.len() as u32 / 2 {
        panic!();
    }

    for _i in 0.. num_phases {
        caclulate_short_fft( input_val, offset )
    }
}

fn caclulate_short_fft( input_val: &mut Vec<u8>, offset: u32 ) {
    let length = input_val.len() as u32;
    for idx in ( offset.. length - 1 ).rev() {
        input_val[idx as usize] = ( input_val[idx as usize] + input_val[(idx + 1) as usize] ) % 10 as u8;
    }
}

fn print_num( input_val: Vec<u8>, num_skip: u32, num_digits_print: u32 ) {
    for v in num_skip.. num_digits_print + num_skip {
        print!( "{}", input_val[v as usize] );
    }

    println!();
}

#[allow(dead_code)]
pub fn day_16( _args: Vec<String> ) {
    let mut input_data = process_input( DATA );
    let mut part_two_input_data: Vec<u8> = input_data.repeat(10_000);

    let mut num_skip = 0;
    let factor: u32 = 10;

    for i in ( 0..7 ).rev() {
        num_skip += input_data[6-i] as u32 * factor.pow( i as u32 );
    }

    // part one 
    print!( "Part one: initial 8 numbers: " );
    perform_fft_phases( &mut input_data, 100 );
    print_num( input_data.clone(), 0, 8 );

    // part two 
    print!( "Part two: " );
    fft_phases_shortcut( &mut part_two_input_data, 100, num_skip );
    print_num( part_two_input_data.clone(), num_skip, 8 );
}
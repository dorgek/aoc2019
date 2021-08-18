mod cpu;
use self::cpu::{
    CPU,
    Computer
};

use std::f64::consts::PI;

use std::collections::{
    HashMap,
    BTreeMap
};

use image;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64
}

enum Colour {
    BLACK, 
    WHITE 
}

fn process_input( input: &str ) -> HashMap<usize, i64> {
    return input.split( "," )
        .enumerate()
        .map( |(i, v)| (i, v.parse().unwrap()))
        .collect();
}

fn run_program( mut cpu: CPU, mut ship_angle: f64, initial_colour: Colour ) -> BTreeMap<Coordinate, Colour> {
    let mut robot_location: Coordinate = Coordinate { x: 0, y: 0 };
    let mut hull: BTreeMap<Coordinate, Colour> = BTreeMap::new();
    hull.insert( robot_location, initial_colour );

    loop {
        let input_val: i64;

        if let Some( colour ) = hull.get( &robot_location ) {
            input_val = match colour {
                Colour::BLACK => 0, 
                _ => 1
            }
        } else {
            input_val = 0; 
        }

        cpu.set_inputs( vec![input_val] );
        cpu.execute_instructions();

        if cpu.has_finished() {
            break;
        }

        let colour = match cpu.get_output_value() {
            1 => Colour::WHITE,
            _ => Colour::BLACK
        };

        hull.insert( robot_location, colour );

        // update ship based on second output
        cpu.execute_instructions();

        if cpu.get_output_value() == 0 { 
            ship_angle += PI / 2.0;
        } else {
            ship_angle -= PI / 2.0;
        }

        robot_location.x = robot_location.x + ship_angle.cos() as i64;
        robot_location.y = robot_location.y - ship_angle.sin() as i64;
    }

    return hull;
}

fn to_image( tree: &BTreeMap<Coordinate, Colour>, name: &str ) {
    let mut top_left = Coordinate { x: 0, y: 0 };
    let mut bottom_right = Coordinate { x: 0, y: 0 };

    for coord in tree.keys() {
        top_left.x = i64::min( top_left.x, coord.x );
        top_left.y = i64::min( top_left.y, coord.y );

        bottom_right.x = i64::max( bottom_right.x, coord.x );
        bottom_right.y = i64::max( bottom_right.y, coord.y );
    }

    let width = ( bottom_right.x - top_left.x + 1 ) as u32;
    let height = ( bottom_right.y - top_left.y + 1 ) as u32;

    let mut img_buff = image::ImageBuffer::new( width, height );

    let black = image::Rgb( [0, 0, 0] );
    let white = image::Rgb( [255, 255, 255] );

    for (coord, color) in tree {
        let pixel = match color {
            Colour::BLACK => black,
            Colour::WHITE => white,
        };

        let x = ( coord.x - top_left.x ) as u32;
        let y = ( coord.y - top_left.y ) as u32;

        img_buff.put_pixel( x, y, pixel );
    }

    img_buff.save( name ).unwrap();
}

fn part_one( mut cpu: CPU ) {
    let ship_angle = PI / 2.0;
    cpu.display_std_out( false );
    cpu.pause_execution_on_output( true );

    let hull = run_program( cpu, ship_angle, Colour::BLACK );

    println!( "Part one: number of painted squares {}", hull.len() );
}

fn part_two( mut cpu: CPU ) {
    let ship_angle = PI / 2.0;
    cpu.display_std_out( false );
    cpu.pause_execution_on_output( true );

    let hull = run_program( cpu, ship_angle, Colour::WHITE );

    to_image( &hull, "second_image.png" );
}


pub fn day_11( _args: Vec<String> ) {
    let input = "3,8,1005,8,327,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,28,1006,0,42,2,1104,11,10,1006,0,61,2,1005,19,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,102,1,8,65,1006,0,4,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,89,1,1108,10,10,1,1103,11,10,1,109,18,10,1006,0,82,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,102,1,8,126,2,109,7,10,1,104,3,10,1006,0,64,2,1109,20,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,101,0,8,163,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,185,2,1109,12,10,2,103,16,10,1,107,11,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,219,1,1005,19,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,102,1,8,245,2,1002,8,10,1,2,9,10,1006,0,27,1006,0,37,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,102,1,8,281,1006,0,21,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,1001,8,0,306,101,1,9,9,1007,9,1075,10,1005,10,15,99,109,649,104,0,104,1,21102,1,847069852568,1,21101,344,0,0,1105,1,448,21101,0,386979963688,1,21101,355,0,0,1105,1,448,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,46346031251,1,1,21101,0,402,0,1105,1,448,21102,1,29195594775,1,21101,0,413,0,1105,1,448,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,868498428772,1,21101,0,436,0,1106,0,448,21102,718170641172,1,1,21102,1,447,0,1105,1,448,99,109,2,21202,-1,1,1,21102,40,1,2,21102,1,479,3,21102,1,469,0,1105,1,512,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,474,475,490,4,0,1001,474,1,474,108,4,474,10,1006,10,506,1101,0,0,474,109,-2,2106,0,0,0,109,4,2102,1,-1,511,1207,-3,0,10,1006,10,529,21101,0,0,-3,22101,0,-3,1,22101,0,-2,2,21101,0,1,3,21101,548,0,0,1106,0,553,109,-4,2106,0,0,109,5,1207,-3,1,10,1006,10,576,2207,-4,-2,10,1006,10,576,21202,-4,1,-4,1106,0,644,22101,0,-4,1,21201,-3,-1,2,21202,-2,2,3,21102,1,595,0,1105,1,553,21201,1,0,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,614,21102,1,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,636,22102,1,-1,1,21102,1,636,0,106,0,511,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0";
    let opcodes = process_input( input );
    let cpu: CPU = Computer::initialise( opcodes.clone() );

    part_one( cpu );
    part_two( Computer::initialise( opcodes.clone() ) );
}
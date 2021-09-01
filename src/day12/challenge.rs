use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    cmp::Ordering
};

use regex::Regex;

use num::Integer;
use num::FromPrimitive;
use num::bigint::BigInt;

#[derive(Copy, Clone)]
struct Body {
    xp: i64, 
    yp: i64, 
    zp: i64, 
    xv: i64, 
    yv: i64, 
    zv: i64
}

#[derive(Clone)]
struct Space {
    moons: Vec<Body>,
    initial_state:  Vec<Body>
}

#[derive(Copy, Clone)]
enum Lookup {
    XP,
    XV,
    YP,
    YV,
    ZP,
    ZV
}

trait Simulation {
    fn initialise( filename: impl AsRef<Path> ) -> Self;
    fn step( &mut self );

    fn get_to_initial_state( &mut self ) -> BigInt;
    fn check_state( &mut self, position: Lookup, velocity: Lookup ) -> bool;
    fn get_struct_field( &mut self, moon: Body, field: Lookup ) -> i64;

    fn update_speeds( &mut self );
    fn update_speed( &mut self, moon_one_idx: usize, moon_two_idx: usize );
    fn update_positions( &mut self );

    fn total_energy( &mut self ) -> i64;

    fn print_moons( &mut self );
}

impl Simulation for Space {
    fn initialise( filename: impl AsRef<Path> ) -> Self {
        let file = File::open( filename ).expect( "no such file" );
        let buf  = BufReader::new( file );
        let re = Regex::new( "[0-9-]+" ).unwrap();
    
        let bodies: Vec<Body> = buf.lines()
            .map( |l| create_moon( l.expect( "could not parse line" ), re.clone() ) )
            .collect();

        return Space{ moons: bodies.clone(), initial_state: bodies };
    }

    fn step( &mut self ) {
        self.update_speeds();
        self.update_positions();
    }

    fn get_to_initial_state( &mut self ) -> BigInt {
        let mut x_iter = 0;
        let mut y_iter = 0;
        let mut z_iter = 0;
        let mut x_lcm = 0;
        let mut y_lcm = 0;
        let mut z_lcm = 0;

        while x_lcm == 0 || y_lcm == 0 || z_lcm == 0 {
            self.update_speeds();
            self.update_positions();
            
            // check if x axis is back to initial state
            if !self.check_state( Lookup::XP, Lookup::XV ) {
                x_iter += 1;
            } else if x_lcm == 0 {
                x_lcm = x_iter + 1;
            }

            if !self.check_state( Lookup::YP, Lookup::YV ) {
                y_iter += 1;
            } else if y_lcm == 0 {
                y_lcm = y_iter + 1;
            }

            if !self.check_state( Lookup::ZP, Lookup::ZV ) {
                z_iter += 1;
            } else if z_lcm == 0 {
                z_lcm = z_iter + 1;
            }
        }
        
        let x_num: BigInt = FromPrimitive::from_u64( x_lcm ).unwrap();
        let y_num: BigInt = FromPrimitive::from_u64( y_lcm ).unwrap();
        let z_num: BigInt = FromPrimitive::from_u64( z_lcm ).unwrap();
        let temp = x_num.lcm( &y_num );

        return temp.lcm( &z_num );
    }

    fn check_state( &mut self, position: Lookup, velocity: Lookup ) -> bool {
        let mut ret = true;

        for idx in 0.. self.moons.len() {
            if self.get_struct_field( self.moons[idx], position ) != self.get_struct_field( self.initial_state[idx], position ) ||
                self.get_struct_field( self.moons[idx], velocity ) != 0 {
                    ret = false;
                    break;
            }
        }

        return ret;
    }

    fn get_struct_field( &mut self, moon: Body, field: Lookup ) -> i64 {
        match field {
            Lookup::XP => moon.xp, 
            Lookup::XV => moon.xv,
            Lookup::YP => moon.yp, 
            Lookup::YV => moon.yv,
            Lookup::ZP => moon.zp, 
            Lookup::ZV => moon.zv
        }
    }

    fn update_speeds( &mut self ) {
        for ( idx, _moon ) in self.moons.clone().iter().enumerate() {
            for i in idx + 1 .. self.moons.clone().len() {
                self.update_speed( idx, i );
            }
        }
    }

    fn update_speed( &mut self, moon_one_idx: usize, moon_two_idx: usize ) {
        let mut moon_one = self.moons[moon_one_idx]; // TODO: get two mut objects?
        let mut moon_two = self.moons[moon_two_idx];

        moon_one.xv += check_values( moon_one.xp, moon_two.xp );
        moon_two.xv += check_values( moon_two.xp, moon_one.xp );
        moon_one.yv += check_values( moon_one.yp, moon_two.yp );
        moon_two.yv += check_values( moon_two.yp, moon_one.yp );
        moon_one.zv += check_values( moon_one.zp, moon_two.zp );
        moon_two.zv += check_values( moon_two.zp, moon_one.zp );

        self.moons[moon_one_idx] = moon_one;
        self.moons[moon_two_idx] = moon_two;
    }

    fn update_positions( &mut self ) {
        for ( _idx, moon ) in self.moons.iter_mut().enumerate() {
            moon.xp += moon.xv;
            moon.yp += moon.yv;
            moon.zp += moon.zv;
        }
    }

    fn total_energy( &mut self ) -> i64 {
        let mut energy = 0;

        for moon in self.moons.clone() {
            energy += ( moon.xp.abs() + moon.yp.abs() + moon.zp.abs() ) * ( moon.xv.abs() + moon.yv.abs() + moon.zv.abs() );
        }

        return energy;
    }

    fn print_moons( &mut self ) {
        for moon in self.moons.clone() {
            println!( "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}, y={:3}, z={:3}>", moon.xp, moon.yp, moon.zp, moon.xv, moon.yv, moon.zv );
        }

        println!()
    }
}

fn check_values( a: i64, b: i64 ) -> i64 {
    match a.cmp( &b ) {
        Ordering::Less => 1, 
        Ordering::Greater => -1,
        _ => 0
    }
}

fn create_moon( line: String, re: Regex ) -> Body {
    let mut cap = re.captures_iter( &line );

    let x = &cap.next().unwrap()[0].parse::<i64>().unwrap();
    let y = &cap.next().unwrap()[0].parse::<i64>().unwrap();
    let z = &cap.next().unwrap()[0].parse::<i64>().unwrap();

    return Body { xp: *x, yp: *y, zp: *z, xv: 0, yv: 0, zv: 0 };
}

fn part_one( mut sim: Space ) {
    for _i in 0.. 1000 {
        sim.step();
    }

    println!( "Part one: total energy: {}", sim.total_energy() );
}

fn part_two( mut sim: Space ) {
    println!( "Initial state: " );
    sim.print_moons();

    let num_steps = sim.get_to_initial_state();

    println!( "Current state: " );
    sim.print_moons();

    println!( "Part Two: num steps to initial state {}", num_steps );
}

#[allow(dead_code)]
pub fn day_12( args: Vec<String> ) {
    let sim: Space = Simulation::initialise( args[1].clone() );

    part_one( sim.clone() );
    part_two( sim.clone() );
}
mod cpu;
use self::cpu::{
    CPU,
    Computer,
    Consumer,
    Producer
};

use std::io::prelude::*;        
use std::io;

use std::collections::LinkedList;
use std::collections::HashMap;

const DATA: &str = include_str!( "./puzzleInput.txt" );

struct StandardConsumer {
    cache: LinkedList<i64>
}

trait SingleConsumer: Consumer {
    fn initialise() -> Self;
}

impl Consumer for StandardConsumer {
    fn get_value( &mut self ) -> i64 {
        if let Some( v ) = self.cache.pop_front() {
            return v;
        }

        let mut buff = String::new();
        io::stdout().flush().ok().expect( "could not flush" );
        io::stdin().read_line( &mut buff ).expect( "failed to read line" );

        buff.chars()
            .for_each( |c| { 
                self.cache.push_back( c as u8 as i64 );
            });

        return self.cache.pop_front().unwrap();
    }
}

impl SingleConsumer for StandardConsumer {
    fn initialise() -> Self {
        return StandardConsumer { cache: LinkedList::new() };
    }
}

struct StandardProducer {}

trait SingleProducer: Producer {
    fn initialise() -> Self;
}

impl Producer for StandardProducer {
    fn load_value( &mut self, value: i64 ) {
        print!( "{}", value as u8 as char );
    }
}

impl SingleProducer for StandardProducer {
    fn initialise() -> Self {
        return StandardProducer{};
    }
}

fn process_input( input: &str ) -> HashMap<usize, i64> {
    return input.split( "," )
        .enumerate()
        .map( |(i, v)| (i, v.parse().unwrap()))
        .collect();
}

pub fn day_25( _: Vec< String > ) {
    let opcodes = process_input( DATA );
    let consumer = StandardConsumer::initialise();
    let producer = StandardProducer::initialise();
    let mut cpu = CPU::initialise( opcodes, consumer, producer );

    cpu.display_std_out( false );
    cpu.pause_execution_on_output( false );

    cpu.execute_instructions();
}
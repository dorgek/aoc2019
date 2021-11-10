use std::collections::HashMap;
use std::io::prelude::*;        
use std::io;

pub type Instruction<T: Consumer, U: Producer> = fn( &mut CPU<T, U>, Option<i64>, Option<i64>, Option<i64> );

pub trait Computer< T: Consumer, U: Producer > {
    fn initialise( memory: HashMap< usize, i64 >, _: T, _: U ) -> Self;
    fn add( &mut self, _: Option<i64>, _: Option<i64>, _: Option<i64> ); 
    fn multiply( &mut self, _: Option<i64>, _: Option<i64>, _: Option<i64> ); 
    fn exit( &mut self, _: Option<i64>, _: Option<i64>, _: Option<i64> ); 
    fn read( &mut self, _: Option<i64>, _: Option<i64>, _: Option<i64> ); 
    fn write( &mut self, _: Option<i64>, _: Option<i64>, _: Option<i64> ); 
    fn jump_if_true( &mut self, _: Option<i64>, _: Option<i64>, _: Option<i64> ); 
    fn jump_if_false( &mut self, _: Option<i64>, _: Option<i64>, _: Option<i64> ); 
    fn less_than( &mut self, _: Option<i64>, _: Option<i64>, _: Option<i64> ); 
    fn equals( &mut self, _: Option<i64>, _: Option<i64>, _: Option<i64> ); 
    fn add_relative_offset( &mut self, _: Option<i64>, _: Option<i64>, _: Option<i64> ); 
    fn execute_instruction( &mut self );
    fn execute_instructions( &mut self );
    fn print_initial_value( &mut self );
    fn get_output_value( &mut self ) -> i64;
    fn set_inputs( &mut self, inputs: Vec<i64> );
    fn set_inputs_consumer( &mut self, _: T );
    fn set_outputs_producer( &mut self, _: U );
    fn display_std_out( &mut self, disp_std_out: bool );
    fn pause_execution_on_output( &mut self, pause_execution: bool );
    fn has_finished( &self ) -> bool;
    fn has_value( &mut self ) -> bool;
    fn get_previous_instruction( &self ) -> usize;
    fn run_until_num_out( &mut self, _: usize );

    fn private_read_parameter( &mut self, parameter_mode: Option<i64> ) -> i64;
    fn private_write_parameter( &mut self, parameter_mode: Option<i64>, output: i64 );
    fn read_digit( &mut self, digit: i64, place: u32 ) -> Option< i64 >;
    fn private_check_memory( &mut self, idx: usize ) -> i64;
    fn private_get_index( &mut self, param_mode: Option<i64> );
}

pub trait Consumer {
    fn get_value( &mut self ) -> i64;
}


struct StandardConsumer {}

impl Consumer for StandardConsumer {
    fn get_value( &mut self ) -> i64 {
        let mut buff = String::new();

        print!( "Enter a value: " );
        io::stdout().flush().ok().expect( "could not flush" );
        io::stdin().read_line( &mut buff ).expect( "failed to read line" );
        return buff.trim().parse().unwrap();
    }
}

pub trait Producer {
    fn load_value( &mut self, _: i64 );
}

struct StandardProducer {}

impl Producer for StandardProducer {
    fn load_value( &mut self, value: i64 ) {
        print!( "{}", value );
    }
}

pub struct CPU<T: Consumer, U: Producer> {
    dispatcher: HashMap<usize, Instruction<T, U>>,
    memory: HashMap<usize, i64>,
    idx: usize,
    out: Option<i64>,
    inputs: Vec<i64>,
    disp_std_out: bool,
    pause_execution: bool,
    finished: bool,
    relative_offset: i64,
    input_consumer: T,
    output_producer: U,
    previous_instruction: usize,
    num_output: usize
}

impl < T: Consumer, U: Producer > Computer<T, U> for CPU< T, U > {
    fn initialise( memory: HashMap< usize, i64 >, consumer: T, producer: U ) -> CPU<T, U> {
        let mut dispatcher = HashMap::new();
        dispatcher.insert( 1, Self::add as Instruction<T, U> );
        dispatcher.insert( 2, Self::multiply as Instruction<T, U> );
        dispatcher.insert( 3, Self::read as Instruction<T, U> );
        dispatcher.insert( 4, Self::write as Instruction<T, U> );
        dispatcher.insert( 5, Self::jump_if_true as Instruction<T, U> );
        dispatcher.insert( 6, Self::jump_if_false as Instruction<T, U> );
        dispatcher.insert( 7, Self::less_than as Instruction<T, U> );
        dispatcher.insert( 8, Self::equals as Instruction<T, U> );
        dispatcher.insert( 9, Self::add_relative_offset as Instruction<T, U> );
        dispatcher.insert( 99, Self::exit as Instruction<T, U> );

        CPU { dispatcher: dispatcher, memory: memory, idx: 0, out: None, inputs: Vec::new(), disp_std_out: true, pause_execution: false, finished: false, relative_offset: 0,
            input_consumer: consumer,
            output_producer: producer,
            previous_instruction: 0,
            num_output: 0
         }
    }

    fn execute_instructions( &mut self ) {
        while !self.finished && !( self.pause_execution && !self.out.is_none() ) {
            self.execute_instruction();
        }
    }

    fn read_digit( &mut self, digit: i64, place: u32 ) -> Option< i64 > {
        return Some( digit / ( i64::pow( 10, place - 1 ) ) % 10 );
    }

    fn execute_instruction( &mut self ) {
        let next_code = self.memory[&self.idx] as usize;
        let param_mode_one: Option<i64> = self.read_digit( next_code as i64, 3 );
        let param_mode_two: Option<i64> = self.read_digit( next_code as i64, 4 );
        let param_mode_three: Option<i64> = self.read_digit( next_code as i64, 5 );

        let instruction = next_code % 100;
        self.idx += 1;

        self.previous_instruction = instruction;

        self.dispatcher[&(instruction)]( self, param_mode_one, param_mode_two, param_mode_three );
    }

    fn run_until_num_out( &mut self, idx: usize ) {
        while self.num_output < idx {
            self.execute_instruction();
        }

        self.num_output = 0;
    }

    fn get_previous_instruction( &self ) -> usize {
        return self.previous_instruction;
    }

    // TODO: workout how to have this private
    fn add( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64>, param_three_mode: Option<i64> ) {
        let first_val  : i64 = self.private_read_parameter( param_one_mode );
        let second_val : i64 = self.private_read_parameter( param_two_mode );

        let output = first_val + second_val;

        // write into memory
        self.private_write_parameter( param_three_mode, output );
    }

    fn multiply( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64>, param_three_mode: Option<i64> ) {
        let first_val  : i64 = self.private_read_parameter( param_one_mode );
        let second_val : i64 = self.private_read_parameter( param_two_mode );

        let output = first_val * second_val;

        // write into memory
        self.private_write_parameter( param_three_mode, output );
    }

    fn read( &mut self, param_one_mode: Option<i64>, _: Option<i64>, _: Option<i64> ) {
        let input: i64 = Consumer::get_value( &mut self.input_consumer );
        self.private_write_parameter( param_one_mode, input );
    }

    fn write( &mut self, param_one_mode: Option<i64>, _: Option<i64>, _: Option<i64> ) {
        let value = self.private_read_parameter( param_one_mode );
        self.output_producer.load_value( value );
        self.out = Some( value );
        self.num_output += 1;
    }

    fn jump_if_true( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64>, _: Option<i64> ) {
        let first_val : i64 = self.private_read_parameter( param_one_mode );

        // write to memory
        if first_val != 0 {
            self.private_get_index( param_two_mode );
        } else {
            self.idx += 1;
        }
    }

    fn jump_if_false( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64>, _: Option<i64>) {
        let first_val  : i64 = self.private_read_parameter( param_one_mode );

        // write to memory
        if first_val == 0 {
            self.private_get_index( param_two_mode );
        } else {
            self.idx += 1;
        }
    }

    fn less_than( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64>, param_three_mode: Option<i64> ) {
        let first_val  : i64 = self.private_read_parameter( param_one_mode );
        let second_val : i64 = self.private_read_parameter( param_two_mode );

        if first_val < second_val {
            self.private_write_parameter( param_three_mode, 1 );
        } else {
            self.private_write_parameter( param_three_mode, 0 );
        }
    }

    fn equals( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64>, param_three_mode: Option<i64> ) {
        let first_val  : i64 = self.private_read_parameter( param_one_mode );
        let second_val : i64 = self.private_read_parameter( param_two_mode );

        if first_val == second_val {
            self.private_write_parameter( param_three_mode, 1 );
        } else {
            self.private_write_parameter( param_three_mode, 0 );
        }
    }

    fn add_relative_offset( &mut self, param_one_mode: Option<i64>, _: Option<i64>, _: Option<i64> ) {
        let offset = self.private_read_parameter( param_one_mode );
        self.relative_offset += offset;
    }

    fn exit( &mut self, _: Option<i64>, _: Option<i64>, _: Option<i64> ) {
        self.finished = true;
    }

    fn print_initial_value( &mut self ) {
        println!( "{}", self.memory[&0] );
    }

    fn get_output_value( &mut self ) -> i64 {
        let ret = self.out.unwrap();
        self.out = None;
        return ret;
    }

    fn has_value( &mut self ) -> bool {
        return self.out.is_some();
    }

    fn set_inputs( &mut self, inputs: Vec<i64> ) {
        self.inputs = inputs;
    }

    fn set_inputs_consumer( &mut self, consumer: T ) {
        self.input_consumer = consumer;
    }

    fn set_outputs_producer( &mut self, producer: U ) {
        self.output_producer = producer;
    }

    fn display_std_out(&mut self, disp_std_out: bool) {
        self.disp_std_out = disp_std_out;
    }

    fn private_read_parameter( &mut self, parameter_mode: Option<i64> ) -> i64 {
        let idx: usize;

        match parameter_mode {
            Some( 1 ) => idx = self.idx,
            Some( 2 ) => idx = (self.private_check_memory(self.idx) + self.relative_offset) as usize,
            _ => idx = self.private_check_memory(self.idx) as usize
        }

        self.idx += 1;

        return self.private_check_memory( idx );
    }

    fn private_get_index( &mut self, param_mode: Option<i64> ) {
        match param_mode {
            Some( 1 ) => {
                self.idx = self.memory[&(self.idx)] as usize;
            },
            Some( 2 ) => {
                let sec_val_pos = (self.memory[&(self.idx)] + self.relative_offset) as usize;
                self.idx = self.memory[&sec_val_pos] as usize;
            }
            _ => {
                let sec_val_pos = self.memory[&(self.idx)] as usize;
                self.idx = self.memory[&sec_val_pos] as usize;
            }
        }
    }

    fn private_check_memory( &mut self, idx: usize ) -> i64 {
        let ret: i64;

        if !self.memory.contains_key( &idx ) {
            ret = 0;
            self.memory.insert( idx, 0 );
        } else {
            ret = self.memory[&idx];
        }

        return ret;
    }

    fn private_write_parameter( &mut self, parameter_mode: Option<i64>, output: i64 ) {
        let output_save: i64;
        
        match parameter_mode {
            Some( 2 ) => output_save = self.private_check_memory(self.idx) + self.relative_offset,
            _ => output_save = self.private_check_memory(self.idx)
        }

        self.memory.insert( output_save as usize, output );
        self.idx += 1;
    }

    fn pause_execution_on_output( &mut self, pause_execution: bool ) {
        self.pause_execution = pause_execution;
    }

    fn has_finished( &self ) -> bool {
        return self.finished;
    }
}
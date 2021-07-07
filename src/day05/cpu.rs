use std::collections::HashMap;
use std::io::prelude::*;        
use std::io;

pub type Instruction = fn( &mut CPU, Option<i64>, Option<i64> );

pub trait Computer {
    fn initialise( memory: HashMap< usize, i64 > ) -> Self;
    fn add( &mut self, _: Option<i64>, _: Option<i64> ); 
    fn multiply( &mut self, _: Option<i64>, _: Option<i64> );
    fn exit( &mut self, _: Option<i64>, _: Option<i64> );
    fn read( &mut self, _: Option<i64>, _: Option<i64> );
    fn write( &mut self, _: Option<i64>, _: Option<i64> );
    fn jump_if_true( &mut self, _: Option<i64>, _: Option<i64> );
    fn jump_if_false( &mut self, _: Option<i64>, _: Option<i64> );
    fn less_than( &mut self, _: Option<i64>, _: Option<i64> );
    fn equals( &mut self, _: Option<i64>, _: Option<i64> );
    fn execute_instruction( &mut self );
    fn execute_instructions( &mut self );
    fn print_initial_value( &mut self );
}

pub struct CPU {
    dispatcher: HashMap<usize, Instruction>,
    memory: HashMap<usize, i64>,
    idx: usize
}

impl Computer for CPU {
    fn initialise( memory: HashMap< usize, i64 > ) -> CPU {
        let mut dispatcher = HashMap::new();
        dispatcher.insert( 1, Self::add as Instruction );
        dispatcher.insert( 2, Self::multiply as Instruction );
        dispatcher.insert( 3, Self::read as Instruction );
        dispatcher.insert( 4, Self::write as Instruction );
        dispatcher.insert( 5, Self::jump_if_true as Instruction );
        dispatcher.insert( 6, Self::jump_if_false as Instruction );
        dispatcher.insert( 7, Self::less_than as Instruction );
        dispatcher.insert( 8, Self::equals as Instruction );
        dispatcher.insert( 99, Self::exit as Instruction );

        CPU { dispatcher: dispatcher, memory: memory, idx: 0 }
    }

    fn execute_instructions( &mut self ) {
        let size = self.memory.len();

        while self.idx < size {
            self.execute_instruction();
        }
    }

    fn execute_instruction( &mut self ) {
        let next_code = self.memory[&self.idx] as usize;
        let param_mode_one: Option<i64>;
        let param_mode_two: Option<i64>;

        let mut val = ( next_code / 100 ) % 10;

        match val {
            1 => {
                param_mode_one = Some( 1 );
            },
            _ => {
                param_mode_one = None;
            }
        }

        val = ( next_code / 1000 ) % 10;

        match val {
            1 => {
                param_mode_two = Some( 1 );
            },
            _ => {
                param_mode_two = None;
            }
        }

        let instruction = ( ( next_code / 10 ) % 10 ) * 10 + ( next_code % 10 );

        self.dispatcher[&(instruction)]( self, param_mode_one, param_mode_two );
    }

    // TODO: workout how to have this private
    fn add( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64> ) {
        let first_val  : i64;
        let second_val : i64;

        match param_one_mode {
            Some( 1 ) => {
                first_val = self.memory[&(self.idx + 1)];
            },
            _ => {
                let first_val_pos = self.memory[&(self.idx + 1)] as usize;
                first_val = self.memory[&first_val_pos];
            }
        }

        match param_two_mode {
            Some( 1 ) => {
                second_val = self.memory[&(self.idx + 2)];
            },
            _ => {
                let sec_val_pos   = self.memory[&(self.idx + 2)] as usize;
                second_val = self.memory[&sec_val_pos];
            }
        }


        let output = first_val + second_val;

        // write into memory
        let output_save = self.memory[&(self.idx + 3)] as usize;
        *self.memory.get_mut( &output_save ).unwrap() = output;

        self.idx += 4;
    }

    fn multiply( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64> ) {
        let first_val  : i64;
        let second_val : i64;

        match param_one_mode {
            Some( 1 ) => {
                first_val = self.memory[&(self.idx + 1)];
            },
            _ => {
                let first_val_pos = self.memory[&(self.idx + 1)] as usize;
                first_val = self.memory[&first_val_pos];
            }
        }

        match param_two_mode {
            Some( 1 ) => {
                second_val = self.memory[&(self.idx + 2)];
            },
            _ => {
                let sec_val_pos   = self.memory[&(self.idx + 2)] as usize;
                second_val = self.memory[&sec_val_pos];
            }
        }

        let output = first_val * second_val;

        // write into memory
        let output_save = self.memory[&(self.idx + 3)] as usize;
        *self.memory.get_mut( &output_save ).unwrap() = output;

        self.idx += 4;
    }

    fn read( &mut self, _: Option<i64>, _: Option<i64> ) {
        let save_location = self.memory[&(self.idx + 1)] as usize;
        let mut buff = String::new();

        print!( "Enter a value: " );
        io::stdout().flush().ok().expect( "could not flush" );
        io::stdin().read_line( &mut buff ).expect( "failed to read line" );

        *self.memory.get_mut( &save_location ).unwrap() = buff.trim().parse().unwrap();
        self.idx += 2;
    }

    fn write( &mut self, param_one_mode: Option<i64>, _: Option<i64> ) {
        let value: i64;

        match param_one_mode {
            Some( 1 ) => {
                value = self.memory[&( self.idx + 1 )];
            },
            _ => {
                let read   = self.memory[&(self.idx + 1)] as usize;
                value = self.memory[&read];
            }
        }

        println!( "{}", value );

        self.idx += 2;
    }

    fn jump_if_true( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64>) {
        let first_val  : i64;

        match param_one_mode {
            Some( 1 ) => {
                first_val = self.memory[&(self.idx + 1)];
            },
            _ => {
                let first_val_pos = self.memory[&(self.idx + 1)] as usize;
                first_val = self.memory[&first_val_pos];
            }
        }

        // write to memory
        if first_val != 0 {
            match param_two_mode {
                Some( 1 ) => {
                    self.idx = self.memory[&(self.idx + 2)] as usize;
                },
                _ => {
                    let sec_val_pos   = self.memory[&(self.idx + 2)] as usize;
                    self.idx = self.memory[&sec_val_pos] as usize;
                }
            }

        } else {
            self.idx += 3;
        }
    }

    fn jump_if_false( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64>) {
        let first_val  : i64;

        match param_one_mode {
            Some( 1 ) => {
                first_val = self.memory[&(self.idx + 1)];
            },
            _ => {
                let first_val_pos = self.memory[&(self.idx + 1)] as usize;
                first_val = self.memory[&first_val_pos];
            }
        }

        // write to memory
        if first_val == 0 {
            match param_two_mode {
                Some( 1 ) => {
                    self.idx = self.memory[&(self.idx + 2)] as usize;
                },
                _ => {
                    let sec_val_pos = self.memory[&(self.idx + 2)] as usize;
                    self.idx = self.memory[&sec_val_pos] as usize;
                }
            }

        } else {
            self.idx += 3;
        }
    }

    fn less_than( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64> ) {
        let first_val  : i64;
        let second_val : i64;

        match param_one_mode {
            Some( 1 ) => {
                first_val = self.memory[&(self.idx + 1)];
            },
            _ => {
                let first_val_pos = self.memory[&(self.idx + 1)] as usize;
                first_val = self.memory[&first_val_pos];
            }
        }

        match param_two_mode {
            Some( 1 ) => {
                second_val = self.memory[&(self.idx + 2)];
            },
            _ => {
                let sec_val_pos   = self.memory[&(self.idx + 2)] as usize;
                second_val = self.memory[&sec_val_pos];
            }
        }

        let output_save = self.memory[&(self.idx + 3)] as usize;

        if first_val < second_val {
            *self.memory.get_mut( &output_save ).unwrap() = 1;
        } else {
            *self.memory.get_mut( &output_save ).unwrap() = 0;
        }

        self.idx += 4;
    }

    fn equals( &mut self, param_one_mode: Option<i64>, param_two_mode: Option<i64> ) {
        let first_val  : i64;
        let second_val : i64;

        match param_one_mode {
            Some( 1 ) => {
                first_val = self.memory[&(self.idx + 1)];
            },
            _ => {
                let first_val_pos = self.memory[&(self.idx + 1)] as usize;
                first_val = self.memory[&first_val_pos];
            }
        }

        match param_two_mode {
            Some( 1 ) => {
                second_val = self.memory[&(self.idx + 2)];
            },
            _ => {
                let sec_val_pos   = self.memory[&(self.idx + 2)] as usize;
                second_val = self.memory[&sec_val_pos];
            }
        }

        let output_save = self.memory[&(self.idx + 3)] as usize;

        if first_val == second_val {
            *self.memory.get_mut( &output_save ).unwrap() = 1;
        } else {
            *self.memory.get_mut( &output_save ).unwrap() = 0;
        }

        self.idx += 4;
    }

    fn exit( &mut self, _: Option<i64>, _: Option<i64> ) {
        self.idx = self.memory.len();
    }

    fn print_initial_value( &mut self ) {
        println!( "{}", self.memory[&0] );
    }
}

mod cpu;
use self::cpu::{
    CPU,
    Computer,
    Producer,
    Consumer
};

use std::collections::LinkedList;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::sync_channel;

// block on input and execute the other instructions

const DATA: &str = include_str!( "./puzzleInput.txt" );
const NUMBER_OF_COMPUTERS: u64 = 50;

trait SingleConsumer : Consumer {
    fn initialise( _: Receiver<i64>, _: SyncSender<i64>, _: i64 ) -> Self;
}

trait SingleProducer: Producer {
    fn initialise( _: Vec<SyncSender<i64>>, _: SyncSender<i64> ) -> Self;
}

struct ConnectConsumer {
    receiver: Receiver<i64>,
    nat_sender: SyncSender<i64>,
    idx: i64
}

impl SingleConsumer for ConnectConsumer {
    fn initialise( receiver: Receiver<i64>, nat_sender: SyncSender<i64>, idx: i64 ) -> Self {
        return ConnectConsumer { 
            receiver: receiver,
            nat_sender: nat_sender,
            idx: idx
        }
    }
}

impl Consumer for ConnectConsumer {
    fn get_value( &mut self ) -> i64 {
        let received_value = self.receiver.try_recv().unwrap_or( -1 );

        if received_value == -1 {
            self.nat_sender.send( -1 ).ok();
        }

        return received_value;
    }
}

struct ConnectProducer {
    senders: Vec<SyncSender<i64>>,
    nat_sender: SyncSender<i64>,
    cpu_send: usize,
    idx: usize,
    cache: LinkedList<i64>
}

impl SingleProducer for ConnectProducer {
    fn initialise( senders: Vec<SyncSender<i64>>, nat_sender: SyncSender<i64> ) -> Self {
        return ConnectProducer { 
            senders: senders,
            nat_sender: nat_sender,
            cpu_send: 0,
            idx: 0,
            cache: LinkedList::new()
        };
    }
}

impl Producer for ConnectProducer {
    fn load_value( &mut self, value: i64 ) {
        if self.idx == 0 {
            self.cpu_send = value as usize;
        } else {
            self.cache.push_back( value );
        }

        self.idx += 1;

        if self.cpu_send == 255 && self.cache.len() == 2 {
            let x = self.cache.pop_front().unwrap();
            let y = self.cache.pop_front().unwrap();

            self.nat_sender.send( self.cpu_send as i64 ).ok();
            self.nat_sender.send( x ).ok();
            self.nat_sender.send( y ).ok();
        } else if self.cache.len() == 2 {
            self.idx = 0;

            self.nat_sender.send( self.cpu_send as i64 ).ok();

            while let Some( send_value ) = self.cache.pop_front() {
                self.senders[self.cpu_send].send( send_value ).ok();
                self.nat_sender.send( send_value ).ok();
            }
        }
    }
}

struct Network {
    network_interface_controllers: Vec<CPU<ConnectConsumer, ConnectProducer>>
}

trait NetworkControl {
    fn execute_loop( &mut self, nat: &mut NAT ) -> bool;
}

impl NetworkControl for Network {

    fn execute_loop( &mut self, nat: &mut NAT ) -> bool {
        for cpu in self.network_interface_controllers.iter_mut() {
            cpu.execute_instruction();

            if cpu.get_previous_instruction() == 4 {
                cpu.run_until_num_out(3);
            }
        }

        nat.check_nat();

        for cpu in self.network_interface_controllers.iter_mut() {
            if !cpu.has_finished() {
                return false;
            }
        }
    
        return true;
    }
}

struct NAT {
    receiver: Receiver<i64>,
    x: i64,
    y: i64,
    idle_count: usize,
    sender: SyncSender<i64>
}

trait NATImpl {
    fn initialise( receiver: Receiver<i64>, sender: SyncSender<i64> ) -> Self;
    fn check_nat( &mut self );
}

impl NATImpl for NAT {
    fn initialise( receiver: Receiver<i64>, sender: SyncSender<i64> ) -> Self {
        return NAT {
            receiver: receiver,
            sender: sender,
            x: 0,
            y: 0,
            idle_count: 0
        };
    }

    fn check_nat( &mut self ) {
        while let Some( recv_value ) = self.receiver.try_recv().ok() {
            if recv_value == -1 {
                self.idle_count += 1;
            } else if recv_value == 255 {
                let x = self.receiver.try_recv().unwrap();
                let y = self.receiver.try_recv().unwrap();

                self.x = x;
                self.y = y;

                println!( "NAT received value: x = {}, y = {}", x, y );
            } else {
                self.idle_count = 0;

                // clear out packet
                let x = self.receiver.try_recv().unwrap();
                let y = self.receiver.try_recv().unwrap();
            }
        }

        if self.idle_count > 100_000 {
            println!( "Sending packets to address 0: x = {}, y = {}", self.x, self.y );
            self.sender.send( self.x ).ok();
            self.sender.send( self.y ).ok();
            self.idle_count = 0;
        }
    }
}

fn process_input( input: &str ) -> HashMap<usize, i64> {
    return input.split( "," )
        .enumerate()
        .map( |(i, v)| (i, v.parse().unwrap()))
        .collect();
}

fn initialise_network() -> ( Network, Option< NAT > ) {
    let opcodes = process_input( DATA );
    let mut senders: Vec<SyncSender<i64>> = Vec::new();
    let mut consumers: Vec<ConnectConsumer> = Vec::new();
    let mut cpus: Vec<CPU<ConnectConsumer, ConnectProducer>> = Vec::new();

    // set up NAT
    let ( mut nat_sender, _ ) = sync_channel( 1000 );
    let mut nat: Option< NAT > = None;

    // set up sender and receivers for the producer and consumers
    for idx in 0.. NUMBER_OF_COMPUTERS {
        let ( tx, rx ) : ( SyncSender<i64>, Receiver<i64> ) = sync_channel( 1000 );
        tx.send( idx as i64 ).ok();
        
        if idx == 0 {
            let ( tx_nat, rx_nat ) : ( SyncSender<i64>, Receiver<i64> ) = sync_channel( 1000 );
            nat = Some( NAT::initialise( rx_nat, tx.clone() ) );
            nat_sender = tx_nat.clone();
        }

        senders.push( tx.clone() );
        consumers.push( ConnectConsumer::initialise( rx, nat_sender.clone(), idx as i64 ) );
    }

    // initialise cpu's
    for _ in 0.. NUMBER_OF_COMPUTERS {
        let producer = ConnectProducer::initialise( senders.clone(), nat_sender.clone() );
        let consumer = consumers.remove(0);
        
        let mut cpu = CPU::initialise( opcodes.clone(), consumer, producer );
        cpu.pause_execution_on_output( true );
        cpus.push( cpu );
    }

    return ( Network {
        network_interface_controllers: cpus
    }, nat );
}

fn part_one() {
    let ( mut network, nat_option ) = initialise_network();
    let mut nat = nat_option.unwrap();

    while !network.execute_loop( &mut nat ) {
    }
}

#[allow(dead_code)]
pub fn day_23( _: Vec<String> ) {
    part_one();
}
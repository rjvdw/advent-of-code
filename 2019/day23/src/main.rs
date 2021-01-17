use std::collections::VecDeque;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode;
use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);
    simulate_network(&program, 50, 255, false);
}

fn simulate_network(
    program: &intcode::Program,
    nr_nodes: usize,
    nat_address: i64,
    debugging: bool,
) {
    let mut network = build_network(program, nr_nodes);
    let mut idx = 0;
    let mut nat: Option<Packet> = None;
    let mut nat_prev: Option<Packet> = None;
    let mut idle = true;
    loop {
        for (address, packet) in network[idx].run() {
            idle = false;
            if debugging {
                println!("Node {} sends {:?} to node {}.", idx, packet, address);
            }
            if address == nat_address {
                if nat.is_none() {
                    println!("The first packet sent to the NAT address is {:?}.", packet);
                }
                nat_prev = nat;
                nat = Some(packet);
            } else {
                network[address as usize].queue(packet);
            }
        }

        idx += 1;
        if idx == nr_nodes {
            idx = 0;
            if idle {
                if nat == nat_prev {
                    println!(
                        "The packet {:?} is sent to node 0 twice in a row, halting.",
                        nat
                    );
                    return;
                }

                let packet = nat.unwrap();
                if debugging {
                    println!("The NAT sends {:?} to node 0.", packet);
                }
                network[0].queue(packet);
            }
            idle = true;
        }
    }
}

fn build_network(program: &intcode::Program, size: usize) -> Vec<Node> {
    let mut network = Vec::with_capacity(size);
    for i in 0..size {
        let mut program = program.clone();
        program.send_message(i as i64);
        network.push(Node {
            program,
            message_queue: VecDeque::new(),
        });
    }
    network
}

#[derive(Debug)]
struct Node {
    program: intcode::Program,
    message_queue: VecDeque<Packet>,
}

impl Node {
    fn run(&mut self) -> Vec<(i64, Packet)> {
        if let Some(packet) = self.message_queue.pop_front() {
            self.program.send_message(packet.x);
            self.program.send_message(packet.y);
        } else {
            self.program.send_message(-1);
        }
        self.program.run();
        let mut packets = vec![];
        while let Some(address) = self.program.receive_message() {
            packets.push((
                address,
                Packet {
                    x: self.program.receive_message().unwrap(),
                    y: self.program.receive_message().unwrap(),
                },
            ));
        }
        packets
    }

    fn queue(&mut self, packet: Packet) {
        self.message_queue.push_back(packet);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Packet {
    x: i64,
    y: i64,
}

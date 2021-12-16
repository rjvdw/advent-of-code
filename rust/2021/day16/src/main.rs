extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::Add;

use crate::evaluable_packet::EvaluablePacket;
use crate::parsable_packet::ParsablePacket;
use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;

mod evaluable_packet;
mod parsable_packet;

/// https://adventofcode.com/2021/day/16
fn main() {
    let args = get_args(&["<input file>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let reader = BufReader::new(file).lines();
    let packet = parse_input(reader).or_exit_with(1);

    println!("The sum of the versions is {}.", sum_versions(&packet));
    println!("The transmission evaluates to {}.", packet.eval());
}

fn parse_input<I>(mut reader: I) -> Result<Packet, ParseError>
where
    I: Iterator<Item = io::Result<String>>,
{
    let line = match reader.next() {
        Some(v) => v?,
        None => return Err(parse_error!("Invalid input file.")),
    };
    let bits = line.chars().flat_map(parse_char).collect::<String>();
    Packet::parse(&bits, 0).map(|(_, p)| p)
}

fn sum_versions(packet: &Packet) -> u64 {
    let mut sum = 0;
    match packet {
        Packet::Literal(lp) => {
            sum += lp.version as u64;
        }
        Packet::Operator(op) => {
            sum += op.version as u64;
            for p in &op.sub_packets {
                sum += sum_versions(p);
            }
        }
    }
    sum
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(dead_code)]
struct LiteralPacket {
    version: u8,
    type_id: u8,
    value: String,
}

impl ParsablePacket for LiteralPacket {
    fn parse(
        bits: &str,
        mut position: usize,
        version: u8,
        type_id: u8,
    ) -> Result<(usize, Self), ParseError> {
        let mut value = String::new();
        while position < bits.len() {
            let chunk = &bits[position..position + 5];
            position += 5;
            value = value.add(&chunk[1..]);
            if chunk.starts_with('0') {
                return Ok((
                    position,
                    LiteralPacket {
                        version,
                        type_id,
                        value,
                    },
                ));
            }
        }

        Err(parse_error!("Invalid literal packet"))
    }
}

impl EvaluablePacket for LiteralPacket {
    fn eval(&self) -> u64 {
        u64::from_str_radix(&self.value, 2).unwrap()
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct OperatorPacket {
    version: u8,
    type_id: u8,
    length_type_id: u8,
    sub_packets: Vec<Packet>,
}

impl ParsablePacket for OperatorPacket {
    fn parse(
        bits: &str,
        mut position: usize,
        version: u8,
        type_id: u8,
    ) -> Result<(usize, Self), ParseError> {
        let length_type_id = u8::from_str_radix(&bits[position..position + 1], 2)?;
        position += 1;

        let mut sub_packets = vec![];
        if length_type_id == 0 {
            let length = usize::from_str_radix(&bits[position..position + 15], 2)?;
            position += 15;

            let sub = &bits[position..position + length];
            let mut i = 0;
            while i < sub.len() {
                let (np, packet) = Packet::parse(sub, i)?;
                sub_packets.push(packet);
                i = np;
            }
            position += length;
        } else {
            let length = usize::from_str_radix(&bits[position..position + 11], 2)?;
            position += 11;

            while sub_packets.len() < length {
                let (np, packet) = Packet::parse(bits, position)?;
                sub_packets.push(packet);
                position = np;
            }
        }

        Ok((
            position,
            OperatorPacket {
                version,
                type_id,
                length_type_id,
                sub_packets,
            },
        ))
    }
}

impl EvaluablePacket for OperatorPacket {
    fn eval(&self) -> u64 {
        if self.type_id < 4 {
            let subs = self.sub_packets.iter().map(|p| p.eval());

            match self.type_id {
                0 => subs.sum(),
                1 => subs.product(),
                2 => subs.min().unwrap(),
                3 => subs.max().unwrap(),
                _ => panic!("Invalid type id: {}", self.type_id),
            }
        } else {
            let a = self.sub_packets[0].eval();
            let b = self.sub_packets[1].eval();
            let p = |b: bool| if b { 1 } else { 0 };

            match self.type_id {
                5 => p(a > b),
                6 => p(a < b),
                7 => p(a == b),
                _ => panic!("Invalid type id: {}", self.type_id),
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

impl Packet {
    fn parse(bits: &str, mut position: usize) -> Result<(usize, Self), ParseError> {
        let version = u8::from_str_radix(&bits[position..position + 3], 2)?;
        position += 3;
        let type_id = u8::from_str_radix(&bits[position..position + 3], 2)?;
        position += 3;

        if type_id == 4 {
            let (p, packet) = LiteralPacket::parse(bits, position, version, type_id)?;
            Ok((p, Packet::Literal(packet)))
        } else {
            let (p, packet) = OperatorPacket::parse(bits, position, version, type_id)?;
            Ok((p, Packet::Operator(packet)))
        }
    }
}

impl EvaluablePacket for Packet {
    fn eval(&self) -> u64 {
        match self {
            Packet::Literal(lp) => lp.eval(),
            Packet::Operator(op) => op.eval(),
        }
    }
}

fn parse_char(ch: char) -> [char; 4] {
    match ch {
        '0' => ['0', '0', '0', '0'],
        '1' => ['0', '0', '0', '1'],
        '2' => ['0', '0', '1', '0'],
        '3' => ['0', '0', '1', '1'],
        '4' => ['0', '1', '0', '0'],
        '5' => ['0', '1', '0', '1'],
        '6' => ['0', '1', '1', '0'],
        '7' => ['0', '1', '1', '1'],
        '8' => ['1', '0', '0', '0'],
        '9' => ['1', '0', '0', '1'],
        'A' | 'a' => ['1', '0', '1', '0'],
        'B' | 'b' => ['1', '0', '1', '1'],
        'C' | 'c' => ['1', '1', '0', '0'],
        'D' | 'd' => ['1', '1', '0', '1'],
        'E' | 'e' => ['1', '1', '1', '0'],
        'F' | 'f' => ['1', '1', '1', '1'],
        _ => panic!("Illegal character: {}", ch),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal_packet() {
        let bits = "110100101111111000101000";
        let (p, packet) = Packet::parse(bits, 0).unwrap();

        assert_eq!(p, 21);

        if let Packet::Literal(lp) = packet {
            assert_eq!(
                lp,
                LiteralPacket {
                    version: 6,
                    type_id: 4,
                    value: "011111100101".to_string(),
                }
            );
        } else {
            panic!("Expected a literal packet.");
        }
    }

    #[test]
    fn test_parse_operator_packet_with_length_type_0() {
        let bits = "00111000000000000110111101000101001010010001001000000000";
        let (p, packet) = Packet::parse(bits, 0).unwrap();

        assert_eq!(p, 49);

        if let Packet::Operator(op) = packet {
            assert_eq!(op.version, 1);
            assert_eq!(op.type_id, 6);
            assert_eq!(op.length_type_id, 0);
            assert_eq!(op.sub_packets.len(), 2);

            assert_sub_literal(
                &op.sub_packets,
                0,
                LiteralPacket {
                    version: 0b110,
                    type_id: 4,
                    value: "1010".to_string(),
                },
            );

            assert_sub_literal(
                &op.sub_packets,
                1,
                LiteralPacket {
                    version: 0b010,
                    type_id: 4,
                    value: "00010100".to_string(),
                },
            );
        } else {
            panic!("Expected an operator packet.");
        }
    }

    #[test]
    fn test_parse_operator_packet_with_length_type_1() {
        let bits = "11101110000000001101010000001100100000100011000001100000";
        let (p, packet) = Packet::parse(bits, 0).unwrap();

        assert_eq!(p, 51);

        if let Packet::Operator(op) = packet {
            assert_eq!(op.version, 7);
            assert_eq!(op.type_id, 3);
            assert_eq!(op.length_type_id, 1);
            assert_eq!(op.sub_packets.len(), 3);

            assert_sub_literal(
                &op.sub_packets,
                0,
                LiteralPacket {
                    version: 0b010,
                    type_id: 4,
                    value: "0001".to_string(),
                },
            );

            assert_sub_literal(
                &op.sub_packets,
                1,
                LiteralPacket {
                    version: 0b100,
                    type_id: 4,
                    value: "0010".to_string(),
                },
            );

            assert_sub_literal(
                &op.sub_packets,
                2,
                LiteralPacket {
                    version: 0b001,
                    type_id: 4,
                    value: "0011".to_string(),
                },
            );
        }
    }

    #[test]
    fn test_sum_versions() {
        let lines = vec![Ok("8A004A801A8002F478".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(sum_versions(&packet), 16);

        let lines = vec![Ok("620080001611562C8802118E34".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(sum_versions(&packet), 12);

        let lines = vec![Ok("C0015000016115A2E0802F182340".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(sum_versions(&packet), 23);

        let lines = vec![Ok("A0016C880162017C3686B18A3D4780".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(sum_versions(&packet), 31);
    }

    #[test]
    fn test_eval() {
        let lines = vec![Ok("C200B40A82".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 3);

        let lines = vec![Ok("04005AC33890".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 54);

        let lines = vec![Ok("880086C3E88112".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 7);

        let lines = vec![Ok("CE00C43D881120".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 9);

        let lines = vec![Ok("D8005AC2A8F0".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 1);

        let lines = vec![Ok("F600BC2D8F".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 0);

        let lines = vec![Ok("9C005AC2F8F0".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 0);

        let lines = vec![Ok("9C0141080250320F1802104A08".to_string())];
        let packet = parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 1);
    }

    fn assert_sub_literal(sub_packets: &[Packet], i: usize, expected: LiteralPacket) {
        if let Packet::Literal(actual) = &sub_packets[i] {
            assert_eq!(actual, &expected);
        } else {
            panic!("Expected a literal packet.");
        }
    }
}

use std::cmp::Ordering;
use std::io;
use std::ops::Add;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Packet {
    version: u8,
    type_id: u8,
    value: u64,
    sub_packets: Vec<Packet>,
}

impl Packet {
    pub fn sum_versions(&self) -> u64 {
        self.sub_packets
            .iter()
            .map(Packet::sum_versions)
            .sum::<u64>()
            + (self.version as u64)
    }

    pub fn eval(&self) -> u64 {
        match self.type_id.cmp(&4) {
            Ordering::Equal => self.value,
            Ordering::Less => {
                let subs = self.sub_packets.iter().map(Packet::eval);

                match self.type_id {
                    0 => subs.sum(),
                    1 => subs.product(),
                    2 => subs.min().unwrap(),
                    _ => subs.max().unwrap(),
                }
            }
            Ordering::Greater => {
                let a = self.sub_packets[0].eval();
                let b = self.sub_packets[1].eval();

                u64::from(match self.type_id {
                    5 => a > b,
                    6 => a < b,
                    _ => a == b,
                })
            }
        }
    }

    pub fn parse_input<I>(mut lines: I) -> Result<Packet, ParseError>
    where
        I: Iterator<Item = io::Result<String>>,
    {
        let bits = lines
            .next()
            .unwrap()?
            .chars()
            .flat_map(parse_char)
            .collect::<String>();

        Self::parse(&bits, 0).map(|(_, packet)| packet)
    }

    fn parse(bits: &str, mut position: usize) -> Result<(usize, Packet), ParseError> {
        let version = u8::from_str_radix(&bits[position..position + 3], 2)?;
        position += 3;
        let type_id = u8::from_str_radix(&bits[position..position + 3], 2)?;
        position += 3;

        if type_id == 4 {
            Self::parse_literal(bits, position, version, type_id)
        } else {
            Self::parse_operator(bits, position, version, type_id)
        }
    }

    fn parse_literal(
        bits: &str,
        mut position: usize,
        version: u8,
        type_id: u8,
    ) -> Result<(usize, Self), ParseError> {
        let mut value = String::new();
        loop {
            let chunk = &bits[position..position + 5];
            position += 5;
            value = value.add(&chunk[1..]);
            if chunk.starts_with('0') {
                return Ok((
                    position,
                    Packet {
                        version,
                        type_id,
                        value: u64::from_str_radix(&value, 2)?,
                        sub_packets: vec![],
                    },
                ));
            }
        }
    }

    fn parse_operator(
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
            position += length;

            let mut i = 0;
            while i < sub.len() {
                let (np, packet) = Packet::parse(sub, i)?;
                sub_packets.push(packet);
                i = np;
            }
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
            Packet {
                version,
                type_id,
                value: 0,
                sub_packets,
            },
        ))
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
        assert_eq!(
            packet,
            Packet {
                version: 6,
                type_id: 4,
                value: 2021,
                sub_packets: vec![],
            }
        );
    }

    #[test]
    fn test_parse_operator_packet_with_length_type_0() {
        let bits = "00111000000000000110111101000101001010010001001000000000";
        let (p, packet) = Packet::parse(bits, 0).unwrap();

        assert_eq!(p, 49);
        assert_eq!(
            packet,
            Packet {
                version: 1,
                type_id: 6,
                value: 0,
                sub_packets: vec![
                    Packet {
                        version: 6,
                        type_id: 4,
                        value: 10,
                        sub_packets: vec![],
                    },
                    Packet {
                        version: 2,
                        type_id: 4,
                        value: 20,
                        sub_packets: vec![],
                    },
                ],
            }
        );
    }

    #[test]
    fn test_parse_operator_packet_with_length_type_1() {
        let bits = "11101110000000001101010000001100100000100011000001100000";
        let (p, packet) = Packet::parse(bits, 0).unwrap();

        assert_eq!(p, 51);
        assert_eq!(
            packet,
            Packet {
                version: 7,
                type_id: 3,
                value: 0,
                sub_packets: vec![
                    Packet {
                        version: 2,
                        type_id: 4,
                        value: 1,
                        sub_packets: vec![],
                    },
                    Packet {
                        version: 4,
                        type_id: 4,
                        value: 2,
                        sub_packets: vec![],
                    },
                    Packet {
                        version: 1,
                        type_id: 4,
                        value: 3,
                        sub_packets: vec![],
                    },
                ],
            }
        );
    }

    #[test]
    fn test_sum_versions() {
        let lines = vec![Ok("8A004A801A8002F478".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.sum_versions(), 16);

        let lines = vec![Ok("620080001611562C8802118E34".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.sum_versions(), 12);

        let lines = vec![Ok("C0015000016115A2E0802F182340".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.sum_versions(), 23);

        let lines = vec![Ok("A0016C880162017C3686B18A3D4780".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.sum_versions(), 31);
    }

    #[test]
    fn test_eval() {
        let lines = vec![Ok("C200B40A82".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 3);

        let lines = vec![Ok("04005AC33890".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 54);

        let lines = vec![Ok("880086C3E88112".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 7);

        let lines = vec![Ok("CE00C43D881120".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 9);

        let lines = vec![Ok("D8005AC2A8F0".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 1);

        let lines = vec![Ok("F600BC2D8F".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 0);

        let lines = vec![Ok("9C005AC2F8F0".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 0);

        let lines = vec![Ok("9C0141080250320F1802104A08".to_string())];
        let packet = Packet::parse_input(lines.into_iter()).unwrap();
        assert_eq!(packet.eval(), 1);
    }
}

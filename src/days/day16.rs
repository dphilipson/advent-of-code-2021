use crate::harness::input::RawInput;
use hex::FromHex;
use std::iter;

pub fn solve_part1(input: RawInput) -> u32 {
    let bytes = input.single_line(|line| Vec::from_hex(line.as_str()).unwrap());
    let mut parser = Parser::new(&bytes);
    parser.read_packet();
    parser.version_sum
}

pub fn solve_part2(input: RawInput) -> u64 {
    let bytes = input.single_line(|line| Vec::from_hex(line.as_str()).unwrap());
    let packet = Parser::new(&bytes).read_packet();
    packet.value()
}

#[derive(Debug)]
enum Packet {
    Literal(u64),
    Operator(u32, Vec<Packet>),
}

#[derive(Debug)]
struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
    version_sum: u32,
}

impl<'a> Parser<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Parser {
            bytes,
            pos: 0,
            version_sum: 0,
        }
    }

    fn read_packet(&mut self) -> Packet {
        self.version_sum += self.read_number(3);
        let packet_type = self.read_number(3);
        match packet_type {
            4 => self.read_literal(),
            _ => Packet::Operator(packet_type, self.read_subpackets()),
        }
    }

    fn read_literal(&mut self) -> Packet {
        let mut result = 0;
        while self.read_bit() {
            result = 16 * result + self.read_number(4) as u64;
        }
        result = 16 * result + self.read_number(4) as u64;
        Packet::Literal(result)
    }

    fn read_subpackets(&mut self) -> Vec<Packet> {
        if self.read_bit() {
            self.read_counted_subpackets()
        } else {
            self.read_bit_counted_subpackets()
        }
    }

    fn read_bit_counted_subpackets(&mut self) -> Vec<Packet> {
        let nbits = self.read_number(15);
        let end = self.pos + nbits as usize;
        let mut subpackets = vec![];
        while self.pos < end {
            subpackets.push(self.read_packet());
        }
        subpackets
    }

    fn read_counted_subpackets(&mut self) -> Vec<Packet> {
        let nsubpackets = self.read_number(11) as usize;
        iter::repeat_with(|| self.read_packet())
            .take(nsubpackets)
            .collect()
    }

    fn read_number(&mut self, nbits: usize) -> u32 {
        (0..nbits).fold(0, |acc, _| 2 * acc + self.read_bit() as u32)
    }

    fn read_bit(&mut self) -> bool {
        let bit = get_bit(self.bytes, self.pos);
        self.pos += 1;
        bit
    }
}

impl Packet {
    fn value(&self) -> u64 {
        match self {
            Self::Literal(value) => *value,
            Self::Operator(op_type, subpackets) => {
                let mut subvalues = subpackets.iter().map(|subpacket| subpacket.value());
                match *op_type {
                    0 => subvalues.sum(),
                    1 => subvalues.product(),
                    2 => subvalues.min().unwrap(),
                    3 => subvalues.max().unwrap(),
                    _ => {
                        let a = subvalues.next().unwrap();
                        let b = subvalues.next().unwrap();
                        (match *op_type {
                            5 => a > b,
                            6 => a < b,
                            7 => a == b,
                            _ => unreachable!(),
                        }) as u64
                    }
                }
            }
        }
    }
}

fn get_bit(bytes: &[u8], i: usize) -> bool {
    bytes[i / 8] & (1 << (7 - (i % 8))) > 0
}

mod decoder {
    #[derive(Clone, Debug)]
    pub enum Packet {
        Operator(Operator),
        Literal(Literal),
    }

    #[derive(Clone, Debug)]
    pub struct Literal {
        pub header: Header,
        pub value: u64,
        size: usize,
    }

    #[derive(Clone, Debug)]
    pub struct Operator {
        pub header: Header,
        pub children: Vec<Packet>,
        size: usize,
    }

    #[derive(Clone, Debug)]
    pub struct Header {
        pub version: u64,
        pub type_id: u64,
    }

    pub fn decode(message: &str) -> Packet {
        decode_packet(&decode_message(message))
    }

    fn decode_packet(bits: &[u8]) -> Packet {
        if to_u64(&bits[3..6]) == 4 {
            decode_literal(bits)
        } else {
            decode_operator(bits)
        }
    }

    fn decode_literal(bits: &[u8]) -> Packet {
        let rest = &bits[6..];
        let mut index = 0;
        let mut num = Vec::new();

        while index <= rest.len() - 5 {
            let next = index + 5;
            let chunk = &rest[index..next];
            let signal = chunk[0];

            num.extend(&chunk[1..5]);
            index = next;

            if signal == 0 {
                break;
            }
        }

        Packet::Literal(Literal {
            header: decode_header(bits),
            value: to_u64(&num),
            size: 6 + index,
        })
    }

    fn decode_operator(bits: &[u8]) -> Packet {
        let mode = bits[6];
        let content_offset = 7 + if mode == 0 { 15 } else { 11 };
        let len = to_u64(&bits[7..content_offset]) as usize;

        let mut children = Vec::new();
        let mut index = 0;

        while (mode == 0 && index < len) || (mode == 1 && children.len() < len) {
            let packet = decode_packet(&bits[(content_offset + index)..]);

            match &packet {
                Packet::Literal(data) => index += data.size,
                Packet::Operator(data) => index += data.size,
            }

            children.push(packet);
        }

        Packet::Operator(Operator {
            header: decode_header(bits),
            children,
            size: content_offset + index,
        })
    }

    fn decode_header(bits: &[u8]) -> Header {
        Header {
            version: to_u64(&bits[0..3]),
            type_id: to_u64(&bits[3..6]),
        }
    }

    // instruction set is small, use a lookup table.
    fn decode_message(message: &str) -> Vec<u8> {
        message
            .chars()
            .flat_map(|c| match c {
                '0' => [0, 0, 0, 0],
                '1' => [0, 0, 0, 1],
                '2' => [0, 0, 1, 0],
                '3' => [0, 0, 1, 1],
                '4' => [0, 1, 0, 0],
                '5' => [0, 1, 0, 1],
                '6' => [0, 1, 1, 0],
                '7' => [0, 1, 1, 1],
                '8' => [1, 0, 0, 0],
                '9' => [1, 0, 0, 1],
                'A' => [1, 0, 1, 0],
                'B' => [1, 0, 1, 1],
                'C' => [1, 1, 0, 0],
                'D' => [1, 1, 0, 1],
                'E' => [1, 1, 1, 0],
                'F' => [1, 1, 1, 1],
                c => panic!("unexpected token in message: {}", c),
            })
            .collect()
    }

    fn to_u64(bits: &[u8]) -> u64 {
        bits.iter().fold(0, |acc, &b| acc * 2 + (b as u64))
    }
}

mod interpreter {
    use super::decoder::Packet;

    pub fn interpret(packet: Packet) -> u64 {
        match packet {
            Packet::Literal(_) => panic!("unexpected literal on root level."),
            Packet::Operator(data) => {
                let values: Vec<u64> = data
                    .children
                    .iter()
                    .map(|child| match child {
                        Packet::Literal(child_data) => child_data.value as u64,
                        Packet::Operator(child_data) => {
                            interpret(Packet::Operator(child_data.clone()))
                        }
                    })
                    .collect();

                match data.header.type_id {
                    0 => values.iter().sum(),
                    1 => values.iter().product(),
                    2 => *values.iter().min().unwrap(),
                    3 => *values.iter().max().unwrap(),
                    5 => (values[0] > values[1]) as u64,
                    6 => (values[0] < values[1]) as u64,
                    7 => (values[0] == values[1]) as u64,
                    c => panic!("unknown type_id {}", c),
                }
            }
        }
    }

    pub fn sum_versions(packet: Packet) -> u64 {
        match packet {
            Packet::Literal(_) => panic!("unexpected literal on root level."),
            Packet::Operator(data) => {
                data.children
                    .iter()
                    .fold(data.header.version, |acc, curr| match curr {
                        Packet::Literal(child_data) => acc + child_data.header.version,
                        Packet::Operator(child_data) => {
                            acc + sum_versions(Packet::Operator(child_data.clone()))
                        }
                    })
            }
        }
    }
}

use self::decoder::decode;
use self::interpreter::{interpret, sum_versions};

pub fn part_one(input: &str) -> u64 {
    let packet = decode(input.lines().next().unwrap());
    sum_versions(packet)
}

pub fn part_two(input: &str) -> u64 {
    let packet = decode(input.lines().next().unwrap());
    interpret(packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("8A004A801A8002F478"), 16);
        assert_eq!(part_one("A0016C880162017C3686B18A3D4780"), 31);
        assert_eq!(part_one("620080001611562C8802118E34"), 12);
        assert_eq!(part_one("C0015000016115A2E0802F182340"), 23);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("C200B40A82"), 3);
        assert_eq!(part_two("04005AC33890"), 54);
        assert_eq!(part_two("880086C3E88112"), 7);
        assert_eq!(part_two("CE00C43D881120"), 9);
        assert_eq!(part_two("D8005AC2A8F0"), 1);
        assert_eq!(part_two("F600BC2D8F"), 0);
        assert_eq!(part_two("9C005AC2F8F0"), 0);
        assert_eq!(part_two("9C0141080250320F1802104A08"), 1);
    }
}

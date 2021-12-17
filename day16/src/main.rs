use std::env;
use std::fs;

#[derive (Debug)]
enum PacketType {
    Literal(u64),
    Operator(Vec<Packet>)
}

#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: TypeId,
    content: PacketType,
    bits: usize
}

#[derive(Debug)]
enum TypeId {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    Literal,
    LessThan,
    EqualTo
}

fn int_to_typeid(int: u64) -> TypeId {
    match int {
        0 => TypeId::Sum,
        1 => TypeId::Product,
        2 => TypeId::Minimum,
        3 => TypeId::Maximum,
        4 => TypeId::Literal,
        5 => TypeId::GreaterThan,
        6 => TypeId::LessThan,
        7 => TypeId::EqualTo,
        _ => panic!()
    }
}

fn parse_file_to_binary(filename: &String) -> Vec<u8> {
    fs::read_to_string(filename)
        .unwrap()
        .trim()
        .chars()
        .flat_map(|c| match c {
            '0' => vec![0,0,0,0],
            '1' => vec![0,0,0,1],
            '2' => vec![0,0,1,0],
            '3' => vec![0,0,1,1],
            '4' => vec![0,1,0,0],
            '5' => vec![0,1,0,1],
            '6' => vec![0,1,1,0],
            '7' => vec![0,1,1,1],
            '8' => vec![1,0,0,0],
            '9' => vec![1,0,0,1],
            'A' => vec![1,0,1,0],
            'B' => vec![1,0,1,1],
            'C' => vec![1,1,0,0],
            'D' => vec![1,1,0,1],
            'E' => vec![1,1,1,0],
            'F' => vec![1,1,1,1],
            _ => panic!("unrecognised char {}", c)
        }).collect()
}

fn bits_to_dec(bits: &[u8]) -> u64 {
    bits.iter().fold(0, |dec, bit| dec << 1 ^ *bit as u64)
}

fn parse_literal(bits: &[u8]) -> (u64, usize) {
    let chunk_count = 1 + bits.iter().step_by(5).position(|b| b == &0).unwrap();
    let literal :Vec<_> = bits.chunks(5)
                              .take(chunk_count)
                              .flat_map(|chunk| chunk[1..5].to_vec())
                              .collect();
    (bits_to_dec(&literal), chunk_count*5)
}

fn parse_operator(bits: &[u8]) -> (Vec<Packet>, usize) {
    let length_type_id = bits[0];
    match length_type_id {
        0 => {
            let length_in_bits = bits_to_dec(&bits[1..16]) as usize;
            let mut packets = Vec::new();
            let mut start = 16;
            while start < 16 + length_in_bits {
                packets.push(parse_packet(&bits[start..]));
                start += packets.last().unwrap().bits;
            };
            return (packets, length_in_bits + 16) // 15 bit number + 1 bit id
        }
        1 => {
            let num_packets = bits_to_dec(&bits[1..12]);
            let mut start = 12;
            let mut packets = Vec::new();
            for _ in 0..num_packets {
                packets.push(parse_packet(&bits[start..]));
                start += packets.last().unwrap().bits;
            }
            return (packets, start);
        }
        _ => {
            panic!() 
        }
    }
}

fn parse_packet(input: &[u8]) -> Packet {
    let version = bits_to_dec(&input[..3]);
    let type_id = int_to_typeid(bits_to_dec(&input[3..6]));
    println!("type_id : {:?}", type_id);

    let (content, bits) = match type_id {
        TypeId::Literal => {
            let (value, bits) = parse_literal(&input[6..]);
            (PacketType::Literal(value), bits)
        },
        _ => {
            let (value, bits) = parse_operator(&input[6..]);
            (PacketType::Operator(value), bits)
        }
    };

    Packet {
        version: version,
        type_id: type_id,
        content: content,
        bits: bits + 6 // 6 bit header
    }
}

fn get_version_number_sum(packet: &Packet) -> u64 {
    match &packet.content {
       PacketType::Literal(_) => packet.version,
       PacketType::Operator(sub_packets) => 
           sub_packets.iter()
                       .fold(packet.version, |acc, sub_packet| 
                             acc + get_version_number_sum(sub_packet))
    }
}

fn calculate_value(packet: &Packet) -> u64 {
    match &packet.content {
        PacketType::Literal(value) => *value as u64,
        PacketType::Operator(sub_packets) => {
            let mut sub_packet_values = sub_packets.iter().map(|packet| calculate_value(packet));
            match packet.type_id {
                TypeId::Sum => sub_packet_values.sum(),
                TypeId::Product => sub_packet_values.product(),
                TypeId::Minimum => sub_packet_values.min().unwrap(),
                TypeId::Maximum => sub_packet_values.max().unwrap(),
                TypeId::GreaterThan => 
                    if sub_packet_values.next().unwrap() > 
                        sub_packet_values.next().unwrap() { 1 } else { 0 },
                TypeId::Literal => unreachable!(),
                TypeId::LessThan => 
                    if sub_packet_values.next().unwrap() <
                        sub_packet_values.next().unwrap() { 1 } else { 0 },
                TypeId::EqualTo => 
                    if sub_packet_values.next().unwrap() ==
                        sub_packet_values.next().unwrap() { 1 } else { 0 },
            }
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = parse_file_to_binary(&args[1]);

    let packet = parse_packet(&input);
    println!("{:#?}", packet);

    let version_sum = get_version_number_sum(&packet);
    println!("version sum: {}", version_sum);

    let value = calculate_value(&packet);
    println!("BITS value: {}", value);
}

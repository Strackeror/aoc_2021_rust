use std::collections::VecDeque;

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal(u32, u64),
    Operator(u32, u32, Vec<Packet>),
}

fn value(bits: impl IntoIterator<Item = bool>) -> u64 {
    bits.into_iter()
        .fold(0, |acc, bit| acc * 2 + if bit { 1 } else { 0 })
}

fn parse_literal(version: u32, bits: &mut VecDeque<bool>) -> Result<Packet> {
    let mut result = 0;
    loop {
        let mut chunk = bits.drain(0..5);
        let cont = chunk.next().unwrap();

        result = result * 16 + value(chunk);
        if !cont {
            break;
        }
    }

    Ok(Packet::Literal(version, result))
}

fn parse_operator(version: u32, packet_type: u32, bits: &mut VecDeque<bool>) -> Result<Packet> {
    let len_type = bits.drain(0..1).next().unwrap();
    let mut sub_packets = Vec::new();
    if !len_type {
        let len = value(bits.drain(0..15)) as usize;
        let mut sub_slice = bits.drain(0..len).collect::<VecDeque<_>>();
        while !sub_slice.is_empty() {
            sub_packets.push(parse(&mut sub_slice)?);
        }
    } else {
        let count = value(bits.drain(0..11)) as usize;
        for _ in 0..count {
            sub_packets.push(parse(bits)?);
        }
    }
    Ok(Packet::Operator(version, packet_type, sub_packets))
}

fn parse(bits: &mut VecDeque<bool>) -> Result<Packet> {
    let version = value(bits.drain(0..3));
    let packet_type = value(bits.drain(0..3));

    match packet_type {
        4 => parse_literal(version as _, bits),
        _ => parse_operator(version as _, packet_type as _, bits),
    }
}

fn parse_str(instructions: &str) -> Result<Packet> {
    let mut bits = instructions
        .trim()
        .chars()
        .flat_map(|c| {
            let i = i8::from_str_radix(&String::from_iter([c]), 16).unwrap();
            (0..4).map(move |bit| (i & (8 >> bit)) != 0)
        })
        .collect::<VecDeque<bool>>();
    parse(&mut bits)
}

fn version_sum(packet: &Packet) -> u32 {
    match packet {
        Packet::Literal(version, _) => *version,
        Packet::Operator(version, _, sub_packets) => {
            *version + sub_packets.iter().map(version_sum).sum::<u32>()
        }
    }
}

fn eval_packet(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(_, val) => *val,
        Packet::Operator(_, op_type, sub_packets) => {
            let mut sub_values = sub_packets.iter().map(eval_packet);
            match op_type {
                0 => sub_values.sum::<u64>(),
                1 => sub_values.product::<u64>(),
                2 => sub_values.min().unwrap(),
                3 => sub_values.max().unwrap(),
                5 => (sub_values.next() > sub_values.next()) as u64,
                6 => (sub_values.next() < sub_values.next()) as u64,
                7 => (sub_values.next() == sub_values.next()) as u64,
                _ => unreachable!(),
            }
        }
    }
}

pub fn run(instructions: &str) -> Result<()> {
    let packet = parse_str(instructions)?;

    dbg!(&packet);
    dbg!(version_sum(&packet));
    dbg!(eval_packet(&packet));
    Ok(())
}

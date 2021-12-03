use std::clone::Clone;

use anyhow::Result;
use itertools::Itertools;

fn day01() -> Result<()> {
    let file = std::fs::read_to_string("input01.txt")?;
    let ints = file.split('\n').filter_map(|s| str::parse::<u32>(s).ok());

    let ex1: usize = ints.clone().tuple_windows().filter(|(a, b)| b > a).count();
    println!("ex01: {}", ex1);

    let ex2: usize = ints
        .tuple_windows()
        .map(|(a, b, c)| [a, b, c].iter().sum::<u32>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();

    println!("ex02: {}", ex2);

    Ok(())
}

fn day02(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path)?;
    let instructions = file
        .lines()
        .map(|l| l.split(' ').collect_vec())
        .map(|vec| (vec[0], vec[1].parse::<i64>().unwrap()));

    let (x, y, aim) = instructions.fold((0, 0, 0), |(x, y, aim), inst| match inst {
        ("forward", n) => (x + n, y + n * aim, aim),
        ("down", n) => (x, y, aim + n),
        ("up", n) => (x, y, aim - n),
        inst => panic!("invalid line {:?}", inst),
    });

    println!("ex1 position:{:?} ex1 result:{:?}", (x, aim), x * aim); // aim in ex2 is just depth in ex1
    println!("ex2 position:{:?} ex2 result:{:?}", (x, y), x * y);

    Ok(())
}

fn day03(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path)?;
    let bits = file.lines().map(|s| s.chars().collect_vec()).map(|v| {
        v.chunks(1)
            .map(|c| c.iter().collect::<String>())
            .filter_map(|c| c.parse::<i32>().ok())
            .map(|i| i != 0)
            .collect_vec()
    });

    fn common_bits(it: impl Iterator<Item = Vec<bool>> + Clone) -> Vec<bool> {
        let len = it.clone().count();
        let size = it.clone().next().unwrap().len();
        it.fold(vec![0; size], |acc, v| {
            acc.iter()
                .zip(v.iter())
                .map(|(a, b)| a + if *b { 1 } else { 0 })
                .collect_vec()
        })
        .iter()
        .map(move |i| *i * 2 >= len)
        .collect()
    }

    fn bits_to_int(vec: impl IntoIterator<Item = bool>) -> i32 {
        vec.into_iter()
            .fold(0, |acc, i| (acc << 1) + if i { 1 } else { 0 })
    }

    let gamma = bits_to_int(common_bits(bits.clone()));
    let epsilon = bits_to_int(common_bits(bits.clone()).iter().map(|b| !b));

    dbg!((gamma, epsilon, gamma * epsilon));

    let mut filtered_bits = bits.clone().collect_vec();
    let mut filter = vec![false; 0];
    while filtered_bits.len() > 1 {
        let common_bits = common_bits(filtered_bits.clone().into_iter());
        filter.push(common_bits[filter.len()]);
        filtered_bits = filtered_bits
            .iter()
            .filter(|bits| bits.starts_with(filter.as_slice()))
            .map(Clone::clone)
            .collect();
    }
    let oxygen = bits_to_int(filtered_bits[0].clone());

    let mut filtered_bits = bits.collect_vec();
    let mut filter = vec![false; 0];
    while filtered_bits.len() > 1 {
        let common_bits = common_bits(filtered_bits.clone().into_iter());
        dbg!(common_bits.clone());
        filter.push(!common_bits[filter.len()]);
        filtered_bits = filtered_bits
            .iter()
            .filter(|bits| bits.starts_with(filter.as_slice()))
            .map(Clone::clone)
            .collect();
    }
    let co2 = bits_to_int(filtered_bits[0].clone());

    dbg!((oxygen, co2, oxygen * co2));
    Ok(())
}

fn main() {
    println!("{:?}", day03(std::env::args().collect_vec()[1].as_str()));
}

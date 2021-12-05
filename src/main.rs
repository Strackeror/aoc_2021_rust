use std::{
    clone::Clone,
    collections::HashMap,
    io::BufRead,
    ops::{Deref, Range},
};

use anyhow::Result;
use itertools::Itertools;

type Coords = (i32, i32);

fn day05(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path)?;
    let coords: Vec<(Coords, Coords)> = file
        .lines()
        .map(|line| {
            line.replace("->", "")
                .replace(",", " ")
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_vec()
        })
        .map(|v| ((v[0], v[1]), (v[2], v[3])))
        .collect();

    fn range(start: i32, end: i32) -> Box<dyn Iterator<Item = i32>> {
        match start.cmp(&end) {
            std::cmp::Ordering::Equal => Box::new(std::iter::repeat(start)),
            std::cmp::Ordering::Greater => Box::new((end..start + 1).rev()),
            std::cmp::Ordering::Less => Box::new(start..end + 1),
        }
    }
    let mut counts: HashMap<Coords, i32> = Default::default();
    coords
        .iter()
        // Uncomment for ex1
        //.filter(|(start, end)| start.0 == end.0 || start.1 == end.1)
        .for_each(|(start, end)| {
            range(start.0, end.0)
                .zip(range(start.1, end.1))
                .for_each(|(x, y)| *counts.entry((x, y)).or_insert(0) += 1);
        });

    let count = counts.iter().filter(|(_, count)| **count > 1).count();
    //dbg!(counts.iter().sorted().collect_vec());
    dbg!(count);
    Ok(())
}

type Grid = Vec<Vec<Option<i32>>>;
fn day04(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let numbers = file
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec();

    let grids = file
        .lines()
        .skip(1)
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            chunk
                .skip(1)
                .map(|grid_line| {
                    grid_line
                        .split_whitespace()
                        .map(|s| Some(s.parse::<i32>().unwrap()))
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    fn is_won(grid: &Grid) -> bool {
        grid.iter().any(|l| l.iter().all(Option::is_none))
            || (0..grid[0].len()).any(|i| grid.iter().all(|l| l[i].is_none()))
    }

    fn grid_result(grid: &Grid, number: i32) -> i32 {
        grid.iter()
            .map(|l| l.iter().filter_map(|o| *o).sum::<i32>())
            .sum::<i32>()
            * number
    }

    let mut marked_grids = grids;
    let mut won_results = vec![0i32; 0];

    for num in numbers {
        for grid in marked_grids.iter_mut() {
            for line in grid.iter_mut() {
                for grid_num in line.iter_mut() {
                    if *grid_num == Some(num) {
                        *grid_num = None
                    }
                }
            }
        }

        let (won, left): (Vec<Grid>, Vec<Grid>) = marked_grids.into_iter().partition(is_won);
        marked_grids = left;
        won_results.extend(won.into_iter().map(|g| grid_result(&g, num)));
    }
    dbg!(won_results.first(), won_results.last());

    Ok(())
}

fn day01(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path)?;
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
    let bits = file
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(2).unwrap() != 0).collect_vec())
        .collect_vec();

    fn common_bits(vec: &[Vec<bool>]) -> Vec<bool> {
        let len = vec.len();
        vec.iter()
            .fold(vec![0; vec[0].len()], |acc, v| {
                acc.iter()
                    .zip(v.iter())
                    .map(|(a, b)| a + if *b { 1 } else { 0 })
                    .collect_vec()
            })
            .iter()
            .map(move |i| *i * 2 >= len)
            .collect()
    }

    fn bits_to_int(it: impl IntoIterator<Item = bool>) -> i32 {
        it.into_iter()
            .fold(0, |acc, i| (acc << 1) + if i { 1 } else { 0 })
    }

    let gamma = bits_to_int(common_bits(&bits));
    let epsilon = bits_to_int(common_bits(&bits).iter().map(|b| !b));

    dbg!((gamma, epsilon, gamma * epsilon));

    let mut filtered_bits = bits.clone();
    let mut filter = vec![false; 0];
    while filtered_bits.len() > 1 {
        filter.push(common_bits(&filtered_bits)[filter.len()]);
        filtered_bits.retain(|v| v.starts_with(filter.as_slice()));
    }
    let oxygen = bits_to_int(filtered_bits[0].clone());

    let mut filtered_bits = bits;
    let mut filter = vec![false; 0];
    while filtered_bits.len() > 1 {
        filter.push(!common_bits(&filtered_bits)[filter.len()]);
        filtered_bits.retain(|v| v.starts_with(filter.as_slice()))
    }
    let co2 = bits_to_int(filtered_bits[0].clone());

    dbg!((oxygen, co2, oxygen * co2));
    Ok(())
}

fn day03_bin(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path)?;
    let lines = file.lines();

    let bit_count = lines.clone().next().unwrap().len();
    let nums = lines
        .map(|l| i32::from_str_radix(l, 2).unwrap())
        .collect_vec();

    let bits = || (0..bit_count).into_iter().rev().map(|i| 1 << i);
    fn common_bit(nums: &[i32], bit: i32) -> i32 {
        if nums.iter().filter(|num| (*num & bit) != 0).count() * 2 >= nums.len() {
            bit
        } else {
            0
        }
    }

    let gamma = bits().fold(0, |acc, bit| acc | common_bit(&nums, bit));
    let epsilon = bits().fold(0, |acc, bit| acc | (common_bit(&nums, bit) ^ bit));
    dbg!(gamma, epsilon, gamma * epsilon);

    let oxygen = bits().fold(nums.clone(), |nums, bit| {
        let common_bit = common_bit(&nums, bit);
        nums.into_iter()
            .filter(|num| num & bit == common_bit)
            .collect()
    })[0];

    let co2 = bits().fold(nums, |nums, bit| {
        let common_bit = common_bit(&nums, bit);
        if nums.len() == 1 {
            return nums;
        }
        nums.into_iter()
            .filter(|num| num & bit != common_bit)
            .collect()
    })[0];

    dbg!(oxygen, co2, oxygen * co2);

    Ok(())
}

fn main() {
    let args = std::env::args().collect_vec();
    let input_path = args[1].clone();

    match args.get(2) {
        Some(n) => match n.as_str() {
            "day01" => day01(&input_path),
            "day02" => day02(&input_path),
            "day03" => day03(&input_path),
            "day03_bin" => day03_bin(&input_path),
            "day04" => day04(&input_path),
            _ => panic!("unexpected arg"),
        },
        None => day05(&input_path),
    }
    .unwrap();
}

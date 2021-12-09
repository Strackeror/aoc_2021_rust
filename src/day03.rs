use anyhow::Result;
use itertools::Itertools;

pub(crate) fn day03(path: &str) -> Result<()> {
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

pub(crate) fn day03_bin(path: &str) -> Result<()> {
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

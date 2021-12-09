use anyhow::Result;
use itertools::Itertools;

pub(crate) fn day01(path: &str) -> Result<()> {
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

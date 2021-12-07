use anyhow::Context;
use itertools::Itertools;

pub fn day07(path: &str) -> anyhow::Result<()> {
    let inputs: Vec<i32> = std::fs::read_to_string(path)?
        .trim()
        .split(',')
        .map(str::parse::<i32>)
        .collect::<Result<_, _>>()?;

    // ex1
    let max = *inputs.iter().max().context("no max")?;
    let minfuel = (0..max)
        .map(|t| inputs.iter().map(|i| (*i - t).abs()).sum::<i32>())
        .min()
        .context("no min")?;
    dbg!(max, minfuel);

    // ex2
    // precalc triangle numbers
    let distances = (0..=max)
        .scan(0, |acc, i| {
            *acc += i;
            Some(*acc)
        })
        .collect_vec();

    dbg!(&distances);
    let minfuel = (0..max)
        .map(|t| {
            inputs
                .iter()
                .map(|i| distances[(*i - t).abs() as usize])
                .sum::<i32>()
        })
        .min()
        .context("no min")?;
    dbg!(minfuel);

    Ok(())
}

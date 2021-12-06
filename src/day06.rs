use itertools::Itertools;

pub fn day06(path: &str) -> anyhow::Result<()> {
    let input: Vec<usize> = std::fs::read_to_string(path)?
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();
    let states = (0..=8)
        .map(|i| {
            (0..128).fold(vec![i], |acc, _| {
                acc.iter()
                    .flat_map(|f| match f {
                        0 => vec![6, 8],
                        n => vec![n - 1],
                    })
                    .collect()
            })
        })
        .collect_vec();
    let count: usize = input
        .iter()
        .map(|i| states[*i].iter().map(|i| states[*i].len()).sum::<usize>())
        .sum();
    dbg!(count);
    Ok(())
}

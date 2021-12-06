use std::vec;
pub fn day06(path: &str) -> anyhow::Result<()> {
    let input: Vec<usize> = std::fs::read_to_string(path)?
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();

    let step = |v: Vec<_>, _| vec![v[1], v[2], v[3], v[4], v[5], v[6], v[7] + v[0], v[8], v[0]];
    let count: usize = input
        .iter()
        .map(|f| {
            let mut init = vec![0usize; 9];
            init[*f] = 1;
            (0..256).fold(init, step).iter().sum::<usize>()
        })
        .sum();
    println!("count:{}", count);

    Ok(())
}

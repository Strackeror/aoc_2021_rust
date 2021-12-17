use anyhow::{Context, Result};
use itertools::Itertools;

fn gauss(i: i32) -> i32 {
    i * (i + 1) / 2
}
fn ypos(velocity: i32, step: i32) -> i32 {
    velocity * step - gauss(step - 1)
}

fn xpos(velocity: i32, step: i32) -> i32 {
    if velocity >= step {
        velocity * step - gauss(step - 1)
    } else {
        gauss(velocity)
    }
}

pub fn run(instruction: &str) -> Result<()> {
    let (xstart, xend, ystart, yend) = instruction
        .split(',')
        .map(|n| dbg!(n))
        .map(str::parse)
        .map(Result::unwrap)
        .next_tuple()
        .context("parsefailed")?;
    let xrange = xstart..=xend;
    let yrange = ystart..=yend;

    let results = (0..=xend * 4)
        .cartesian_product(0..=xend * 4)
        .filter(|(step, x)| xrange.contains(&xpos(*x, *step)))
        .cartesian_product(-10000..10000)
        .filter(|((step, _), y)| yrange.contains(&ypos(*y, *step)))
        .map(|((step, x), y)| (step, (x, y)))
        .unique_by(|(_, coord)| coord.clone())
        .sorted_by_key(|(_, coord)| coord.clone())
        .collect_vec();
    dbg!(results.len());
    dbg!(gauss(results.iter().max_by_key(|f| f.1 .1).unwrap().1 .1));

    dbg!(-yend * ((-yend - 1) / 2));

    Ok(())
}

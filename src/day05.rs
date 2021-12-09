use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

pub(crate) type Coords = (i32, i32);

pub(crate) fn day05(path: &str) -> Result<()> {
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

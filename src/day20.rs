use std::collections::HashMap;

use anyhow::{Context, Result};
use itertools::{fold, Itertools};

fn adjacent((x, y): (isize, isize)) -> Vec<(isize, isize)> {
    (y - 1..=y + 1)
        .cartesian_product(x - 1..=x + 1)
        .map(|(y, x)| (x, y))
        .collect()
}

fn show(map: &HashMap<(isize, isize), bool>) {
    println!();
    let max = map.keys().max().unwrap();
    let min = map.keys().min().unwrap();
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            print!("{}", if map[&(x, y)] { '#' } else { '.' })
        }
        println!();
    }
}
fn step(
    bitrepo: &[bool],
    map: HashMap<(isize, isize), bool>,
    default: bool,
) -> HashMap<(isize, isize), bool> {
    let valid_keys: Vec<(isize, isize)> = map
        .iter()
        .filter(|(_k, &v)| v)
        .map(|p| p.0)
        .copied()
        .collect();

    let (minx, maxx) = valid_keys
        .iter()
        .map(|k| k.0)
        .minmax()
        .into_option()
        .unwrap();

    let (miny, maxy) = valid_keys
        .iter()
        .map(|k| k.1)
        .minmax()
        .into_option()
        .unwrap();
    (minx - 1..=maxx + 1)
        .flat_map(|x| (miny - 1..=maxy + 1).map(move |y| (x, y)))
        .map(|(x, y)| {
            let bit = adjacent((x, y))
                .into_iter()
                .map(|(x, y)| map.get(&(x, y)).copied().unwrap_or(default))
                .fold(0, |acc, elem| acc * 2 + elem as usize);
            //dbg!((x, y), bit, bitrepo[bit]);
            ((x, y), bitrepo[bit])
        })
        .collect()
}

pub fn run(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path)?;
    let (bitrepo, charmap) = file.split_once("\n\n").context("initial split")?;
    let bitrepo = bitrepo.as_bytes().iter().map(|&c| c == b'#').collect_vec();
    let mut charmap = charmap
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(move |(x, &c)| ((x as isize, y as isize), c == b'#'))
        })
        .collect::<HashMap<(isize, isize), _>>();
    let mut default = false;
    for _ in 0..50 {
        charmap = step(&bitrepo, charmap, default);
        default = bitrepo[if default { 511 } else { 0 }]
    }

    let count = charmap.values().filter(|v| **v).count();
    dbg!(count);

    Ok(())
}

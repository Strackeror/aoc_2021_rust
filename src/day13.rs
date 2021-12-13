use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

fn show(set: &HashSet<(i32, i32)>) {
    let max = set
        .iter()
        .copied()
        .reduce(|(a, b), (a2, b2)| (a.max(a2), b.max(b2)))
        .unwrap();

    println!("{} {}", max.0, max.1);
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            print!(
                "{}",
                match set.contains(&(x, y)) {
                    true => "#",
                    false => ".",
                }
            )
        }
        println!()
    }
}

fn step(set: HashSet<(i32, i32)>, inst: &(&str, i32)) -> HashSet<(i32, i32)> {
    set.into_iter()
        .map(|(x, y)| {
            if inst.0 == "x" {
                (inst.1 - (inst.1 - x).abs(), y)
            } else {
                (x, inst.1 - (inst.1 - y).abs())
            }
        })
        .collect()
}

pub fn day13(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path)?;
    let points: HashSet<(i32, i32)> = file
        .lines()
        .take_while(|l| !l.is_empty())
        .filter_map(|l| l.split_once(','))
        .map(|l| (l.0.parse().unwrap(), l.1.parse().unwrap()))
        .collect();

    let instructions: Vec<(&str, i32)> = file
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .filter_map(|l| l.split(' ').nth(2)?.split_once('='))
        .map(|(l, i)| (l, i.parse().unwrap()))
        .collect_vec();

    let point_count = step(points.clone(), &instructions[0]).len();
    dbg!(point_count);
    let final_points = instructions.iter().fold(points, step);
    show(&final_points);

    Ok(())
}

use anyhow::Result;
use itertools::Itertools;

use std::collections::HashMap;

pub(crate) fn _show(map: &HashMap<(i32, i32), i32>) {
    println!();
    for x in 0..10 {
        for y in 0..10 {
            print!("{:2} ", map[&(x, y)])
        }
        println!();
    }
}

pub(crate) fn inc_octopus(map: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) {
    let target_octopus = match map.get_mut(&(x, y)) {
        Some(n) => n,
        None => return,
    };

    *target_octopus += 1;
    if *target_octopus != 10 {
        return;
    }
    (x - 1..=x + 1)
        .cartesian_product(y - 1..=y + 1)
        .filter(|p| *p != (x, y))
        .for_each(|(x, y)| inc_octopus(map, x, y));
}

pub(crate) fn day11(path: &str) -> Result<()> {
    let mut map: HashMap<(i32, i32), i32> = std::fs::read_to_string(path)?
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as _, y as _), c as i32 - '0' as i32))
        })
        .collect();

    let mut flash_count = 0;
    for i in 1..10000 {
        for (x, y) in map.keys().copied().collect_vec() {
            inc_octopus(&mut map, x, y);
        }
        map.values_mut().for_each(|v| {
            if *v > 9 {
                flash_count += 1;
                *v = 0
            }
        });

        if i == 100 {
            // ex1
            dbg!(flash_count);
        }
        if map.values().all(|f| *f == 0) {
            // ex2
            dbg!(i);
            break;
        }
    }

    Ok(())
}

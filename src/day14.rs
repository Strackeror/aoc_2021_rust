use std::{collections::HashMap, ops::AddAssign};

use anyhow::Result;
use itertools::Itertools;

fn id(a: &str, i: usize) -> char {
    a.chars().nth(i).unwrap()
}

pub fn day14(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path)?;
    dbg!(&file);
    let (template, rules) = file.split_once("\n\n").unwrap();
    let rules: HashMap<_, _> = rules
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(a, b)| (id(a, 0), id(a, 1), id(b, 0)))
        .map(|(a, b, c)| ((a, b), ((a, c), (c, b))))
        .collect();
    dbg!(&rules);

    let mut counts: HashMap<(char, char), usize> = template.chars().tuple_windows().counts();
    dbg!(&counts);

    for _ in 0..40 {
        let mut ncounts: HashMap<_, _> = HashMap::new();
        for (pair, count) in &counts {
            ncounts.entry(rules[pair].0).or_insert(0).add_assign(count);
            ncounts.entry(rules[pair].1).or_insert(0).add_assign(count);
        }
        counts = ncounts;
        dbg!(&counts);
    }

    let final_counts = counts
        .iter()
        .map(|((a, _), count)| (*a, *count))
        .chain(Some((template.chars().last().unwrap(), 1)))
        .into_group_map_by(|f| f.0)
        .into_iter()
        .map(|(c, counts)| (c, counts.iter().map(|p| p.1).sum::<usize>()))
        .sorted_by_key(|p| p.1)
        .collect_vec();
    dbg!(final_counts.last().unwrap().1 - final_counts.first().unwrap().1);

    Ok(())
}

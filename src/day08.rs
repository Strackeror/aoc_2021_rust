use std::{
    collections::{HashMap, HashSet},
    vec,
};

use itertools::Itertools;

pub fn day08(path: &str) -> anyhow::Result<()> {
    let lines: Vec<(Vec<String>, Vec<String>)> = std::fs::read_to_string(path)?
        .lines()
        .map(|l| {
            l.split('|')
                .map(|chars| chars.trim().split(' ').map(String::from).collect())
                .next_tuple()
                .unwrap()
        })
        .collect();
    // 1 -> 2 segments, 7 -> 3 segments, 4 -> 4 segments, 8 -> 7 segments
    let count: usize = [2, 3, 4, 7]
        .iter()
        .map(|segment_count| {
            lines
                .iter()
                .map(|segment_list| {
                    segment_list
                        .1
                        .iter()
                        .filter(|segment_desc| segment_desc.len() == *segment_count)
                        .count()
                })
                .sum::<usize>()
        })
        .sum();
    dbg!(count);

    fn map_segments(scrambled_digits: Vec<String>) -> anyhow::Result<HashMap<String, usize>> {
        let normal_digits = vec![
            "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
        ];

        let mut available_segments = "abcdefg"
            .chars()
            .map(|c| (c, String::from("abcdefg")))
            .collect::<HashMap<_, _>>();

        for i in normal_digits.iter().map(|s| s.len()).unique() {
            let real_sets = normal_digits.iter().filter(|s| s.len() == i).collect_vec();
            let scrambled_sets = scrambled_digits
                .iter()
                .filter(|s| s.len() == i)
                .collect_vec();

            let segments = available_segments.keys().copied().collect_vec();
            for segment in segments {
                let segref = available_segments.get_mut(&segment).unwrap();
                *segref = segref.replace(
                    |c| {
                        real_sets.iter().filter(|s| s.contains(segment)).count()
                            != scrambled_sets.iter().filter(|s| s.contains(c)).count()
                    },
                    "",
                )
            }
        }

        if available_segments.iter().any(|f| f.1.len() != 1) {
            panic!("invalid result");
        }

        Ok(normal_digits
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| available_segments[&c].chars().next().unwrap())
                    .sorted()
                    .collect::<String>()
            })
            .enumerate()
            .map(|(num, s)| (s, num))
            .collect())
    }

    let sum: usize = lines
        .into_iter()
        .map(|(map, digits)| {
            let map = map_segments(map).unwrap();
            digits.iter().fold(0, |acc, digit| {
                acc * 10 + map[&digit.chars().sorted().collect::<String>()]
            })
        })
        .sum();
    dbg!(sum);

    Ok(())
}

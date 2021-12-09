use itertools::Itertools;
use std::collections::VecDeque;

use anyhow::Result;

pub(crate) fn day09(path: &str) -> Result<()> {
    let map: Vec<Vec<usize>> = std::fs::read_to_string(path)?
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| (c as usize - '0' as usize) as usize)
                .collect()
        })
        .collect();
    dbg!(&map[0]);

    let get = |map: Vec<Vec<usize>>, x, y| -> Option<usize> {
        Some(*map.get(y as usize)?.get(x as usize)?)
    };
    let adjacent = |map: Vec<Vec<usize>>, x, y| {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter_map(|(yoffs, xoffs)| {
                (
                    (x + xoffs) as usize,
                    (y + yoffs) as usize,
                    get(map.clone(), x + xoffs, y + yoffs)?,
                )
                    .into()
            })
            .collect_vec()
    };

    let low_points: Vec<(usize, usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, height)| (x, y, *height))
                .collect_vec()
        })
        .filter(|(x, y, height)| {
            adjacent(map.clone(), *x as i32, *y as i32)
                .iter()
                .all(|i| i.2 > *height)
        })
        .collect_vec();
    dbg!(&low_points);
    let sum: usize = low_points.iter().map(|f| f.2 + 1).sum();
    dbg!(sum);

    let mut found_map = map.clone();
    let mut sizes = vec![0usize; 0];

    for (x, y, h) in low_points {
        let mut size = 0;
        let mut queue = VecDeque::from([(x, y); 1]);
        println!("LOW POINT {:?}", (x, y, h));

        found_map[y][x] = 9;
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            size += 1;
            for (x, y, h) in adjacent(found_map.clone(), x as _, y as _)
                .iter()
                .filter(|t| t.2 < 9)
            {
                found_map[*y][*x] = 9;
                dbg!((x, y, h));
                queue.push_back((*x, *y));
            }
        }
        sizes.push(size)
    }

    dbg!(&sizes);
    let sum: usize = sizes.iter().sorted().rev().take(3).product();
    dbg!(sum);

    Ok(())
}

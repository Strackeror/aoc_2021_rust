use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

type Coords = (i32, i32);

fn _show(map: &HashMap<(i32, i32), usize>) {
    println!();
    let max = map.keys().max().unwrap();
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            print!("{:2} ", map[&(x, y)])
        }
        println!();
    }
}

fn lowest(map: &HashMap<Coords, usize>, (x, y): Coords, target: Coords) -> usize {
    let mut queue = VecDeque::from([(x, y)]);
    let mut distmap = HashMap::from([((x, y), 0)]);
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter(|p| map.contains_key(p))
        {
            let new_val = distmap[&(x, y)] + map[&(nx, ny)];
            let valref = distmap.entry((nx, ny)).or_insert(usize::MAX);
            if new_val < *valref {
                *valref = new_val;
                if (nx, ny) != target {
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    distmap[&target]
}

pub fn run(path: &str) -> anyhow::Result<()> {
    let map = std::fs::read_to_string(path)?;
    let map: HashMap<Coords, usize> = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as _, y as _), c as usize - '0' as usize))
        })
        .collect();

    let result = lowest(&map, (0, 0), *map.keys().max().unwrap());
    dbg!(result);
    _show(&map);

    let (xsize, ysize) = *map.keys().max().unwrap();
    let xsize = xsize + 1;
    let ysize = ysize + 1;

    let big_map = map
        .iter()
        .flat_map(|((x, y), value)| {
            (0..5).cartesian_product(0..5).map(|(xrepeat, yrepeat)| {
                (
                    (*x + xrepeat * xsize, *y + yrepeat * ysize),
                    (*value + (xrepeat + yrepeat) as usize - 1) % 9 + 1,
                )
            })
        })
        .collect::<HashMap<_, _>>();

    _show(&big_map);
    let result = lowest(&big_map, (0, 0), *big_map.keys().max().unwrap());
    dbg!(result);

    Ok(())
}

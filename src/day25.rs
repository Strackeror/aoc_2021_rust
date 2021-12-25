use std::collections::HashMap;

use anyhow::Result;

fn run(path: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)?;

    let mut map = content
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(move |(x, &b)| ((x as isize, y as isize), b))
        })
        .collect::<HashMap<_, _>>();

    for i in 0.. {
        let mut hmap = map.clone();
        for (&(x, y), _) in map.iter().filter(|&p| *p.1 == b'>') {
            let ncell = if map.get(&(x + 1, y)).is_some() {
                (x + 1, y)
            } else {
                (0, y)
            };

            if map.get(&ncell).copied() == Some(b'.') {
                hmap.insert((x, y), b'.');
                hmap.insert(ncell, b'>');
            }
        }

        let mut vmap = hmap.clone();
        for (&(x, y), _) in hmap.iter().filter(|&p| *p.1 == b'v') {
            let ncell = if map.get(&(x, y + 1)).is_some() {
                (x, y + 1)
            } else {
                (x, 0)
            };
            if hmap.get(&ncell).copied() == Some(b'.') {
                vmap.insert((x, y), b'.');
                vmap.insert(ncell, b'v');
            }
        }
        if map == vmap {
            dbg!(i);
            break;
        }
        map = vmap;
    }

    Ok(())
}

#[test]
fn example() {
    run("input/day25/example.txt");
}
#[test]
fn input() {
    run("input/day25/input.txt");
}

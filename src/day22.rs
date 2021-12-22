use std::{
    collections::{HashMap, HashSet},
    vec,
};

use anyhow::Result;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
struct Volume {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Volume {
    fn get(self, axis: usize) -> Option<(i32, i32)> {
        Some(match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => return None,
        })
    }

    fn cut_axis(self, axis: usize, at: i32) -> (Volume, Volume) {
        let mut before = [self.x, self.y, self.z];
        let mut after = [self.x, self.y, self.z];

        if at > before[axis].0 && at < before[axis].1 {
            before[axis].1 = at;
            after[axis].0 = at;

            (
                Volume {
                    x: before[0],
                    y: before[1],
                    z: before[2],
                },
                Volume {
                    x: after[0],
                    y: after[1],
                    z: after[2],
                },
            )
        } else if at <= before[axis].0 {
            (Default::default(), self)
        } else {
            (self, Default::default())
        }
    }

    fn cut(self, cutter: Volume) -> Vec<Volume> {
        let mut inside = vec![self];
        let mut outside = vec![];

        for axis in 0..3 {
            let cut = cutter.get(axis).unwrap();
            let (n_inside, n_outside) =
                inside
                    .into_iter()
                    .fold((vec![], outside), |(mut inside, mut outside), elem| {
                        let (before, after) = elem.cut_axis(axis, cut.0);
                        if before.count() > 0 {
                            outside.push(before);
                        }
                        if after.count() > 0 {
                            let (before, after) = after.cut_axis(axis, cut.1);
                            if after.count() > 0 {
                                outside.push(after);
                            }
                            if before.count() > 0 {
                                inside.push(before);
                            }
                        }
                        (inside, outside)
                    });
            inside = n_inside;
            outside = n_outside;
        }

        outside
    }

    fn count(self) -> i64 {
        (self.x.1 - self.x.0).abs() as i64
            * (self.y.1 - self.y.0).abs() as i64
            * (self.z.1 - self.z.0).abs() as i64
    }
}

fn parse(line: &str) -> Result<(bool, Volume)> {
    let (on, x1, x2, y1, y2, z1, z2) = scan_fmt!(
        line,
        "{} x={}..{},y={}..{},z={}..{}",
        String,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32
    )?;
    let on = match on.as_str() {
        "on" => true,
        "off" => false,
        _ => return Err(anyhow::anyhow!("unexpected initial string {}", on)),
    };

    Ok((
        on,
        Volume {
            x: (x1, x2 + 1),
            y: (y1, y2 + 1),
            z: (z1, z2 + 1),
        },
    ))
}

pub fn run(path: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)?;
    let steps: Vec<_> = content.lines().map(parse).try_collect()?;

    let init = steps[0];
    let final_volumes = steps[1..].iter().fold(vec![init.1], |acc, &(on, volume)| {
        let mut next = acc.into_iter().flat_map(|v| v.cut(volume)).collect_vec();
        if on {
            next.push(volume);
        }
        let count = next.iter().fold(0, |acc, v| acc + v.count());
        dbg!(count);
        next
    });
    let count = final_volumes.iter().fold(0, |acc, v| acc + v.count());
    dbg!(count);

    Ok(())
}

#[test]
fn example1() {
    dbg!(run("input/day22/example1.txt"));
}

#[test]
fn example2() {
    dbg!(run("input/day22/example2.txt"));
}

#[test]
fn input() {
    dbg!(run("input/day22/input.txt"));
}

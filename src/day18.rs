use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Pair {
    Number(u8),
    Pair(Box<Pair>, Box<Pair>),
}

impl Pair {
    fn npair(a: Pair, b: Pair) -> Pair {
        Pair::Pair(Box::new(a), Box::new(b))
    }
}

fn read_char(reader: &mut &str) -> u8 {
    let (next, nreader) = reader.split_at(1);
    *reader = nreader;
    next.as_bytes()[0]
}

fn parse(reader: &mut &str) -> Result<Pair> {
    let next = read_char(reader);
    if next.is_ascii_digit() {
        Ok(Pair::Number(next - b'0'))
    } else if next == b'[' {
        let pair1 = parse(reader)?;
        if read_char(reader) != b',' {
            return Err(anyhow!("expected comma"));
        }
        let pair2 = parse(reader)?;
        if read_char(reader) != b']' {
            return Err(anyhow!("expected closing bracket"));
        }
        Ok(Pair::npair(pair1, pair2))
    } else {
        Err(anyhow!("unexpected char: {}", next))
    }
}

fn add_left(pair: Pair, val: Option<u8>) -> Pair {
    match val {
        None => pair,
        Some(v) => match pair {
            Pair::Number(n) => Pair::Number(n + v),
            Pair::Pair(a, b) => Pair::npair(add_left(*a, val), *b),
        },
    }
}

fn add_right(pair: Pair, val: Option<u8>) -> Pair {
    match val {
        None => pair,
        Some(v) => match pair {
            Pair::Number(n) => Pair::Number(n + v),
            Pair::Pair(a, b) => Pair::npair(*a, add_right(*b, val)),
        },
    }
}

fn explode(pair: Pair, depth: u8) -> (Pair, Option<u8>, Option<u8>, bool) {
    match pair {
        Pair::Number(_) => (pair, None, None, false),
        Pair::Pair(a, b) => {
            if depth >= 4 {
                match (*a, *b) {
                    (Pair::Number(a), Pair::Number(b)) => {
                        return (Pair::Number(0), Some(a), Some(b), true)
                    }
                    _ => unreachable!(),
                }
            }
            let (sub_pair_left, left_ret, left_add, exploded) = explode(*a, depth + 1);
            if exploded {
                (
                    Pair::npair(sub_pair_left, add_left(*b, left_add)),
                    left_ret,
                    None,
                    true,
                )
            } else {
                let (sub_pair_right, right_add, right_ret, exploded) = explode(*b, depth + 1);
                (
                    Pair::npair(add_right(sub_pair_left, right_add), sub_pair_right),
                    None,
                    right_ret,
                    exploded,
                )
            }
        }
    }
}

fn split(pair: Pair) -> (Pair, bool) {
    match pair {
        Pair::Number(n) => {
            if n >= 10 {
                (
                    Pair::npair(Pair::Number(n / 2), Pair::Number(n / 2 + n % 2)),
                    true,
                )
            } else {
                (pair, false)
            }
        }

        Pair::Pair(a, b) => {
            let (new_a, has_split) = split(*a);
            if has_split {
                (Pair::npair(new_a, *b), true)
            } else {
                let (new_b, has_split) = split(*b);
                (Pair::npair(new_a, new_b), has_split)
            }
        }
    }
}

fn reduce(mut pair: Pair) -> Pair {
    loop {
        let (npair, _, _, exploded) = explode(pair, 0);
        pair = npair;
        if exploded {
            continue;
        }

        let (npair, has_split) = split(pair);
        pair = npair;
        if has_split {
            continue;
        }
        break;
    }
    pair
}

fn magnitude(pair: &Pair) -> u64 {
    match pair {
        Pair::Number(n) => *n as _,
        Pair::Pair(a, b) => 3 * magnitude(a) + 2 * magnitude(b),
    }
}

pub fn run(path: &str) -> Result<()> {
    let input = std::fs::read_to_string(path)?;
    let numbers = input
        .lines()
        .filter_map(|mut line| dbg!(parse(&mut line)).ok())
        .collect_vec();

    let part1 = numbers
        .clone()
        .into_iter()
        .reduce(|acc, elem| reduce(Pair::npair(acc, elem)))
        .context("empty")?;
    dbg!(&part1, magnitude(&part1));

    let part2 = numbers
        .into_iter()
        .permutations(2)
        .map(|perm| reduce(Pair::npair(perm[0].clone(), perm[1].clone())))
        .max_by_key(magnitude)
        .context("part2 not found")?;
    dbg!(&part2, magnitude(&part2));

    Ok(())
}

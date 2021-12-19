use itertools::Itertools;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

mod day19_old;

mod day19 {
    use std::{
        collections::{BTreeMap, HashMap, HashSet, VecDeque},
        ops::{Neg, Not, Sub},
    };

    use anyhow::Result;
    use itertools::Itertools;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Vector(i32, i32, i32);

    impl Sub for Vector {
        type Output = Vector;

        fn sub(self, rhs: Self) -> Self::Output {
            self + -rhs
        }
    }
    impl Neg for Vector {
        type Output = Vector;

        fn neg(self) -> Self::Output {
            Vector(-self.0, -self.1, -self.2)
        }
    }

    impl std::ops::Add for Vector {
        type Output = Vector;

        fn add(self, rhs: Self) -> Self::Output {
            Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
        }
    }

    fn rotate(Vector(x, y, z): Vector, rot: i32) -> Vector {
        match rot {
            0 => Vector(x, y, z),
            1 => Vector(x, z, -y),
            2 => Vector(x, -y, -z),
            3 => Vector(x, -z, y),
            4 => Vector(y, x, -z),
            5 => Vector(y, z, x),
            6 => Vector(y, -x, z),
            7 => Vector(y, -z, -x),
            8 => Vector(z, x, y),
            9 => Vector(z, y, -x),
            10 => Vector(z, -x, -y),
            11 => Vector(z, -y, x),
            12 => Vector(-x, y, -z),
            13 => Vector(-x, z, y),
            14 => Vector(-x, -y, z),
            15 => Vector(-x, -z, -y),
            16 => Vector(-y, x, z),
            17 => Vector(-y, z, -x),
            18 => Vector(-y, -x, -z),
            19 => Vector(-y, -z, x),
            20 => Vector(-z, x, -y),
            21 => Vector(-z, y, x),
            22 => Vector(-z, -x, y),
            23 => Vector(-z, -y, -x),
            _ => unreachable!(),
        }
    }

    fn match_coord(a: &[Vector], b: &[Vector]) -> (Vector, Vec<Vector>) {
        for rid in 0..24 {
            let b_rotated = b.iter().map(|v| rotate(*v, rid)).collect_vec();

            for (&apos, &bpos) in a.iter().cartesian_product(b_rotated.iter()) {
                let diff = bpos - apos;
                let unique_count = b_rotated
                    .iter()
                    .map(|v| *v - diff)
                    .chain(a.iter().copied())
                    .collect::<HashSet<_>>()
                    .len();
                if a.len() + b.len() - unique_count >= 12 {
                    dbg!(diff);
                    return (
                        -diff,
                        b_rotated.iter().copied().map(|v| v - diff).collect_vec(),
                    );
                }
            }
        }
        (Vector(0, 0, 0), vec![])
    }

    pub fn run(path: &str) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        let scanner_list: Vec<Vec<Vector>> = content
            .split("\n\n")
            .map(|section| {
                section
                    .lines()
                    .filter(|line| !line.starts_with("---"))
                    .map(|coord_string| {
                        coord_string
                            .split(',')
                            .map(|s| str::parse(s).unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .map(|(a, b, c)| Vector(a, b, c))
                    .collect_vec()
            })
            .collect_vec();

        let mut not_visited = (1..scanner_list.len()).collect::<HashSet<_>>();
        let mut queue = VecDeque::from([scanner_list[0].clone()]);
        let mut beacons = vec![scanner_list[0].clone()];
        let mut scanners = vec![Vector(0, 0, 0)];
        while queue.is_empty().not() {
            let next = queue.pop_front().unwrap();
            dbg!("pop");

            for &i in not_visited.clone().iter() {
                dbg!(("check", i));
                let (pos, matched) = match_coord(&next, &scanner_list[i]);
                dbg!(matched.len());
                if !matched.is_empty() {
                    dbg!(("push", i));
                    beacons.push(matched.clone());
                    queue.push_back(matched);
                    not_visited.remove(&i);
                    scanners.push(pos)
                }
            }
        }

        let matched_beacons = beacons.iter().flatten().copied().collect::<HashSet<_>>();
        dbg!(&matched_beacons, matched_beacons.len());
        dbg!(not_visited);

        let scanner_distances = scanners.iter().tuple_combinations().map(|(&a, &b)| b - a);
        let max_distance = scanner_distances
            .map(|Vector(x, y, z)| x.abs() + y.abs() + z.abs())
            .max();
        dbg!(max_distance);
        Ok(())
    }
}
fn main() {
    let args = std::env::args().collect_vec();
    let input = args[1].clone();

    match args.get(2) {
        Some(n) => match n.as_str() {
            "day01" => day01::day01(&input),
            "day02" => day02::day02(&input),
            "day03" => day03::day03(&input),
            "day03_bin" => day03::day03_bin(&input),
            "day04" => day04::day04(&input),
            "day05" => day05::day05(&input),
            "day06" => day06::day06(&input),
            "day07" => day07::day07(&input),
            "day08" => day08::day08(&input),
            "day09" => day09::day09(&input),
            "day10" => day10::day10(&input),
            "day11" => day11::day11(&input),
            "day12" => day12::day12(&input),
            "day13" => day13::day13(&input),
            "day14" => day14::day14(&input),
            "day15" => day15::run(&input),
            "day16" => day16::run(&input),
            "day17" => day17::run(&input),
            "day18" => day18::run(&input),
            _ => panic!("unexpected arg"),
        },
        None => day19::run(&input),
    }
    .unwrap();
}

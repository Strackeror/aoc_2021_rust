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

    fn all_rotations() -> Vec<((usize, usize, usize), (i32, i32, i32))> {
        [0, 1, 2]
            .into_iter()
            .permutations(3)
            .map(|v| v.into_iter().collect_tuple().unwrap())
            .cartesian_product(
                [-1, 1]
                    .into_iter()
                    .cartesian_product([-1, 1].into_iter())
                    .cartesian_product([-1, 1].into_iter())
                    .map(|((a, b), c)| (a, b, c)),
            )
            .collect()
    }

    fn apply_rotation(v: Vector, rot: ((usize, usize, usize), (i32, i32, i32))) -> Vector {
        let v = [v.0, v.1, v.2];

        Vector(
            v[rot.0 .0] * rot.1 .0,
            v[rot.0 .1] * rot.1 .1,
            v[rot.0 .2] * rot.1 .2,
        )
    }

    fn diff_list(points: &[Vector]) -> Vec<(usize, usize, Vector)> {
        points
            .iter()
            .copied()
            .enumerate()
            .tuple_combinations::<(_, _)>()
            .map(|((idx_a, point_a), (idx_b, point_b))| (idx_a, idx_b, point_b - point_a))
            .flat_map(|(a, b, diff)| vec![(a, b, diff), (b, a, -diff)])
            .collect_vec()
    }

    fn match_coord(a: &[Vector], b: &[Vector]) -> Vec<Vector> {
        let a_diffs = diff_list(a);

        let mut b_rotated = Vec::new();
        let mut matching_diffs = Vec::new();
        for rotation in all_rotations() {
            b_rotated = b.iter().map(|v| apply_rotation(*v, rotation)).collect_vec();
            let n_matching_diffs = diff_list(&b_rotated)
                .into_iter()
                .filter(|&(_, _, diff)| a_diffs.iter().any(|&(_, _, a_diff)| a_diff == diff))
                .collect_vec();
            if n_matching_diffs.len() >= 2 {
                matching_diffs = n_matching_diffs;
                break;
            }
        }

        let map = matching_diffs
            .iter()
            .map(|&(b1, b2, diff)| {
                (
                    b1,
                    a_diffs
                        .iter()
                        .filter(|&&(_, _, adiff)| adiff == diff)
                        .collect_vec(),
                )
            })
            .filter(|(_, found)| found.iter().all(|&&d| d.0 == found[0].0))
            .map(|(a, b)| (a, b[0].0))
            .collect::<HashMap<_, _>>();

        dbg!(&map);
        if map.len() < 12 {
            return vec![];
        }

        let first = map.keys().next().unwrap();
        let diff = b_rotated[*first] - a[map[first]];
        b_rotated.into_iter().map(|v| v - diff).collect()
    }

    pub fn run(path: &str) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        let coords: Vec<Vec<Vector>> = content
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

        let mut not_visited = (1..coords.len()).collect::<HashSet<_>>();
        let mut queue = VecDeque::from([coords[0].clone()]);
        let mut beacons = vec![coords[0].clone()];
        while queue.is_empty().not() {
            let next = queue.pop_front().unwrap();
            dbg!("pop");

            for &i in not_visited.clone().iter() {
                dbg!(("check", i));
                let matched = match_coord(&next, &coords[i]);
                dbg!(matched.len());
                if !matched.is_empty() {
                    dbg!(("push", i));
                    beacons.push(matched.clone());
                    queue.push_back(matched);
                    not_visited.remove(&i);
                }
            }
        }

        let matched_beacons = beacons.iter().flatten().copied().collect::<HashSet<_>>();
        dbg!(&matched_beacons, matched_beacons.len());
        dbg!(not_visited);

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

// use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

// use anyhow::Result;
// use itertools::Itertools;

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// struct Point(i32, i32, i32);

// type PointDiff = ((usize, usize), i32);
// fn distance_list(points: &[Point]) -> Vec<PointDiff> {
//     points
//         .iter()
//         .copied()
//         .enumerate()
//         .tuple_combinations::<(_, _)>()
//         .map(|((idx_a, point_a), (idx_b, point_b))| ((idx_a, idx_b), point_diff(point_a, point_b)))
//         .collect_vec()
// }

// fn common_points(a: &[Point], b: &[Point]) -> Vec<Point> {
//     let a = distance_list(a);
//     let b = distance_list(b);
//     let common_diffs /* :Vec<(PointDiff, PointDiff)>*/ = a
//         .iter()
//         .map(|&diff| b.iter().filter(|&&bdiff| bdiff.1 == diff.1).copied().collect_vec())
//         .zip(a.iter())
//         .filter(|p| !p.0.is_empty())
//         .map(|(a, b)| (*b, a))
//         .collect_vec();
//     dbg!(&common_diffs[0..5]);

//     let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
//     for (a, b) in common_diffs {
//         for a_idx in [a.0 .0, a.0 .1] {
//             let b_ids = map.entry(a_idx).or_insert_with(|| b.iter().flat_map(f));
//             *b_ids = b_ids
//                 .clone()
//                 .into_iter()
//                 .filter(|&b_idx| b_idx == b.0 .0 || b_idx == b.0 .1)
//                 .collect_vec();
//         }
//     }

//     vec![]
// }

// pub fn run(path: &str) -> Result<()> {
//     let content = std::fs::read_to_string(path)?;
//     let coords: Vec<Vec<Point>> = content
//         .split("\n\n")
//         .map(|section| {
//             section
//                 .lines()
//                 .filter(|line| !line.starts_with("---"))
//                 .map(|coord_string| {
//                     coord_string
//                         .split(',')
//                         .map(|s| str::parse(s).unwrap())
//                         .collect_tuple()
//                         .unwrap()
//                 })
//                 .map(|(a, b, c)| Point(a, b, c))
//                 .collect_vec()
//         })
//         .collect_vec();
//     common_points(&coords[0], &coords[1]);

//     // let mut common_point_list: Vec<((usize, usize), (usize, usize))> = coords[0..=1]
//     //     .iter()
//     //     .enumerate()
//     //     .tuple_combinations()
//     //     .flat_map(|((rid_1, relative_1), (rid_2, relative_2))| {
//     //         common_points(relative_1, relative_2)
//     //             .iter()
//     //             .map(|(a, b)| ((rid_1, *a), (rid_2, *b)))
//     //             .collect::<Vec<_>>()
//     //     })
//     //     .collect_vec();
//     // dbg!(common_point_list);

//     // //     dbg!(&common_point_list);
//     // //     let mut beacons = Vec::new();
//     // //     while !common_point_list.is_empty() {
//     // //         let mut queue = VecDeque::from([(0, 12)]);
//     // //         let mut beacon = BTreeSet::<(usize, usize)>::new();
//     // //         while !queue.is_empty() {
//     // //             let next = queue.pop_front().unwrap();
//     // //             dbg!(next);
//     // //             let (linked, rest): (Vec<_>, Vec<_>) = common_point_list
//     // //                 .iter()
//     // //                 .partition(|(a, b)| *a == next || *b == next);
//     // //             dbg!(&linked);
//     // //             common_point_list = rest;
//     // //             linked.into_iter().for_each(|(a, b)| {
//     // //                 queue.push_back(a);
//     // //                 queue.push_back(b);
//     // //                 beacon.insert(a);
//     // //                 beacon.insert(b);
//     // //             });
//     // //         }
//     // //         dbg!("beacon found", &beacon);
//     // //         beacons.push(beacon);
//     // //         common_point_list.clear();
//     // //     }

//     // //     // for cidx in 0..coords.len() {
//     // //     //     for bidx in 0..coords[cidx].len() {
//     // //     //         if !beacons.iter().any(|set| set.contains(&(cidx, bidx))) {
//     // //     //             beacons.push(HashSet::from([(cidx, bidx)]));
//     // //     //         }
//     // //     //     }
//     // //     // }
//     // //     //dbg!(&beacons, beacons.len());

//     // //     dbg!(point_diff(coords[0][12], coords[1][3]));
//     // //     dbg!(point_diff(coords[1][3], coords[3][11]));

//     Ok(())
// }

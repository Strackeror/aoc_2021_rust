use std::clone::Clone;

use anyhow::Result;

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
struct Node {
    small: bool,
    links: Vec<usize>,
}

fn visit_node(
    current: usize,
    target: usize,
    map: &[Node],
    mut visited: Vec<usize>,
    small_cave: bool,
) -> Vec<Vec<usize>> {
    visited.push(current);
    if current == target {
        return [visited].into();
    }

    if visited.len() > 1 && visited[0] == current {
        return vec![];
    }

    let is_visited = map[current].small && visited.iter().filter(|n| **n == current).count() > 1;
    if is_visited && small_cave {
        return vec![];
    }

    map[current]
        .links
        .iter()
        .flat_map(|id| visit_node(*id, target, map, visited.clone(), small_cave || is_visited))
        .collect()
}

pub(crate) fn day12(path: &str) -> Result<()> {
    let links: Vec<(_, _)> = std::fs::read_to_string(path)?
        .lines()
        .map(|l| l.split('-').next_tuple::<(_, _)>().unwrap())
        .map(|(a, b)| (String::from(a), String::from(b)))
        .collect();
    let inverted_links = links.iter().map(|(a, b)| (b.clone(), a.clone()));

    let link_map: HashMap<_, _> = links
        .iter()
        .map(Clone::clone)
        .chain(inverted_links)
        .into_group_map_by(|f| f.0.clone())
        .into_iter()
        .enumerate()
        .map(|(id, (name, links))| (name, (id, links)))
        .collect();

    let id_map: Vec<_> = link_map
        .iter()
        .sorted_by_key(|(_, (id, _))| id)
        .map(|(name, (_, links))| Node {
            small: name.chars().all(char::is_lowercase),
            links: links.iter().map(|link| link_map[&link.1].0).collect(),
        })
        .collect();
    let start_node = link_map["start"].0;
    let end_node = link_map["end"].0;
    let ex1 = visit_node(start_node, end_node, &id_map, vec![], true);
    dbg!(ex1.len());

    let ex2 = visit_node(start_node, end_node, &id_map, vec![], false);
    dbg!(ex2.len());
    // // ex2.iter().for_each(|n| {
    // //     dbg!(show(n));
    // // });

    Ok(())
}

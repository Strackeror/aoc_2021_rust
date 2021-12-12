use std::clone::Clone;

use anyhow::Result;

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
struct Node {
    name: String,
    small: bool,
    links: Vec<String>,
}

fn visit_node(
    current: Node,
    map: &HashMap<String, Node>,
    mut visited: Vec<Node>,
    small_cave: bool,
) -> Vec<Vec<Node>> {
    visited.push(current.clone());
    if current.name == "end" {
        return [visited].into();
    }

    current
        .links
        .iter()
        .map(|name| map[name].clone())
        .flat_map(|node| {
            if node.name == "start" {
                vec![]
            } else if !current.small {
                visit_node(node, map, visited.clone(), small_cave)
            } else {
                let already_visited = visited.iter().filter(|n| n.name == current.name).count() > 1;
                if already_visited && small_cave {
                    vec![]
                } else {
                    visit_node(node, map, visited.clone(), small_cave || already_visited)
                }
            }
        })
        .collect_vec()
}

fn show(list: &Vec<Node>) -> String {
    list.iter().map(|n| n.name.as_str()).join(",")
}

pub(crate) fn day12(path: &str) -> Result<()> {
    let links: Vec<(_, _)> = std::fs::read_to_string(path)?
        .lines()
        .map(|l| l.split('-').next_tuple::<(_, _)>().unwrap())
        .map(|(a, b)| (String::from(a), String::from(b)))
        .collect();
    let inverted_links = links.iter().map(|(a, b)| (b.clone(), a.clone()));

    let map: HashMap<String, Node> = links
        .iter()
        .map(Clone::clone)
        .chain(inverted_links)
        .into_group_map_by(|f| f.0.clone())
        .into_iter()
        .map(|(name, links)| {
            (
                name.clone(),
                Node {
                    small: name.chars().all(char::is_lowercase),
                    links: links.iter().map(|f| f.1.clone()).collect(),
                    name,
                },
            )
        })
        .collect();
    let ex1 = visit_node(map["start"].clone(), &map, vec![], true);
    dbg!(ex1.len());

    let ex2 = visit_node(map["start"].clone(), &map, vec![], false);
    // ex2.iter().for_each(|n| {
    //     dbg!(show(n));
    // });
    dbg!(ex2.len());

    Ok(())
}

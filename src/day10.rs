use anyhow::Result;
use itertools::Itertools;

pub(crate) fn day10(path: &str) -> Result<()> {
    let lines = std::fs::read_to_string(path)?
        .lines()
        .map(String::from)
        .collect_vec();

    let pairs = [
        ('(', ')', 3, 1),
        ('[', ']', 57, 2),
        ('{', '}', 1197, 3),
        ('<', '>', 25137, 4),
    ];

    let illegals = lines
        .iter()
        .map(|l| {
            l.chars()
                .fold((vec![], vec![]), |(mut stack, mut illegal), c| {
                    if let Some(pair) = pairs.iter().find(|p| p.0 == c) {
                        stack.push(pair);
                    } else {
                        let top = stack.pop().unwrap();
                        if top.1 != c {
                            illegal.push(pairs.iter().find(|p| p.1 == c).unwrap());
                        }
                    }
                    (stack, illegal)
                })
        })
        .collect_vec();
    dbg!(&illegals);

    let ex1 = illegals
        .iter()
        .filter(|v| !v.1.is_empty())
        .map(|v| v.1[0].2)
        .sum::<usize>();
    dbg!(ex1);

    let ex2 = illegals
        .iter()
        .filter(|v| v.1.is_empty())
        .map(|i| i.0.iter().rev().fold(0i64, |acc, x| acc * 5 + x.3))
        .sorted()
        .collect_vec();

    dbg!(ex2[ex2.len() / 2]);

    Ok(())
}

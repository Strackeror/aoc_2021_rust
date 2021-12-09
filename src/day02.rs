use anyhow::Result;
use itertools::Itertools;

pub(crate) fn day02(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path)?;
    let instructions = file
        .lines()
        .map(|l| l.split(' ').collect_vec())
        .map(|vec| (vec[0], vec[1].parse::<i64>().unwrap()));

    let (x, y, aim) = instructions.fold((0, 0, 0), |(x, y, aim), inst| match inst {
        ("forward", n) => (x + n, y + n * aim, aim),
        ("down", n) => (x, y, aim + n),
        ("up", n) => (x, y, aim - n),
        inst => panic!("invalid line {:?}", inst),
    });

    println!("ex1 position:{:?} ex1 result:{:?}", (x, aim), x * aim); // aim in ex2 is just depth in ex1
    println!("ex2 position:{:?} ex2 result:{:?}", (x, y), x * y);

    Ok(())
}

use anyhow::{Context, Result};

pub fn run(input: &str) -> Result<()> {
    let (a, b) = input.split_once(',').context("")?;

    let mut p1_state: (i32, i32) = (a.parse()?, 0);
    let mut p2_state: (i32, i32) = (b.parse()?, 0);
    p1_state.0 -= 1;
    p2_state.0 -= 1;

    let mut die_val = 0;

    'main: loop {
        for target in [&mut p1_state, &mut p2_state] {
            for _ in 0..3 {
                target.0 = (target.0 + die_val % 100 + 1) % 10;
                die_val += 1;
            }
            dbg!(target.0, target.1);
            target.1 += target.0 + 1;
            if target.1 >= 1000 {
                break 'main;
            }
        }
    }
    dbg!(p1_state, p2_state);
    dbg!(p1_state.1.min(p2_state.1) * die_val);

    Ok(())
}

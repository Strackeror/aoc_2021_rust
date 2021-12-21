use std::collections::HashMap;

use anyhow::{Context, Result};
use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct PlayerState {
    position: u64,
    score: u64,
    rollcount: u64,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct GameState {
    p1_state: PlayerState,
    p2_state: PlayerState,
}

fn step(mut state: PlayerState, roll: u64) -> PlayerState {
    state.position = (state.position + roll) % 10;
    state.rollcount += 1;

    if state.rollcount == 3 {
        state.score += state.position + 1;
        state.rollcount = 0;
    }

    state
}

fn game_step(state: GameState, roll: u64, player: u64) -> GameState {
    match player {
        1 => GameState {
            p1_state: step(state.p1_state, roll),
            ..state
        },
        2 => GameState {
            p2_state: step(state.p2_state, roll),
            ..state
        },
        _ => unreachable!(),
    }
}

fn step_probabilites(
    probabilities: HashMap<GameState, u64>,
    player: u64,
) -> HashMap<GameState, u64> {
    probabilities
        .into_iter()
        .flat_map(|(state, count)| (1..=3).map(move |roll| (game_step(state, roll, player), count)))
        .into_grouping_map_by(|t| t.0)
        .fold(0, |acc, _key, val| acc + val.1)
}

pub fn run(input: &str) -> Result<()> {
    let (a, b) = input.split_once(',').context("")?;

    let p1_state = PlayerState {
        position: a.parse::<u64>()? - 1,
        score: 0,
        rollcount: 0,
    };

    let p2_state = PlayerState {
        position: b.parse::<u64>()? - 1,
        score: 0,
        rollcount: 0,
    };

    let mut p1_won = 0;
    let mut p2_won = 0;
    let mut states = HashMap::from([(GameState { p1_state, p2_state }, 1)]);
    while !states.is_empty() {
        for player in 1..=2 {
            for _ in 0..3 {
                states = step_probabilites(states, player);
            }

            let (done, ongoing): (Vec<_>, Vec<_>) =
                states.into_iter().partition(|&(state, _count)| {
                    state.p1_state.score >= 21 || state.p2_state.score >= 21
                });
            states = ongoing.into_iter().collect();
            p1_won += done.iter().fold(0, |acc, (state, count)| {
                acc + if state.p1_state.score >= 21 {
                    *count
                } else {
                    0
                }
            });
            p2_won += done.iter().fold(0, |acc, (state, count)| {
                acc + if state.p2_state.score >= 21 {
                    *count
                } else {
                    0
                }
            });
        }
    }
    dbg!(p1_won, p2_won);

    Ok(())
}

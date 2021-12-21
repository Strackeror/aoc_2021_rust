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
    states: [PlayerState; 2],
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

fn game_step(mut state: GameState, roll: u64, player: usize) -> GameState {
    state.states[player] = step(state.states[player], roll);
    state
}

fn step_probabilites(
    probabilities: HashMap<GameState, u64>,
    player: usize,
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

    let mut states = HashMap::from([(
        GameState {
            states: [p1_state, p2_state],
        },
        1,
    )]);
    let mut won = [0u64; 2];
    while !states.is_empty() {
        for player in 0..2 {
            for _ in 0..3 {
                states = step_probabilites(states, player);
            }

            let (done, ongoing): (Vec<_>, Vec<_>) = states
                .into_iter()
                .partition(|&(state, _count)| state.states[player].score >= 21);
            states = ongoing.into_iter().collect();
            won[player] += done.into_iter().fold(0, |acc, (_state, count)| acc + count);
        }
    }
    dbg!(won);

    Ok(())
}

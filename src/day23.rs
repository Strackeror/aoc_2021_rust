use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use anyhow::Result;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct State {
    hallway: [i8; 11],
    rooms: [[i8; 2]; 4],
}

const A: i8 = 0;
const B: i8 = 1;
const C: i8 = 2;
const D: i8 = 3;

impl State {
    fn show(&self) {
        let charmap = HashMap::from([(-1, '.'), (0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);
        println!("#############");
        print!("#");
        for i in self.hallway {
            print!("{}", charmap[&i])
        }
        println!("#");

        print!("###");
        for i in self.rooms {
            print!("{}#", charmap[&i[0]])
        }
        println!("##");

        print!("  #");
        for i in self.rooms {
            print!("{}#", charmap[&i[1]]);
        }
        println!();
        println!("  #########   ");

        println!()
    }

    fn possible_states(self) -> Vec<(State, i32)> {
        let mut possible = vec![];
        // room to hallway
        for room in 0..4 {
            if self.rooms[room] == [-1, room as i8]
                || self.rooms[room] == [room as i8, room as i8]
                || self.rooms[room] == [-1, -1]
            {
                continue;
            }
            let room_pos = 2 + room * 2;
            let depth = if self.rooms[room][0] >= 0 { 0 } else { 1 };
            let pod = self.rooms[room][depth];
            for target in (0..room_pos).rev() {
                if let 2 | 4 | 6 | 8 = target {
                    continue;
                }

                if self.hallway[target] >= 0 {
                    break;
                }

                let mut hallway = self.hallway;
                hallway[target] = pod;
                let mut rooms = self.rooms;
                rooms[room][depth] = -1;
                let distance = (room_pos - target) + depth + 1;
                possible.push((
                    State { hallway, rooms },
                    distance as i32 * 10i32.pow(pod as u32),
                ))
            }

            for target in room_pos + 1..11 {
                if let 2 | 4 | 6 | 8 = target {
                    continue;
                }

                if self.hallway[target] >= 0 {
                    break;
                }

                let mut hallway = self.hallway;
                hallway[target] = pod;
                let mut rooms = self.rooms;
                rooms[room][depth] = -1;
                let distance = (target - room_pos) + depth + 1;
                possible.push((
                    State { hallway, rooms },
                    distance as i32 * 10i32.pow(pod as u32),
                ))
            }
        }

        // hallway to room
        for hall in 0..11 {
            let pod = self.hallway[hall];
            if pod < 0 {
                continue;
            }
            let room_pos = (2 + pod * 2) as usize;

            if !self.hallway[(hall + 1).min(room_pos)..hall.max(room_pos)]
                .iter()
                .all(|&f| f < 0)
            {
                continue;
            }

            let depth = match self.rooms[pod as usize] {
                [-1, -1] => 1,
                [-1, n] if n == pod => 0,
                _ => continue,
            };

            let mut hallway = self.hallway;
            let mut rooms = self.rooms;
            hallway[hall] = -1;
            rooms[pod as usize][depth] = pod;
            let distance = (hall as isize - room_pos as isize).abs() + depth as isize + 1;
            possible.push((
                State { hallway, rooms },
                distance as i32 * 10i32.pow(pod as u32),
            ))
        }
        possible
    }
}

fn min_cost_to(state: State, mem: &mut HashMap<State, Option<i64>>) -> Option<i64> {
    if state.rooms == [[A, A], [B, B], [C, C], [D, D]] {
        return Some(0);
    }

    if mem.contains_key(&state) {
        return mem[&state];
    }

    let states = state.possible_states();
    // if (states.is_empty()) {
    //     state.show();
    // }
    let result = states
        .into_iter()
        .filter_map(|(state, cost)| min_cost_to(state, mem).map(|f| f + cost as i64))
        .min();
    mem.insert(state, result);
    result
}

fn run(initial: [[i8; 2]; 4]) -> Result<()> {
    let initial = State {
        hallway: [-1; 11],
        rooms: initial,
    };
    let mut result_map = HashMap::new();
    dbg!(min_cost_to(initial, &mut result_map));

    initial.show();
    let mut next = initial;
    loop {
        let states = next.possible_states();
        if states.is_empty() {
            break;
        }
        let n = states
            .into_iter()
            .filter_map(|(state, cost)| Some((state, cost, (*result_map.get(&state)?)?)))
            .min_by_key(|(_state, _cost, tcost)| *tcost);
        if let Some(n) = n {
            dbg!(n.1);
            n.0.show();
            next = n.0;
        } else {
            break;
        }
    }

    Ok(())
}

#[test]
fn cost_0() {
    assert_eq!(
        min_cost_to(
            State {
                hallway: [-1; 11],
                rooms: [[A, A], [B, B], [C, C], [D, D]]
            },
            &mut HashMap::new()
        ),
        Some(0),
    );
}

#[test]
fn paths() {
    let state = State {
        hallway: [C, B, -1, -1, -1, A, -1, -1, -1, D, B],
        rooms: [[-1, -1], [-1, A], [-1, C], [-1, D]],
    };
    state.show();
    println!("->");
    //state.possible_states().into_iter().for_each(|s| s.0.show());

    dbg!(min_cost_to(state, &mut HashMap::new()));
}

#[test]
fn example() {
    dbg!(run([[B, A], [C, D], [B, C], [D, A]]));
}

#[test]
fn input() {
    dbg!(run([[C, B], [D, A], [A, D], [B, C]]));
}

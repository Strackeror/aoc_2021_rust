use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use anyhow::Result;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct State {
    hallway: [i8; 11],
    rooms: [[i8; 4]; 4],
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

        for i in 1..4 {
            print!("  #");
            for r in self.rooms {
                print!("{}#", charmap[&r[i]]);
            }
            println!();
        }

        println!("  #########");

        println!()
    }

    fn room_ok(&self, room: usize) -> bool {
        self.rooms[room]
            .iter()
            .all(|r| *r == room as i8 || *r == -1)
    }

    fn room_depth(&self, room: usize) -> usize {
        self.rooms[room]
            .iter()
            .take_while(|&&pod| pod == -1)
            .count()
    }

    fn possible_states(self) -> Vec<(State, i32)> {
        let mut possible = vec![];
        // room to hallway
        for room in 0..4 {
            if self.room_ok(room) {
                continue;
            }

            let room_pos = 2 + room * 2;
            let depth = self.room_depth(room);
            let pod = self.rooms[room as usize][depth];
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

            if !self.room_ok(pod as usize) {
                continue;
            }
            let depth = self.room_depth(pod as usize);
            if depth == 0 {
                continue;
            }

            let mut hallway = self.hallway;
            let mut rooms = self.rooms;
            hallway[hall] = -1;
            rooms[pod as usize][depth - 1] = pod;
            let distance = (hall as isize - room_pos as isize).abs() + depth as isize;
            possible.push((
                State { hallway, rooms },
                distance as i32 * 10i32.pow(pod as u32),
            ))
        }
        possible
    }
}

fn min_cost_to(state: State, mem: &mut HashMap<State, Option<i64>>) -> Option<i64> {
    if state.rooms == [[A, A, A, A], [B, B, B, B], [C, C, C, C], [D, D, D, D]] {
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
    let rooms = [
        [initial[0][0], D, D, initial[0][1]],
        [initial[1][0], C, B, initial[1][1]],
        [initial[2][0], B, A, initial[2][1]],
        [initial[3][0], A, C, initial[3][1]],
    ];
    let initial = State {
        hallway: [-1; 11],
        rooms,
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
fn example() {
    dbg!(run([[B, A], [C, D], [B, C], [D, A]]));
}

#[test]
fn input() {
    dbg!(run([[C, B], [D, A], [A, D], [B, C]]));
}

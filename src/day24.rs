use std::{cell::RefCell, collections::HashMap};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Param {
    Number(i64),
    Variable(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Expr {
    Input(char),
    Add(char, Param),
    Mul(char, Param),
    Div(char, Param),
    Mod(char, Param),
    Eql(char, Param),
}

peg::parser! {
    grammar instructions_parser() for str {
        rule identifier() -> char = c:$(['a'..='z']) {c.as_bytes()[0] as char}
        rule number() -> Param =  n:$("-"? ['0'..='9']+) { Param::Number(n.parse().unwrap())}
        rule variable() -> Param = i:identifier() { Param::Variable(i) }
        rule param() -> Param = number() / variable()

        rule input() -> Expr = "inp" " " t:identifier()               {Expr::Input(t)}
        rule add()   -> Expr = "add" " " t:identifier() " " p:param() {Expr::Add(t, p)}
        rule mul()   -> Expr = "mul" " " t:identifier() " " p:param() {Expr::Mul(t, p)}
        rule div()   -> Expr = "div" " " t:identifier() " " p:param() {Expr::Div(t, p)}
        rule modu()  -> Expr = "mod" " " t:identifier() " " p:param() {Expr::Mod(t, p)}
        rule eql()   -> Expr = "eql" " " t:identifier() " " p:param() {Expr::Eql(t, p)}

        rule inst() -> Expr = input() / add() / mul() / div() / modu() / eql()
        pub rule list() -> Vec<Expr> = l:inst() ** "\n" {l}
    }
}

fn cid(c: char) -> usize {
    c as usize - 'w' as usize
}

fn val(state: [i64; 4], p: Param) -> i64 {
    match p {
        Param::Number(n) => n,
        Param::Variable(c) => state[cid(c)],
    }
}

fn process_expr(expr: Expr, mut state: [i64; 4]) -> [i64; 4] {
    match expr {
        Expr::Add(c, b) => {
            state[cid(c)] = state[cid(c)] + val(state, b);
        }
        Expr::Mul(c, b) => {
            state[cid(c)] = state[cid(c)] * val(state, b);
        }
        Expr::Div(c, b) => {
            state[cid(c)] = state[cid(c)] / val(state, b);
        }
        Expr::Mod(c, b) => {
            state[cid(c)] = state[cid(c)] % val(state, b);
        }
        Expr::Eql(c, b) => {
            state[cid(c)] = (state[cid(c)] == val(state, b)) as _;
        }
        _ => unreachable!(),
    };
    state
}

fn processr(exprs: &[Expr]) -> Option<i64> {
    fn cid(c: char) -> usize {
        c as usize - 'w' as usize
    }

    fn param(p: Param, state: &[i64]) -> i64 {
        match p {
            Param::Number(n) => n,
            Param::Variable(c) => state[cid(c)],
        }
    }

    fn recursive(
        idx: usize,
        total: i64,
        state: [i64; 4],
        exprs: &[Expr],
        mem: &mut HashMap<(usize, [i64; 4]), Option<i64>>,
    ) -> Option<i64> {
        if idx == exprs.len() {
            return if state[3] == 0 {
                dbg!(Some(total))
            } else {
                None
            };
        }

        if let Some(o) = mem.get(&(idx, state)) {
            return *o;
        }

        let mut nstate = state;
        let result = match exprs[idx] {
            Expr::Input(n) => (0..=9)
                .rev()
                .filter_map(|d| {
                    nstate[cid(n)] = d;
                    recursive(idx + 1, total * 10 + d, nstate, exprs, mem)
                })
                .next(),
            Expr::Eql(c, p) => {
                nstate[cid(c)] = (nstate[cid(c)] == param(p, &nstate)) as i64;
                recursive(idx + 1, total, nstate, exprs, mem)
            }
            Expr::Add(c, p) => {
                nstate[cid(c)] = nstate[cid(c)] + param(p, &nstate);
                recursive(idx + 1, total, nstate, exprs, mem)
            }
            Expr::Mul(c, p) => {
                nstate[cid(c)] = nstate[cid(c)] * param(p, &nstate);
                recursive(idx + 1, total, nstate, exprs, mem)
            }
            Expr::Div(c, p) => {
                nstate[cid(c)] = nstate[cid(c)] / param(p, &nstate);
                recursive(idx + 1, total, nstate, exprs, mem)
            }
            Expr::Mod(c, p) => {
                nstate[cid(c)] = nstate[cid(c)] % param(p, &nstate);
                recursive(idx + 1, total, nstate, exprs, mem)
            }
        };
        mem.insert((idx, state), result);
        result
    }

    recursive(0, 0, [0, 0, 0, 0], exprs, &mut HashMap::new())
}

#[test]
fn example1() {
    let parsed =
        instructions_parser::list(&std::fs::read_to_string("input/day24/example1.txt").unwrap());
    dbg!(&parsed);
    processr(&parsed.unwrap());
}

#[test]
fn example2() {
    let parsed =
        instructions_parser::list(&std::fs::read_to_string("input/day24/example2.txt").unwrap());
    dbg!(&parsed);
    processr(&parsed.unwrap());
}
#[test]
fn input() {
    let parsed =
        instructions_parser::list(&std::fs::read_to_string("input/day24/input.txt").unwrap());
    dbg!(&parsed);
    processr(&parsed.unwrap());
}

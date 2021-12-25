use itertools::Itertools;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    let args = std::env::args().collect_vec();
    let input = args[1].clone();

    match args.get(2) {
        Some(n) => match n.as_str() {
            "day01" => day01::day01(&input),
            "day02" => day02::day02(&input),
            "day03" => day03::day03(&input),
            "day03_bin" => day03::day03_bin(&input),
            "day04" => day04::day04(&input),
            "day05" => day05::day05(&input),
            "day06" => day06::day06(&input),
            "day07" => day07::day07(&input),
            "day08" => day08::day08(&input),
            "day09" => day09::day09(&input),
            "day10" => day10::day10(&input),
            "day11" => day11::day11(&input),
            "day12" => day12::day12(&input),
            "day13" => day13::day13(&input),
            "day14" => day14::day14(&input),
            "day15" => day15::run(&input),
            "day16" => day16::run(&input),
            "day17" => day17::run(&input),
            "day18" => day18::run(&input),
            "day19" => day19::run(&input),
            "day20" => day20::run(&input),
            "day21" => day21::run(&input),
            _ => panic!("unexpected arg"),
        },
        None => day22::run(&input),
    }
    .unwrap();
}

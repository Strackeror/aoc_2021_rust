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

fn main() {
    let args = std::env::args().collect_vec();
    let input_path = args[1].clone();

    match args.get(2) {
        Some(n) => match n.as_str() {
            "day01" => day01::day01(&input_path),
            "day02" => day02::day02(&input_path),
            "day03" => day03::day03(&input_path),
            "day03_bin" => day03::day03_bin(&input_path),
            "day04" => day04::day04(&input_path),
            "day05" => day05::day05(&input_path),
            "day06" => day06::day06(&input_path),
            "day07" => day07::day07(&input_path),
            "day08" => day08::day08(&input_path),
            "day09" => day09::day09(&input_path),
            "day10" => day10::day10(&input_path),
            "day11" => day11::day11(&input_path),
            "day12" => day12::day12(&input_path),
            "day13" => day13::day13(&input_path),
            "day14" => day14::day14(&input_path),
            "day15" => day15::run(&input_path),
            _ => panic!("unexpected arg"),
        },
        None => day16::run(&input_path),
    }
    .unwrap();
}

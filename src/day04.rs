use anyhow::Result;
use itertools::Itertools;

pub(crate) type Grid = Vec<Vec<Option<i32>>>;

pub(crate) fn day04(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let numbers = file
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec();

    let grids = file
        .lines()
        .skip(1)
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            chunk
                .skip(1)
                .map(|grid_line| {
                    grid_line
                        .split_whitespace()
                        .map(|s| Some(s.parse::<i32>().unwrap()))
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    fn is_won(grid: &Grid) -> bool {
        grid.iter().any(|l| l.iter().all(Option::is_none))
            || (0..grid[0].len()).any(|i| grid.iter().all(|l| l[i].is_none()))
    }

    fn grid_result(grid: &Grid, number: i32) -> i32 {
        grid.iter()
            .map(|l| l.iter().filter_map(|o| *o).sum::<i32>())
            .sum::<i32>()
            * number
    }

    let mut marked_grids = grids;
    let mut won_results = vec![0i32; 0];

    for num in numbers {
        for grid in marked_grids.iter_mut() {
            for line in grid.iter_mut() {
                for grid_num in line.iter_mut() {
                    if *grid_num == Some(num) {
                        *grid_num = None
                    }
                }
            }
        }

        let (won, left): (Vec<Grid>, Vec<Grid>) = marked_grids.into_iter().partition(is_won);
        marked_grids = left;
        won_results.extend(won.into_iter().map(|g| grid_result(&g, num)));
    }
    dbg!(won_results.first(), won_results.last());

    Ok(())
}

use std::{collections::HashSet, ops::Range};

use aoc_lib::{aoc, color_eyre::eyre::Result, grid::Grid, rangemap::RangeMap, regex::Regex};

static INPUT: &str = include_str!("../../inputs/day03");

fn part1(input: &str) -> Result<i64> {
	let grid = Grid::for_str(input).unwrap();

	let re = Regex::new(r"\d+").unwrap();
	let check_symbol = |b: u8| b != b'.' && !b.is_ascii_digit();

	let res = re
		.find_iter(input)
		.filter(|m| {
			let Range { start, end } = m.range();
			let start_pos = grid.idx_to_pos(start).unwrap();
			let end_pos = grid.idx_to_pos(end - 1).unwrap();

			grid.adjacent_area(start_pos, end_pos)
				.any(|pos| check_symbol(grid[pos]))
		})
		.map(|m| m.as_str().parse::<i64>().unwrap())
		.sum();

	Ok(res)
}

fn part2(input: &str) -> Result<i64> {
	let grid = Grid::for_str(input).unwrap();

	let re = Regex::new(r"\d+").unwrap();
	let mut num_map: RangeMap<_, i64> = re
		.find_iter(input)
		.map(|m| (m.range(), m.as_str().parse().unwrap()))
		.collect();

	let gears = input
		.as_bytes()
		.iter()
		.enumerate()
		.filter(|(_, &b)| b == b'*');

	let mut res = 0;
	for (gear, _) in gears {
		let pos_to_check = grid.adjacent_pos(grid.idx_to_pos(gear).unwrap());

		let part_nums: HashSet<_> = pos_to_check
			.map(|pos| grid.pos_to_idx(pos).unwrap())
			.filter_map(|idx| num_map.get_key_value(&idx))
			.map(|(r, &v)| (r.clone(), v))
			.collect();

		if part_nums.len() != 2 {
			continue;
		}

		let mut ratio = 1;
		for (r, v) in part_nums {
			num_map.remove(r);
			ratio *= v;
		}

		res += ratio;
	}

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 4361,
	part2 => (EX_INPUT) 467835
}

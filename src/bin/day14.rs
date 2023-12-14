use std::{collections::HashMap, ops::Range};

use aoc_lib::{aoc, color_eyre::eyre::Result};

static INPUT: &str = include_str!("../../inputs/day14");

fn part1(input: &str) -> Result<usize> {
	let input_str = input.trim();
	let mut input = input_str.as_bytes().to_vec();
	let line_width = input_str.lines().next().unwrap().len() + 1;
	let height = input.len() / line_width + 1;
	let width = line_width - 1;

	for x in 0..width {
		let mut last_free_y = None;
		for y in 0..height {
			let idx = y * line_width + x;
			match input[idx] {
				b'.' => {
					if last_free_y.is_none() {
						last_free_y = Some(y)
					}
				}
				b'#' => last_free_y = None,
				b'O' => {
					if let Some(new_y) = last_free_y {
						let new_idx = new_y * line_width + x;
						input[new_idx] = b'O';
						input[idx] = b'.';
						last_free_y = Some(new_y + 1);
					}
				}
				_ => unreachable!(),
			};
		}
	}

	let mut res = 0;
	for x in 0..width {
		for y in 0..height {
			let idx = y * line_width + x;
			if input[idx] == b'O' {
				res += height - y;
			}
		}
	}

	Ok(res)
}

fn part2(input: &str) -> Result<usize> {
	const CYCLE_COUNT: usize = 1_000_000_000;
	let input_str = input.trim();
	let mut input = input_str.as_bytes().to_vec();
	let line_width = input_str.lines().next().unwrap().len() + 1;
	let height = input.len() / line_width + 1;
	let width = line_width - 1;
	let ranges_columns: Vec<_> = (0..width)
		.map(|x| {
			(0..height)
				.filter(|&y| {
					let idx = y * line_width + x;
					input[idx] != b'#'
				})
				.fold(Vec::<Range<usize>>::new(), |mut acc, curr_y| {
					if let Some(last) = acc.last_mut().filter(|last| last.end == curr_y) {
						last.end += 1;
					} else {
						acc.push(curr_y..curr_y + 1);
					}
					acc
				})
		})
		.collect();
	let ranges_rows: Vec<_> = (0..height)
		.map(|y| {
			(0..width)
				.filter(|&x| {
					let idx = y * line_width + x;
					input[idx] != b'#'
				})
				.fold(Vec::<Range<usize>>::new(), |mut acc, curr_x| {
					if let Some(last) = acc.last_mut().filter(|last| last.end == curr_x) {
						last.end += 1;
					} else {
						acc.push(curr_x..curr_x + 1);
					}
					acc
				})
		})
		.collect();

	let mut cache = HashMap::<Vec<u8>, usize>::new();

	let mut i = 0;
	loop {
		if i >= CYCLE_COUNT {
			break;
		}
		if let Some(last_seen) = cache.get(&input) {
			let d = i - last_seen;
			if i + d < CYCLE_COUNT - 1 {
				i += d;
				continue;
			}
		}

		cache.insert(input.clone(), i);

		for (x, ranges) in ranges_columns.iter().enumerate() {
			for range in ranges {
				let rocks = range
					.clone()
					.filter(|&y| input[y * line_width + x] == b'O')
					.count();
				for y in range.start..range.start + rocks {
					input[y * line_width + x] = b'O';
				}
				for y in range.start + rocks..range.end {
					input[y * line_width + x] = b'.';
				}
			}
		}

		for (y, ranges) in ranges_rows.iter().enumerate() {
			for range in ranges {
				let rocks = range
					.clone()
					.filter(|&x| input[y * line_width + x] == b'O')
					.count();
				for x in range.start..range.start + rocks {
					input[y * line_width + x] = b'O';
				}
				for x in range.start + rocks..range.end {
					input[y * line_width + x] = b'.';
				}
			}
		}

		for (x, ranges) in ranges_columns.iter().enumerate() {
			for range in ranges {
				let rocks = range
					.clone()
					.filter(|&y| input[y * line_width + x] == b'O')
					.count();
				for y in range.end - rocks..range.end {
					input[y * line_width + x] = b'O';
				}
				for y in range.start..range.end - rocks {
					input[y * line_width + x] = b'.';
				}
			}
		}

		for (y, ranges) in ranges_rows.iter().enumerate() {
			for range in ranges {
				let rocks = range
					.clone()
					.filter(|&x| input[y * line_width + x] == b'O')
					.count();
				for x in range.end - rocks..range.end {
					input[y * line_width + x] = b'O';
				}
				for x in range.start..range.end - rocks {
					input[y * line_width + x] = b'.';
				}
			}
		}

		i += 1;
	}

	let mut res = 0;
	for x in 0..width {
		for y in 0..height {
			let idx = y * line_width + x;
			if input[idx] == b'O' {
				res += height - y;
			}
		}
	}

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 136,
	part2 => (EX_INPUT) 64
}

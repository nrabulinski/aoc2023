use std::collections::HashSet;

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day11");

fn part1(input: &str) -> Result<usize> {
	let mut grid: Vec<_> = to_lines(input)
		.map(|line| line.as_bytes().to_vec())
		.collect();

	let width = grid[0].len();
	let height = grid.len();

	// expand rows
	for y in (0..height).rev() {
		if grid[y].iter().all(|&c| c == b'.') {
			grid.insert(y, grid[y].clone());
		}
	}

	// expand columns
	for x in (0..width).rev() {
		if grid.iter().map(|row| row[x]).all(|c| c == b'.') {
			grid.iter_mut().for_each(|row| row.insert(x, b'.'));
		}
	}

	let galaxies: Vec<_> = grid
		.iter()
		.enumerate()
		.flat_map(|(y, row)| {
			row.iter()
				.enumerate()
				.filter_map(move |(x, &c)| if c == b'#' { Some((x, y)) } else { None })
		})
		.collect();

	let mut res = 0;
	for i in 0..galaxies.len() - 1 {
		let (ax, ay) = galaxies[i];
		for &(bx, by) in &galaxies[i..] {
			let d = ax.abs_diff(bx) + ay.abs_diff(by);
			res += d;
		}
	}

	Ok(res)
}

fn part2(input: &str) -> Result<i64> {
	#[cfg(not(test))]
	const EMPTY_ROW_SCALE: i64 = 1_000_000;
	#[cfg(test)]
	const EMPTY_ROW_SCALE: i64 = 10;

	let grid: Vec<_> = to_lines(input)
		.map(|line| line.as_bytes().to_vec())
		.collect();

	let width = grid[0].len();
	let height = grid.len();

	let mut expanded_rows = HashSet::new();
	let mut expanded_cols = HashSet::new();

	// expand rows
	for y in (0..height).rev() {
		if grid[y].iter().all(|&c| c == b'.') {
			expanded_rows.insert(y as i64);
		}
	}

	// expand columns
	for x in (0..width).rev() {
		if grid.iter().map(|row| row[x]).all(|c| c == b'.') {
			expanded_cols.insert(x as i64);
		}
	}

	let galaxies: Vec<_> = grid
		.iter()
		.enumerate()
		.flat_map(|(y, row)| {
			row.iter()
				.enumerate()
				.filter_map(move |(x, &c)| if c == b'#' { Some((x, y)) } else { None })
		})
		.collect();

	let mut res = 0;
	for i in 0..galaxies.len() - 1 {
		let (ax, ay) = galaxies[i];
		for &(bx, by) in &galaxies[i..] {
			let mut x = ax as i64;
			let xd = if ax < bx { 1 } else { -1 };
			while x != bx as i64 {
				if expanded_cols.contains(&x) {
					res += EMPTY_ROW_SCALE;
				} else {
					res += 1;
				}
				x += xd;
			}

			let mut y = ay as i64;
			let yd = if ay < by { 1 } else { -1 };
			while y != by as i64 {
				if expanded_rows.contains(&y) {
					res += EMPTY_ROW_SCALE;
				} else {
					res += 1;
				}
				y += yd;
			}
		}
	}

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 374,
	part2 => (EX_INPUT) 1030
}

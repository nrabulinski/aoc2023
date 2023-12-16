use std::collections::HashSet;

use aoc_lib::{aoc, color_eyre::eyre::Result, grid::Grid};

static INPUT: &str = include_str!("../../inputs/day16");

fn laser(grid: Grid<'_>, direction: (i64, i64), start_pos: (i64, i64)) -> usize {
	let mut cycles = HashSet::new();
	let mut dirs_pos = Vec::<((i64, i64), (i64, i64))>::new();
	dirs_pos.push((direction, start_pos));

	while dirs_pos.iter().any(|&(_, pos)| grid.is_valid_pos(pos)) {
		for i in (0..dirs_pos.len()).rev() {
			let (dir, pos) = dirs_pos[i];
			if !grid.is_valid_pos(pos) || cycles.contains(&(dir, pos)) {
				continue;
			}
			cycles.insert((dir, pos));

			match grid.get_pos(pos).unwrap() {
				b'.' => (),
				b'-' if dir.1 == 0 => (),
				b'|' if dir.0 == 0 => (),
				b'\\' => {
					dirs_pos[i].0 = (dir.1, dir.0);
				}
				b'/' => {
					dirs_pos[i].0 = (-dir.1, -dir.0);
				}
				b'-' | b'|' => {
					dirs_pos.remove(i);
					let new_dir = (dir.1, dir.0);
					let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
					dirs_pos.insert(i, (new_dir, new_pos));

					let new_dir = (-dir.1, -dir.0);
					let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
					dirs_pos.insert(i, (new_dir, new_pos));

					continue;
				}
				&c => unreachable!("char: {}", c as char),
			}

			dirs_pos[i].1 = (pos.0 + dirs_pos[i].0 .0, pos.1 + dirs_pos[i].0 .1);
		}
		for i in (0..dirs_pos.len()).rev() {
			if !grid.is_valid_pos(dirs_pos[i].1) || cycles.contains(&dirs_pos[i]) {
				dirs_pos.remove(i);
			}
		}
	}

	cycles
		.into_iter()
		.map(|(_, pos)| pos)
		.collect::<HashSet<_>>()
		.len()
}

fn part1(input: &str) -> Result<usize> {
	let grid = Grid::for_str(input).unwrap();
	let res = laser(grid, (1, 0), (0, 0));

	Ok(res)
}

fn part2(input: &str) -> Result<usize> {
	let grid = Grid::for_str(input).unwrap();

	let top = (0..grid.width()).map(|x| ((0, 1), (x, 0)));
	let bottom = (0..grid.width()).map(|x| ((0, -1), (x, grid.height() - 1)));
	let left = (0..grid.height()).map(|y| ((1, 0), (0, y)));
	let right = (0..grid.height()).map(|y| ((-1, 0), (grid.width() - 1, y)));

	let res = top
		.chain(bottom)
		.chain(left)
		.chain(right)
		.map(|(dir, start_pos)| laser(grid, dir, start_pos))
		.max()
		.unwrap();

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 46,
	part2 => (EX_INPUT) 51
}

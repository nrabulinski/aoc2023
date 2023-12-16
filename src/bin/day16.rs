use std::collections::HashSet;

use aoc_lib::{aoc, color_eyre::eyre::Result};

static INPUT: &str = include_str!("../../inputs/day16");

fn valid_position((x, y): (i64, i64), width: i64, height: i64) -> bool {
	x >= 0 && x < width && y >= 0 && y < height
}

fn laser(
	input: &[u8],
	direction: (i64, i64),
	start_pos: (i64, i64),
	width: i64,
	height: i64,
) -> usize {
	let mut cycles = HashSet::new();
	let mut dirs_pos = Vec::<((i64, i64), (i64, i64))>::new();
	dirs_pos.push((direction, start_pos));

	while dirs_pos
		.iter()
		.any(|&(_, pos)| valid_position(pos, width, height))
	{
		for i in (0..dirs_pos.len()).rev() {
			let (dir, pos) = dirs_pos[i];
			if !valid_position(pos, width, height) || cycles.contains(&(dir, pos)) {
				continue;
			}
			cycles.insert((dir, pos));
			let idx = pos.1 * (width + 1) + pos.0;

			match input[idx as usize] {
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
				c => unreachable!("char: {}", c as char),
			}

			dirs_pos[i].1 = (pos.0 + dirs_pos[i].0 .0, pos.1 + dirs_pos[i].0 .1);
		}
		for i in (0..dirs_pos.len()).rev() {
			if !valid_position(dirs_pos[i].1, width, height) || cycles.contains(&dirs_pos[i]) {
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
	let input = input.trim();
	let line_width = input.lines().next().unwrap().len() + 1;
	let height = input.len() / line_width + 1;
	let width = line_width - 1;
	let res = laser(
		input.as_bytes(),
		(1, 0),
		(0, 0),
		width as i64,
		height as i64,
	);

	Ok(res)
}

fn part2(input: &str) -> Result<usize> {
	let input = input.trim();
	let line_width = input.lines().next().unwrap().len() + 1;
	let height = input.len() / line_width + 1;
	let width = line_width - 1;

	let top = (0..width).map(|x| ((0, 1), (x as i64, 0)));
	let bottom = (0..width).map(|x| ((0, -1), (x as i64, height as i64 - 1)));
	let left = (0..height).map(|y| ((1, 0), (0, y as i64)));
	let right = (0..height).map(|y| ((-1, 0), (width as i64 - 1, y as i64)));

	let res = top
		.chain(bottom)
		.chain(left)
		.chain(right)
		.map(|(dir, start_pos)| {
			laser(
				input.as_bytes(),
				dir,
				start_pos,
				width as i64,
				height as i64,
			)
		})
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

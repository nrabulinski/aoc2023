use std::collections::{HashMap, HashSet};

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day10");

fn adj_pipes(
	coords: impl IntoIterator<Item = (i64, i64)>,
	width: i64,
	height: i64,
) -> impl Iterator<Item = (i64, i64)> {
	coords
		.into_iter()
		.filter(move |&(x, y)| x >= 0 && x < width && y >= 0 && y < height)
}

fn max_dist(pipes: &HashMap<(i64, i64), Vec<(i64, i64)>>, start: (i64, i64)) -> i64 {
	let (mut prev_left, mut prev_right) = (start, start);
	let (mut left, mut right) = {
		let adj = pipes.get(&start).unwrap();
		(adj[0], adj[1])
	};
	let mut d1 = 1;
	let mut d2 = 1;

	while left != right {
		let new_left = pipes
			.get(&left)
			.unwrap()
			.iter()
			.copied()
			.find(|&pipe| pipe != prev_left)
			.unwrap();
		let new_right = pipes
			.get(&right)
			.unwrap()
			.iter()
			.copied()
			.find(|&pipe| pipe != prev_right)
			.unwrap();
		d1 += 1;
		d2 += 1;
		(prev_left, left, prev_right, right) = (left, new_left, right, new_right);
	}

	d1.max(d2)
}

fn find_main_loop(
	pipes: &HashMap<(i64, i64), Vec<(i64, i64)>>,
	start: (i64, i64),
) -> HashSet<(i64, i64)> {
	let mut main_loop = HashSet::new();

	fn dfs(
		curr: (i64, i64),
		vis: &mut HashSet<(i64, i64)>,
		adj: &HashMap<(i64, i64), Vec<(i64, i64)>>,
	) {
		vis.insert(curr);

		for pos in adj.get(&curr).unwrap() {
			if !vis.contains(pos) {
				dfs(*pos, vis, adj);
			}
		}
	}

	dfs(start, &mut main_loop, &pipes);

	main_loop
}

// this is all useless but it was my first instinct to parse it this way, oh well
fn parse_pipes(input: &str) -> ((i64, i64), HashMap<(i64, i64), Vec<(i64, i64)>>, (i64, i64)) {
	// width including the newline
	let line_width = input.trim().lines().next().unwrap().len() + 1;
	let height = (input.trim().len() / line_width + 1) as i64;
	let width = (line_width - 1) as i64;

	let mut pipes: HashMap<_, Vec<_>> = to_lines(input)
		.enumerate()
		.map(|(y, line)| {
			let y = y as i64;
			line.as_bytes()
				.iter()
				.enumerate()
				.filter_map(move |(x, &c)| {
					let x = x as i64;
					match c {
						b'.' => None,
						b'|' => Some((
							(x, y),
							adj_pipes([(x, y - 1), (x, y + 1)], width, height).collect(),
						)),
						b'-' => Some((
							(x, y),
							adj_pipes([(x - 1, y), (x + 1, y)], width, height).collect(),
						)),
						b'L' => Some((
							(x, y),
							adj_pipes([(x, y - 1), (x + 1, y)], width, height).collect(),
						)),
						b'J' => Some((
							(x, y),
							adj_pipes([(x, y - 1), (x - 1, y)], width, height).collect(),
						)),
						b'7' => Some((
							(x, y),
							adj_pipes([(x - 1, y), (x, y + 1)], width, height).collect(),
						)),
						b'F' => Some((
							(x, y),
							adj_pipes([(x + 1, y), (x, y + 1)], width, height).collect(),
						)),
						b'S' => Some(((x, y), Vec::new())),
						c => panic!("Unexpected character in input: {}", c as char),
					}
				})
		})
		.flatten()
		.collect();

	// find and fix up starting position
	let start_idx = input.trim().find('S').unwrap();
	let start_pos = (
		(start_idx % line_width) as i64,
		(start_idx / line_width) as i64,
	);
	let start_adj = [-1, 1]
		.into_iter()
		.map(|dx| (start_pos.0 + dx, start_pos.1))
		.chain(
			[-1, 1]
				.into_iter()
				.map(|dy| (start_pos.0, start_pos.1 + dy))
				.into_iter(),
		)
		.filter(|pos| {
			pipes
				.get(pos)
				.map(|adj| adj.iter().any(|&pos| pos == start_pos))
				.unwrap_or(false)
		})
		.collect();
	*pipes.get_mut(&start_pos).unwrap() = start_adj;

	((width, height), pipes, start_pos)
}

fn part1(input: &str) -> Result<i64> {
	let (_, pipes, start_pos) = parse_pipes(input);

	Ok(max_dist(&pipes, start_pos))
}

/// Check whether a pipe is a "cross" pipe.
/// A "cross" pipe is any pipe that has a connection above it,
/// meaning when we cross it, we're either entering the pipe loop or leaving it.
/// The *above* is arbitrary, it could be below as well.
/// It could even be left/right, if I went for vertical scanlines instead.
///
/// The point is to not count situations like: └┐ as entering and leaving the loop,
/// hence we count └, but not ┐.
fn is_cross((x, y): (i64, i64), pipes: &HashMap<(i64, i64), Vec<(i64, i64)>>) -> bool {
	pipes
		.get(&(x, y))
		.map(|adj| adj.iter().any(|&(_, other_y)| other_y == y - 1))
		.unwrap_or(false)
}

fn part2(input: &str) -> Result<i64> {
	let ((width, height), pipes, start_pos) = parse_pipes(input);

	let main_loop = find_main_loop(&pipes, start_pos);

	let mut res = 0;
	for y in 0..height {
		let mut inside = false;
		for x in 0..width {
			if main_loop.contains(&(x, y)) {
				if is_cross((x, y), &pipes) {
					inside = !inside;
				}
			} else if inside {
				res += 1;
			}
		}
	}

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT_1: &str = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

#[allow(dead_code)]
static EX_INPUT_2: &str = r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;
#[allow(dead_code)]
const EX_INPUT_2_SOL: i64 = 10;

#[allow(dead_code)]
static EX_INPUT_3: &str = r#"
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"#;
#[allow(dead_code)]
const EX_INPUT_3_SOL: i64 = 4;

aoc! {
	INPUT:
	part1 => (EX_INPUT_1) 8,
	part2 => (EX_INPUT_2) crate::EX_INPUT_2_SOL
}

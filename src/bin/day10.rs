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

fn is_corner(curr: (i64, i64), pipes: &HashMap<(i64, i64), Vec<(i64, i64)>>) -> bool {
	let adj = pipes.get(&curr).unwrap();
	let (a, b) = (adj[0], adj[1]);
	a.0 != b.0 && a.1 != b.1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge {
	Left,
	Right,
	Top,
	Bot,
	// The boolean indicates whether the edge is "inner".
	// An "outer" edge is comparable to a single pixel in the corner,
	// while an "inner" edge is the corner + the edges it connects to.
	TopLeft(bool),
	TopRight(bool),
	BotLeft(bool),
	BotRight(bool),
}

impl Edge {
	fn is_corner(&self) -> bool {
		match self {
			Edge::TopLeft(_) | Edge::TopRight(_) | Edge::BotLeft(_) | Edge::BotRight(_) => true,
			_ => false,
		}
	}

	fn is_inside(left: Edge, right: Edge) -> bool {
		matches!(left, Edge::Left | Edge::TopLeft(_) | Edge::BotLeft(_))
			&& matches!(right, Edge::Right | Edge::TopRight(_) | Edge::BotRight(_))
	}

	fn is_top(&self) -> bool {
		matches!(self, Edge::Top | Edge::TopLeft(_) | Edge::TopRight(_))
	}

	fn is_right(&self) -> bool {
		matches!(self, Edge::Right | Edge::TopRight(_) | Edge::BotRight(_))
	}
}

fn part2(input: &str) -> Result<i64> {
	let ((width, height), pipes, start_pos) = parse_pipes(input);

	let main_loop = find_main_loop(&pipes, start_pos);

	let &min_point = main_loop.iter().min_by_key(|&(x, y)| (y, x)).unwrap();
	assert!(is_corner(min_point, &pipes));

	let edges: HashMap<_, _> = std::iter::successors(
		Some((min_point, Edge::TopLeft(true), None)),
		|&(prev, typ, parent)| {
			let adj = pipes.get(&prev).unwrap();

			let next = parent
				.and_then(|p| adj.iter().find(|&&n| n != p))
				.copied()
				.unwrap_or(adj[0]);

			let new_typ = if is_corner(next, &pipes) {
				// next element of the loop to decide the edge type
				let child = pipes
					.get(&next)
					.unwrap()
					.iter()
					.find(|&&n| n != prev)
					.unwrap();

				// prev -> next -> child
				// we know next is some form of a corner,
				// so based on prev and child coordinates we find out what corner this is

				match (
					(next.0 - prev.0, next.1 - prev.1),
					(child.0 - next.0, child.1 - next.1),
				) {
					(a, b) if a == b => unreachable!("Previous was a corner"),
					((1, 0), (0, 1)) => {
						if typ.is_top() {
							Edge::TopRight(true)
						} else {
							Edge::BotLeft(false)
						}
					}
					((1, 0), (0, -1)) => {
						if typ.is_top() {
							Edge::TopLeft(false)
						} else {
							Edge::TopRight(true)
						}
					}
					((-1, 0), (0, 1)) => {
						if typ.is_top() {
							Edge::TopLeft(true)
						} else {
							Edge::BotRight(false)
						}
					}
					((-1, 0), (0, -1)) => {
						if typ.is_top() {
							Edge::TopRight(false)
						} else {
							Edge::BotLeft(true)
						}
					}
					((0, 1), (1, 0)) => {
						if typ.is_right() {
							Edge::TopRight(false)
						} else {
							Edge::BotLeft(true)
						}
					}
					((0, 1), (-1, 0)) => {
						if typ.is_right() {
							Edge::BotRight(true)
						} else {
							Edge::TopLeft(false)
						}
					}
					((0, -1), (1, 0)) => {
						if typ.is_right() {
							Edge::BotRight(false)
						} else {
							Edge::TopLeft(true)
						}
					}
					((0, -1), (-1, 0)) => {
						if typ.is_right() {
							Edge::TopRight(true)
						} else {
							Edge::BotLeft(false)
						}
					}
					_ => unreachable!(),
				}
			} else if typ.is_corner() {
				match typ {
					Edge::TopLeft(_) => {
						if next.0 == prev.0 {
							Edge::Left
						} else {
							Edge::Top
						}
					}
					Edge::BotLeft(_) => {
						if next.0 == prev.0 {
							Edge::Left
						} else {
							Edge::Bot
						}
					}
					Edge::TopRight(_) => {
						if next.0 == prev.0 {
							Edge::Right
						} else {
							Edge::Top
						}
					}
					Edge::BotRight(_) => {
						if next.0 == prev.0 {
							Edge::Right
						} else {
							Edge::Bot
						}
					}
					_ => unreachable!(),
				}
			} else {
				typ
			};

			if next == min_point {
				None
			} else {
				Some((next, new_typ, Some(prev)))
			}
		},
	)
	.map(|(pos, typ, _)| (pos, typ))
	.collect();

	let mut flodded = HashSet::<(i64, i64)>::new();

	let mut res = 0;
	{
		let mut y = 0;

		loop {
			let mut x = 0;
			loop {
				if let Some(left) = edges.get(&(x, y)) {
					let Some(next_x) =
						(x + 1..width).find(|&new_x| edges.contains_key(&(new_x, y)))
					else {
						// no more pipes on this line
						break;
					};

					if Edge::is_inside(*left, *edges.get(&(next_x, y)).unwrap()) {
						for new_x in x + 1..next_x {
							flodded.insert((new_x, y));
						}
						res += next_x - x - 1;
					}

					x = next_x - 1;
				}

				x += 1;
				if x >= width {
					break;
				}
			}

			y += 1;
			if y >= height {
				break;
			}
		}
	}

	eprintln!("{}", input.trim());

	for y in 0..height {
		for x in 0..width {
			eprint!(
				"{}",
				if let Some(typ) = edges.get(&(x, y)) {
					match typ {
						Edge::Left => '▎',
						Edge::Right => '🮇',
						Edge::Top => '🮂',
						Edge::Bot => '▂',
						Edge::TopLeft(true) => '▛',
						Edge::TopRight(true) => '▜',
						Edge::BotLeft(true) => '▙',
						Edge::BotRight(true) => '▟',
						Edge::TopLeft(false) => '▘',
						Edge::TopRight(false) => '▝',
						Edge::BotLeft(false) => '▖',
						Edge::BotRight(false) => '▗',
					}
				} else {
					if flodded.contains(&(x, y)) {
						'I'
					} else {
						'o'
					}
				}
			);
		}
		eprintln!();
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

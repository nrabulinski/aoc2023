use std::collections::{BTreeSet, HashMap};

use aoc_lib::{
	aoc,
	color_eyre::eyre::Result,
	grid::{Point, PointExt},
	iter::IterExt,
	to_lines,
};

static INPUT: &str = include_str!("../../inputs/day18");

// NOTE: Point is (y, x) instead of the usual (x, y),
//       because we want them to be sorted by row.
fn solve(points: BTreeSet<Point>) -> i64 {
	let mut y_values: Vec<_> = points.iter().map(|&(y, _)| y).collect();
	y_values.dedup();
	let mut iter = points.into_iter().peekable();

	let mut res = 0;
	let mut previous_y = None;
	let mut curr_points = BTreeSet::<i64>::new();
	for y in y_values {
		let maybe_sum = previous_y
			.take()
			.map(|(prev_y, prev_sum)| {
				let d = y - prev_y - 1;
				res += d * prev_sum;
				prev_sum
			})
			.unwrap_or(0);

		for p in std::iter::from_fn(|| iter.next_if(move |&(next_y, _)| next_y == y))
			.map(move |(_, x)| x)
		{
			if curr_points.contains(&p) {
				curr_points.remove(&p);
			} else {
				curr_points.insert(p);
			}
		}

		let next_sum = curr_points
			.iter()
			.arr_chunks()
			.map(|[a, b]| b - a + 1)
			.sum();

		res += maybe_sum.max(next_sum);

		previous_y = Some((y, next_sum));
	}

	res
}

fn is_cross((x, y): Point, pipes: &HashMap<Point, [Point; 2]>) -> bool {
	pipes
		.get(&(x, y))
		.map(|adj| adj.iter().any(|&(_, other_y)| other_y == y - 1))
		.unwrap_or(false)
}

fn parse(line: &str) -> (Point, i64, &str) {
	let mut i = line.trim().split_ascii_whitespace();
	let dir = match i.next().unwrap() {
		"L" => (-1, 0),
		"U" => (0, -1),
		"D" => (0, 1),
		"R" => (1, 0),
		_ => panic!(),
	};
	let cnt = i.next().unwrap().parse().unwrap();
	let color = i.next().unwrap();
	let color = &color[1..color.len() - 1];
	(dir, cnt, color)
}

fn part1(input: &str) -> Result<usize> {
	let mut map = HashMap::new();

	let mut curr = (0, 0);
	let mut top_left = curr;
	let mut bottom_right = curr;
	for (dir, cnt, _) in to_lines(input).map(parse) {
		for _ in 0..cnt {
			let next = curr.add(&dir);
			top_left = (top_left.0.min(next.0), top_left.1.min(next.1));
			bottom_right = (bottom_right.0.max(next.0), bottom_right.1.max(next.1));

			map.entry(curr).or_insert([(0, 0); 2])[1] = next;
			map.entry(next).or_insert([(0, 0); 2])[0] = curr;

			curr = next;
		}
	}

	let mut res = map.len();
	for y in top_left.1..=bottom_right.1 {
		let mut inside = false;
		for x in top_left.0..=bottom_right.0 {
			if map.contains_key(&(x, y)) {
				if is_cross((x, y), &map) {
					inside = !inside;
				}
			} else if inside {
				res += 1;
			}
		}
	}

	Ok(res)
}

fn part2(input: &str) -> Result<i64> {
	let map: BTreeSet<_> = to_lines(input)
		.map(parse)
		.scan((0, 0), |curr, (.., hex)| {
			let cnt = &hex[1..][..5];
			let cnt = i64::from_str_radix(cnt, 16).unwrap();

			let diff = match hex.as_bytes()[6] {
				b'0' => (0, cnt),
				b'1' => (cnt, 0),
				b'2' => (0, -cnt),
				b'3' => (-cnt, 0),
				_ => panic!(),
			};
			*curr = curr.add(&diff);
			Some(*curr)
		})
		.collect();

	Ok(solve(map))
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 62,
	part2 => (EX_INPUT) 952408144115
}

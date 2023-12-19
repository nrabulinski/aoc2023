//! NOTE: Point is (y, x) instead of the usual (x, y),
//!       because we want them to be sorted by row.
// TODO: Normalize ranges without allocations since we know they're sorted
// TODO: Only allocate curr_points, nothing else should be needed
// TODO: Try the shoelace formula with no allocations

use std::{collections::BTreeSet, ops::RangeInclusive};

use aoc_lib::{
	aoc,
	color_eyre::eyre::Result,
	grid::{Point, PointExt},
	iter::IterExt,
	to_lines,
};

static INPUT: &str = include_str!("../../inputs/day18");

fn normalize_ranges(i: impl IntoIterator<Item = RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
	let mut r: Vec<_> = i.into_iter().collect();
	r.sort_unstable_by_key(|r| *r.start());

	r.into_iter().fold(Vec::new(), |mut acc, curr| {
		if acc.last().is_some_and(|prev| prev.contains(curr.start())) {
			let last = acc.last_mut().unwrap();
			let start = *last.start();
			let end = *last.end().max(curr.end());
			*last = start..=end;
		} else {
			acc.push(curr);
		}
		acc
	})
}

fn solve(diffs: impl Iterator<Item = Point>) -> i64 {
	let points: BTreeSet<_> = diffs
		.scan((0, 0), |curr, diff| {
			*curr = curr.add(&diff);
			Some(*curr)
		})
		.collect();
	let mut y_values: Vec<_> = points.iter().map(|&(y, _)| y).collect();
	y_values.dedup();
	let mut iter = points.into_iter().peekable();

	let mut res = 0;
	let mut previous_y = None;
	let mut curr_points = BTreeSet::<i64>::new();
	for y in y_values {
		let curr_ranges: Vec<_> = curr_points
			.iter()
			.arr_chunks()
			.map(|[&a, &b]| a..=b)
			.collect();
		let prev_sum: i64 = curr_ranges.iter().map(|r| *r.end() - *r.start() + 1).sum();
		if let Some(prev_y) = previous_y {
			let d = y - prev_y - 1;
			res += d * prev_sum;
		}

		let next_points: BTreeSet<_> =
			std::iter::from_fn(|| iter.next_if(move |&(next_y, _)| next_y == y))
				.map(move |(_, x)| x)
				.collect();

		let curr_row = normalize_ranges(
			curr_ranges
				.into_iter()
				.chain(next_points.iter().arr_chunks().map(|[&a, &b]| a..=b)),
		);
		res += curr_row
			.into_iter()
			.map(|r| *r.end() - *r.start() + 1)
			.sum::<i64>();

		curr_points = curr_points
			.symmetric_difference(&next_points)
			.copied()
			.collect();

		previous_y = Some(y);
	}

	res
}

fn part1(input: &str) -> Result<i64> {
	let map = to_lines(input).map(|line| {
		let mut i = line.split_ascii_whitespace();
		let dir = i.next().unwrap();
		let cnt: i64 = i.next().unwrap().parse().unwrap();
		match dir {
			"L" => (0, -cnt),
			"U" => (-cnt, 0),
			"D" => (cnt, 0),
			"R" => (0, cnt),
			_ => panic!(),
		}
	});

	Ok(solve(map))
}

fn part2(input: &str) -> Result<i64> {
	let map = to_lines(input).map(|line| {
		let mut i = line.split_ascii_whitespace();
		let _dir = i.next().unwrap();
		let _cnt = i.next().unwrap();
		let hex = i.next().unwrap();
		let cnt = &hex[2..][..5];
		let cnt = i64::from_str_radix(cnt, 16).unwrap();

		match hex.as_bytes()[7] {
			b'0' => (0, cnt),
			b'1' => (cnt, 0),
			b'2' => (0, -cnt),
			b'3' => (-cnt, 0),
			_ => panic!(),
		}
	});

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

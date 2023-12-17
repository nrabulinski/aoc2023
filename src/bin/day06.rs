#![feature(isqrt)]
use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day06");

fn find_res(time: i64, dist: i64) -> i64 {
	(1..time)
		.filter(|t| {
			let rest = time - t;
			t * rest > dist
		})
		.count() as i64
}

fn parse(input: &str) -> impl Iterator<Item = i64> + '_ {
	let (_, nums) = input.split_once(':').unwrap();
	nums.trim()
		.split_ascii_whitespace()
		.map(|n| n.parse().unwrap())
}

fn part1(input: &str) -> Result<i64> {
	let mut input = to_lines(input);
	let time = input.next().map(parse).unwrap();
	let distance = input.next().map(parse).unwrap();

	let res = time
		.zip(distance)
		.map(|(time, distance)| find_res(time, distance))
		.product();

	Ok(res)
}

fn parse2(input: &str) -> i64 {
	let (_, nums) = input.split_once(':').unwrap();
	nums.trim()
		.split_ascii_whitespace()
		.collect::<String>()
		.parse()
		.unwrap()
}

fn part2(input: &str) -> Result<i64> {
	let mut input = to_lines(input);
	let time = input.next().map(parse2).unwrap();
	let distance = input.next().map(parse2).unwrap();

	Ok(find_res(time, distance))
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
Time:      7  15   30
Distance:  9  40  200
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 288,
	part2 => (EX_INPUT) 71503
}

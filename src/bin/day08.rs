use std::collections::HashMap;

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day08");

fn part1(input: &str) -> Result<i64> {
	let mut input = to_lines(input);

	let ins = input.next().unwrap().as_bytes().iter().cycle();

	let coll: HashMap<_, _> = input
		.skip(1)
		.map(|line| {
			let (from, to) = line.split_once(" = ").unwrap();
			let (left, right) = to[1..to.len() - 1].split_once(", ").unwrap();

			(from, (left, right))
		})
		.collect();

	let res = ins
		.scan("AAA", |state, dir| {
			let (left, right) = coll.get(state).unwrap();
			match dir {
				b'L' => *state = left,
				b'R' => *state = right,
				_ => unreachable!(),
			}
			Some(*state)
		})
		.position(|curr| curr == "ZZZ")
		.unwrap() as i64;

	Ok(res + 1)
}

fn gcd(a: i64, b: i64) -> i64 {
	if b == 0 {
		a
	} else {
		gcd(b, a % b)
	}
}

fn lcm(a: i64, b: i64) -> i64 {
	a * b / gcd(a, b)
}

// This only works because every path properly cycles,
// and every period is constant.
// I tried to be general at first because I naturally assumed this wouldn't be the case,
// but we can't have nice things.
// I HATE AOC I HATE AOC I HATE AOC I HATE AOC I HATE AOC I HATE AOC
// I HATE AOC I HATE AOC I HATE AOC I HATE AOC I HATE AOC I HATE AOC
// I HATE AOC I HATE AOC I HATE AOC I HATE AOC I HATE AOC I HATE AOC
// I HATE AOC I HATE AOC I HATE AOC I HATE AOC I HATE AOC I HATE AOC
// I HATE AOC I HATE AOC I HATE AOC I HATE AOC I HATE AOC I HATE AOC
fn part2(input: &str) -> Result<i64> {
	let mut input = to_lines(input);

	let ins = input.next().unwrap().as_bytes();

	let coll: HashMap<_, _> = input
		.skip(1)
		.map(|line| {
			let (from, to) = line.split_once(" = ").unwrap();
			let (left, right) = to[1..to.len() - 1].split_once(", ").unwrap();

			(from, (left, right))
		})
		.collect();

	let res = coll
		.keys()
		.filter(|node| node.ends_with('A'))
		.map(|node| {
			ins.iter()
				.copied()
				.cycle()
				.scan(*node, |state, dir| {
					let (left, right) = coll.get(state).unwrap();
					match dir {
						b'L' => *state = left,
						b'R' => *state = right,
						_ => unreachable!(),
					}
					Some(*state)
				})
				.position(|node| node.ends_with('Z'))
				.unwrap() as i64 + 1
		})
		.reduce(lcm)
		.unwrap();

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT_1: &str = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

#[allow(dead_code)]
static EX_INPUT_2: &str = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT_1) 2,
	part2 => (EX_INPUT_2) 6
}

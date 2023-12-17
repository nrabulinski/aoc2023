use std::collections::{HashSet, VecDeque};

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day04");

fn parse_game(line: &str) -> i64 {
	let (_, nums) = line.split_once(':').unwrap();
	let (win, ours) = nums.split_once('|').unwrap();

	let win: HashSet<i64> = win
		.trim()
		.split_ascii_whitespace()
		.map(|n| n.parse().unwrap())
		.collect();
	let ours: HashSet<i64> = ours
		.trim()
		.split_ascii_whitespace()
		.map(|n| n.parse().unwrap())
		.collect();

	win.intersection(&ours).count() as _
}

fn part1(input: &str) -> Result<i64> {
	let res = to_lines(input)
		.filter_map(|line| match parse_game(line) {
			0 => None,
			exp => Some(i64::pow(2, exp as u32 - 1)),
		})
		.sum();

	Ok(res)
}

fn part2(input: &str) -> Result<i64> {
	let res = to_lines(input)
		.scan(VecDeque::new(), |pile, line| {
			let wins = parse_game(line);
			let copies = pile.pop_front().unwrap_or(0) + 1;
			for i in 0..wins as usize {
				if let Some(e) = pile.get_mut(i) {
					*e += copies;
				} else {
					pile.push_back(copies);
				}
			}
			Some(copies)
		})
		.sum();

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT_1: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

aoc! {
	INPUT :
	part1 => (EX_INPUT_1) 13,
	part2 => (EX_INPUT_1) 30
}

#![feature(iter_array_chunks)]
use std::mem::replace;

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day05");

fn part1(input: &str) -> Result<i64> {
	let mut categories = input.trim().split("\n\n");
	let seeds = categories.next().unwrap();
	let (_, seeds) = seeds.split_once(':').unwrap();
	let seeds: Vec<i64> = seeds
		.trim()
		.split_ascii_whitespace()
		.map(|s| s.parse().unwrap())
		.collect();

	let res = categories
		.fold(seeds, |mut seeds, cat| {
			let lines = to_lines(cat).skip(1).map(|line| {
				line.split_ascii_whitespace()
					.map(|n| n.parse().unwrap())
					.collect::<Vec<i64>>()
			});

			let mut res = Vec::new();

			for range in lines {
				let dst = range[0];
				let src = range[1];
				let len = range[2];
				let max_src = src + len;
				let d = src - dst;

				for i in (0..seeds.len()).rev() {
					if seeds[i] >= src && seeds[i] < max_src {
						res.push(seeds.remove(i) - d);
					}
				}
			}

			res.append(&mut seeds);

			res
		})
		.into_iter()
		.min()
		.unwrap();

	Ok(res)
}

fn part2(input: &str) -> Result<i64> {
	let mut categories = input.trim().split("\n\n");
	let seeds = categories.next().unwrap();
	let (_, seeds) = seeds.split_once(':').unwrap();
	let seeds: Vec<(i64, i64)> = seeds
		.trim()
		.split_ascii_whitespace()
		.map(|s| s.parse().unwrap())
		.array_chunks()
		.map(|[start, len]| (start, len))
		.collect();

	let res = categories
		.fold(seeds, |mut seeds, cat| {
			let lines = to_lines(cat).skip(1).map(|line| {
				line.split_ascii_whitespace()
					.map(|n| n.parse().unwrap())
					.collect::<Vec<i64>>()
			});

			let mut res = Vec::new();

			for range in lines {
				let dst = range[0];
				let src = range[1];
				let len = range[2];
				let max_src = src + len;
				let d = src - dst;

				for i in (0..seeds.len()).rev() {
					let (cs, clen) = seeds[i];
					let last = cs + clen - 1;
					if cs >= max_src {
						continue;
					} else if cs >= src {
						let offset = cs - src;
						let nlen = clen.min(len - offset);
						res.push((cs - d, nlen));

						if nlen < clen {
							let diff = clen - nlen;
							let _ = replace(&mut seeds[i], (cs + nlen, diff));
						} else {
							seeds.remove(i);
						}
					} else if last >= src {
						let offset = src - cs;
						let nlen = len.min(clen - offset);
						res.push((cs + offset - d, nlen));
						let _ = replace(&mut seeds[i], (cs, offset));
						if last >= max_src {
							seeds.insert(i + 1, (cs + offset + nlen, clen - nlen - offset));
						}
					}
				}
			}

			res.append(&mut seeds);

			res
		})
		.into_iter()
		.map(|(s, _)| s)
		.min()
		.unwrap();

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 35,
	part2 => (EX_INPUT) 46
}

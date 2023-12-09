use aoc_lib::{aoc, color_eyre::eyre::Result, iter::IterExt, to_lines};

static INPUT: &str = include_str!("../../inputs/day09");

fn extrapolate(d: impl Iterator<Item = i64>) -> i64 {
	let mut p = d.peekable2();

	let drv: Vec<_> = std::iter::from_fn(|| match p.peek_pair() {
		(Some(&a), Some(&b)) => {
			p.next();
			Some(b - a)
		}
		_ => None,
	})
	.collect();

	if drv.iter().all(|e| *e == 0) {
		p.next().unwrap()
	} else {
		let last = p.next().unwrap();
		last + extrapolate(drv.into_iter())
	}
}

fn part1(input: &str) -> Result<i64> {
	let res = to_lines(input)
		.map(|line| {
			let nums = line.split_ascii_whitespace().map(|n| n.parse().unwrap());
			extrapolate(nums)
		})
		.sum();

	Ok(res)
}

fn part2(input: &str) -> Result<i64> {
	let res = to_lines(input)
		.map(|line| {
			let nums = line.split_ascii_whitespace().map(|n| n.parse().unwrap());
			extrapolate(nums.rev())
		})
		.sum();

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 114,
	part2 => (EX_INPUT) 2
}

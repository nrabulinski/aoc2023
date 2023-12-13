use aoc_lib::{aoc, color_eyre::eyre::Result};

static INPUT: &str = include_str!("../../inputs/day13");

fn part1(input: &str) -> Result<usize> {
	let input = input.trim();
	let res = input
		.split("\n\n")
		.map(|group| {
			let group = group.trim();
			let line_width = group.trim().lines().next().unwrap().len() + 1;
			let height = group.trim().len() / line_width + 1;
			let width = line_width - 1;

			let mut reflection_x = 0;
			let mut reflection_y = 0;

			'outer: for x in 1..width {
				for y in 0..height {
					let y_idx = y * line_width;
					let left = &group[y_idx..y_idx + x];
					let right = &group[y_idx + x..y_idx + width];
					if left
						.as_bytes()
						.iter()
						.rev()
						.zip(right.as_bytes().iter())
						.any(|(a, b)| a != b)
					{
						continue 'outer;
					}
				}
				reflection_x = x;
				break;
			}

			'outer: for y in 1..height {
				for x in 0..width {
					let above = (0..y)
						.map(|y| {
							let idx = y * line_width + x;
							group.as_bytes()[idx]
						})
						.rev();
					let below = (y..height).map(|y| {
						let idx = y * line_width + x;
						group.as_bytes()[idx]
					});
					if above.zip(below).any(|(a, b)| a != b) {
						continue 'outer;
					}
				}
				reflection_y = y;
				break;
			}

			reflection_x + 100 * (reflection_y)
		})
		.sum();

	Ok(res)
}

fn part2(input: &str) -> Result<usize> {
	let input = input.trim();
	let res = input
		.split("\n\n")
		.map(|group| {
			let group = group.trim();
			let line_width = group.trim().lines().next().unwrap().len() + 1;
			let height = group.trim().len() / line_width + 1;
			let width = line_width - 1;

			let mut reflection_x = 0;
			let mut reflection_y = 0;

			'outer: for x in 1..width {
				let mut d = 0;
				for y in 0..height {
					let y_idx = y * line_width;
					let left = &group[y_idx..y_idx + x];
					let right = &group[y_idx + x..y_idx + width];
					d += left
						.as_bytes()
						.iter()
						.rev()
						.zip(right.as_bytes().iter())
						.filter(|(a, b)| a != b)
						.count();
					if d > 1 {
						continue 'outer;
					}
				}
				if d == 1 {
					reflection_x = x;
					break;
				}
			}

			'outer: for y in 1..height {
				let mut d = 0;
				for x in 0..width {
					let above = (0..y)
						.map(|y| {
							let idx = y * line_width + x;
							group.as_bytes()[idx]
						})
						.rev();
					let below = (y..height).map(|y| {
						let idx = y * line_width + x;
						group.as_bytes()[idx]
					});
					d += above.zip(below).filter(|(a, b)| a != b).count();
					if d > 1 {
						continue 'outer;
					}
				}
				if d == 1 {
					reflection_y = y;
					break;
				}
			}

			reflection_x + 100 * (reflection_y)
		})
		.sum();

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 405,
	part2 => (EX_INPUT) 400
}

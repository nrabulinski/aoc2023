use aoc_lib::{aoc, color_eyre::eyre::Result, grid::Grid, iter::IterExt};

static INPUT: &str = include_str!("../../inputs/day13");

fn solve(input: &str, allowed_smudges: usize) -> i64 {
	input
		.trim()
		.split("\n\n")
		.map(|group| {
			let group = Grid::for_str(group).unwrap();

			let reflection_x = (1..group.width())
				.find(|&x| {
					let x = x as usize;
					group
						.iter_rows()
						.map(|row| {
							let left = row.iter().take(x);
							let right = row.iter().skip(x);

							left.rev().zip(right).filter(|(a, b)| a != b)
						})
						.flatten()
						.has_n(allowed_smudges)
				})
				.unwrap_or(0);

			let reflection_y = (1..group.height())
				.find(|&y| {
					let y = y as usize;
					group
						.iter_columns()
						.map(|column| {
							let above = column.iter().take(y);
							let below = column.iter().skip(y);

							above.rev().zip(below).filter(|(a, b)| a != b)
						})
						.flatten()
						.has_n(allowed_smudges)
				})
				.unwrap_or(0);

			reflection_x + 100 * (reflection_y)
		})
		.sum()
}

fn part1(input: &str) -> Result<i64> {
	let res = solve(input, 0);
	Ok(res)
}

fn part2(input: &str) -> Result<i64> {
	let res = solve(input, 1);
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

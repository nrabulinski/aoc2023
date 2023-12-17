use aoc_lib::{
	aoc,
	color_eyre::eyre::Result,
	grid::{Grid, PointExt},
};

static INPUT: &str = include_str!("../../inputs/day17");

fn solve(input: &str, min_moves: i64, max_moves: i64) -> i64 {
	let grid = Grid::for_str(input).unwrap();

	let start = (0, 0);
	let end = (grid.width() - 1, grid.height() - 1);

	let get_val = |pos| grid.get_pos(pos).map(|&v| (v - b'0') as i64);

	let neighbors = |&((x, y), (dx, dy)): &_| {
		let horizontal = (dx == 0)
			.then(move || {
				(min_moves..=max_moves)
					.flat_map(move |dx| [((x + dx, y), (dx, 0)), ((x - dx, y), (-dx, 0))])
			})
			.into_iter()
			.flatten();
		let vertical = (dy == 0)
			.then(move || {
				(min_moves..=max_moves)
					.flat_map(move |dy| [((x, y + dy), (0, dy)), ((x, y - dy), (0, -dy))])
			})
			.into_iter()
			.flatten();

		horizontal.chain(vertical).filter_map(move |(pos, d)| {
			let diff = d.map(i64::signum);
			let cost =
				std::iter::successors(Some(diff), |prev| (*prev != d).then_some(diff.add(prev)))
					.map(|d| get_val(d.add(&(x, y))))
					.try_fold(0, |acc, curr| Some(acc + curr?))?;
			Some(((pos, diff), cost))
		})
	};

	let (dist, _) = aoc_lib::algo::dijkstra((start, (0, 0)), neighbors);

	dist.iter()
		.filter_map(|(&(p, _), &d)| (p == end).then_some(d))
		.min()
		.unwrap()
}

fn part1(input: &str) -> Result<i64> {
	let res = solve(input, 1, 3);

	Ok(res)
}

fn part2(input: &str) -> Result<i64> {
	let res = solve(input, 4, 10);

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 102,
	part2 => (EX_INPUT) 94
}

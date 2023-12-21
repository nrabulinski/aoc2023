use std::collections::HashSet;

use aoc_lib::{aoc, color_eyre::eyre::Result, grid::Grid};

static INPUT: &str = include_str!("../../inputs/day21");

fn part1(input: &str) -> Result<usize> {
	#[cfg(test)]
	const TARGET_STEPS: usize = 6;
	#[cfg(not(test))]
	const TARGET_STEPS: usize = 64;

	let grid = Grid::for_str(input).unwrap();

	let start_idx = input.trim().find('S').unwrap();
	let start_pos = grid.idx_to_pos(start_idx).unwrap();

	let (dist, _) = aoc_lib::algo::dijkstra(start_pos, |&pos| {
		grid.orthogonal_pos(pos)
			.filter(|&pos| grid[pos] != b'#')
			.map(|pos| (pos, 1))
	});

	let modulo = TARGET_STEPS & 1;
	let res = dist
		.into_values()
		.filter(|&d| d <= TARGET_STEPS && d & 1 == modulo)
		.count();

	Ok(res)
}

// Yes, this solution is suboptimal.
// Yes, I do not care in the slightest.
//
// To understand this solution we first need to notice that the path to the center
// is completely unobscured.
// Wait, center? Yes, the starting position is also perfectly centered
// (which is what makes this particular solution possible).
// Basically, because the view is unobscured, and because the start is in the center,
// every time we reach a multiple of the starting posisiton,
// we will have added to the amount of possible paths we can reach a multiple of our map's size.
// (Oh, right, the map is a square, that also contributes to the symmetry)
//
// With this we can notice that the amount of steps each "loop" adds is constant,
// i.e. the acceleration of the number of steps at any given target step count is equal to
// target_steps % grid_width.
// Let's call target_steps % grid_width n. What I'm doing is taking points at n, n + width, n + width * 2
// And based on that I can extrapolate what the number of reachable steps will be at n + width * K for any K.
// The *actual* solution is a bit more complicated
// since we need to take the fact whether target steps is even or odd into account,
// and it doesn't actually work for all target steps or target steps low enough,
// but at this point I really don't care so I'm finishing this monologue, pushing to github, and going to sleep.
// Goodnight, and hopefully the challenge that's releasing in 5 hours is more fun than whatever this was.
fn part2(input: &str) -> Result<i64> {
	const TARGET_STEPS: i64 = 26501365;
	let grid = Grid::for_str(input).unwrap();

	let start_idx = input.trim().find('S').unwrap();
	let start_pos = grid.idx_to_pos(start_idx).unwrap();

	let pos_mod = |(x, y)| {
		let x = x % grid.width();
		let x = if x < 0 { x + grid.width() } else { x };
		let y = y % grid.height();
		let y = if y < 0 { y + grid.height() } else { y };
		(x, y)
	};

	let mut res = 0;
	let modulo = TARGET_STEPS & 1;

	let mut prev_points = HashSet::new();
	let mut curr_points = HashSet::new();
	curr_points.insert(start_pos);
	let mut starts_reached = 0;
	let mut trends = Vec::new();

	for step in 0.. {
		if step & 1 == modulo {
			res += curr_points.len() as i64;
			trends.push(res);
			if curr_points.iter().any(|&pos| pos_mod(pos) == start_pos) {
				if starts_reached == 3 {
					break;
				}
				starts_reached += 1;
			}
		}

		let next_points = curr_points
			.iter()
			.flat_map(|&(x, y)| {
				let h = [-1, 1].into_iter().map(move |dx| (x + dx, y));
				let v = [-1, 1].into_iter().map(move |dy| (x, y + dy));

				h.chain(v)
			})
			.filter(|pos| !prev_points.contains(pos) && grid[pos_mod(*pos)] != b'#')
			.collect();

		prev_points = curr_points;
		curr_points = next_points;
	}

	let steps_to_start = grid.width();

	let need = TARGET_STEPS % steps_to_start;
	let need = if need >= steps_to_start / 2 {
		need - steps_to_start
	} else {
		need
	};
	let times = TARGET_STEPS / steps_to_start / 2;
	let steps_at_nth = |n| {
		let idx = (steps_to_start * n + need) / 2;
		trends[idx as usize]
	};

	let needed_points: Vec<_> = (0..3).map(|i| steps_at_nth(i * 2 + modulo)).collect();
	let d: Vec<_> = needed_points.windows(2).map(|d| d[1] - d[0]).collect();
	let dd = d[1] - d[0];
	let res = (0..times).map(|i| d[0] + dd * i).sum::<i64>() + needed_points[0];

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 16,
	part2 => (INPUT) 616583483179597
}

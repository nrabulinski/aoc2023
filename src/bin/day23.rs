use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_lib::{
	aoc,
	color_eyre::eyre::Result,
	grid::{Grid, Point},
	iter::IterExt,
};

static INPUT: &str = include_str!("../../inputs/day23");

fn part1(input: &str) -> Result<i64> {
	let grid = Grid::for_str(input).unwrap();
	let mut dist = HashMap::new();
	let mut queue = BinaryHeap::new();

	let start = (1, 0);
	let end = (grid.width() - 2, grid.height() - 1);

	dist.insert(start, 0);
	queue.push((0, start, start));

	let valid_step = |step_from: Point, step_to: Point| match grid[step_from] {
		b'>' => step_from.0 < step_to.0,
		b'<' => step_from.0 > step_to.0,
		b'v' => step_from.1 < step_to.1,
		b'^' => step_from.1 > step_to.1,
		_ => true,
	};

	while let Some((d, curr, came_from)) = queue.pop() {
		let neighbors = grid
			.orthogonal_pos(curr)
			.filter(|&pos| grid[pos] != b'#' && pos != came_from && valid_step(curr, pos));
		for neighbor in neighbors {
			let d = d + 1;
			if dist.get(&neighbor).map(|curr| curr < &d).unwrap_or(true) {
				queue.push((d, neighbor, curr));
				dist.insert(neighbor, d);
			}
		}
	}

	Ok(*dist.get(&end).unwrap())
}

// This takes forever to finish, I have no clue why
// (though it does finish eveutnaully),
// so I just always print the longest path "so far".
// I then left this running for like a minute, put it in as the answer,
// and it turned out to be correct.
fn part2(input: &str) -> Result<i64> {
	let grid = Grid::for_str(input).unwrap();

	let start = (1, 0);
	let end = (grid.width() - 2, grid.height() - 1);
	let mut largest = 0;

	fn dfs(
		mut curr: Point,
		mut d: i64,
		came_from: &HashSet<Point>,
		grid: Grid<'_>,
		looking_for: Point,
		largest: &mut i64,
	) -> i64 {
		let mut came_from = came_from.clone();
		loop {
			if curr == looking_for {
				if d < *largest {
					return -1;
				} else {
					*largest = d;
					println!("got here: {d}");
					return d;
				}
			}
			came_from.insert(curr);

			let mut neighbors = grid
				.orthogonal_pos(curr)
				.filter(|pos| !came_from.contains(pos) && grid[*pos] != b'#')
				.peekable2();

			match neighbors.peek_pair() {
				(None, None) => return -1,
				(Some(_), Some(_)) => {
					let res = neighbors
						.map(|neighbor| {
							dfs(neighbor, d + 1, &came_from, grid, looking_for, largest)
						})
						.max()
						.unwrap();
					if res > *largest {
						*largest = res;
						println!("got here: {res}");
					}
					return res;
				}
				(Some(&neighbor), None) => {
					curr = neighbor;
					d += 1
				}
				_ => unreachable!(),
			}
		}
	}

	Ok(dfs(start, 0, &HashSet::new(), grid, end, &mut largest))
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 94,
	part2 => (EX_INPUT) 154
}

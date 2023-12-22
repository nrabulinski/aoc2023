use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day22");

type Vec3 = (i64, i64, i64);

fn parse(s: &str) -> Vec3 {
	let mut i = s.split(',').map(|n| n.parse().unwrap());
	(i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
}

fn overlap(a: (i64, i64), b: (i64, i64)) -> bool {
	// a starts in b but goes out
	let a_in_b = a.0 >= b.0 && a.0 <= b.1;

	// b starts in a but goes out
	let b_in_a = b.0 >= a.0 && b.0 <= a.1;

	a_in_b || b_in_a
}

fn collides(a: (Vec3, Vec3), b: (Vec3, Vec3)) -> bool {
	let ax = (a.0 .0, a.1 .0);
	let ay = (a.0 .1, a.1 .1);
	let az = (a.0 .2, a.1 .2);
	let bx = (b.0 .0, b.1 .0);
	let by = (b.0 .1, b.1 .1);
	let bz = (b.0 .2, b.1 .2);

	overlap(ax, bx) && overlap(ay, by) && overlap(az, bz)
}

fn lower((from, to): (Vec3, Vec3)) -> (Vec3, Vec3) {
	let from = (from.0, from.1, from.2 - 1);
	let to = (to.0, to.1, to.2 - 1);
	(from, to)
}

fn get_blocks(input: &str) -> (Vec<Vec<usize>>, usize) {
	let mut blocks: Vec<_> = to_lines(input)
		.map(|line| {
			let (from, to) = line.split_once('~').unwrap();
			(parse(from), parse(to))
		})
		.collect();

	blocks.sort_unstable_by_key(|(from, _)| from.2);

	let mut supported_by = Vec::new();

	for i in 0..blocks.len() {
		loop {
			let maybe = lower(blocks[i]);
			let collisions: Vec<_> = blocks
				.iter()
				.enumerate()
				.take(i)
				.filter_map(|(i, &other)| collides(maybe, other).then_some(i))
				.collect();

			if blocks[i].0 .2 > 1 && collisions.is_empty() {
				blocks[i] = maybe;
			} else {
				supported_by.push(collisions);
				break;
			}
		}
	}

	(supported_by, blocks.len())
}

fn part1(input: &str) -> Result<usize> {
	let (supported_by, block_cnt) = get_blocks(input);

	// for each block make sure that blocks that depend on it have more than one supporter
	let res = (0..block_cnt)
		.filter(|&i| {
			supported_by
				.iter()
				.skip(i + 1)
				.filter(|sup| sup.contains(&i))
				.all(|sup| sup.len() > 1)
		})
		.count();

	Ok(res)
}

fn part2(input: &str) -> Result<usize> {
	let (supported_by, block_cnt) = get_blocks(input);

	// for each block count how many would fall if it was removed,
	// and then how many would fall if those were also removed,
	// and then how many would fall if those were also removed,
	// ...
	let res = (0..block_cnt)
		.map(|i| {
			supported_by
				.iter()
				.enumerate()
				.skip(i + 1)
				.fold(vec![i], |mut fallen, (j, sup)| {
					if !sup.is_empty() && sup.iter().all(|s| fallen.contains(s)) {
						fallen.push(j);
					}
					fallen
				})
				.len() - 1
		})
		.sum();

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 5,
	part2 => (EX_INPUT) 7
}

use std::collections::HashMap;

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day12");

// God bless @deliveroo
// Wouldn't have solved this day without him
// VERY heavily based on https://github.com/MarekSosnicki-deliveroo/AOC_2023/blob/master/examples/day_12_p2.rs
fn test_str(s: &str, groups: &[usize]) -> usize {
	fn recur(
		s: &[u8],
		groups: &[usize],
		idx: usize,
		mut group_idx: usize,
		mut group_sz: usize,
		cache: &mut HashMap<(usize, usize, usize), usize>,
	) -> usize {
		if let Some(&opts) = cache.get(&(idx, group_idx, group_sz)) {
			return opts;
		}
		for idx in idx..s.len() {
			match s[idx] {
				b'.' => {
					if group_sz > 0 {
						if groups.get(group_idx) != Some(&group_sz) {
							return 0;
						}
						group_idx += 1;
					}
					group_sz = 0;
				}
				b'#' => {
					group_sz += 1;
					if let Some(&expected_sz) = groups.get(group_idx) {
						if group_sz > expected_sz {
							return 0;
						}
					} else {
						return 0;
					}
				}
				b'?' => {
					// If current group size is 0 (i.e. we haven't started a group yet),
					// continue with the current group index.
					let new_group_idx = (group_sz == 0).then_some(group_idx).or_else(|| {
						// Otherwise...
						groups
							.get(group_idx)
							// ...if the current group has reached its desired size yet...
							.filter(|&&sz| sz == group_sz)
							// ...continue to the next group.
							.map(|_| group_idx + 1)
						// If it hasn't, we don't need to try replacing the current ? with .
					});
					let a = new_group_idx
						.map(|group_idx| recur(s, groups, idx + 1, group_idx, 0, cache))
						.unwrap_or(0);
					let b = recur(s, groups, idx + 1, group_idx, group_sz + 1, cache);
					let res = a + b;
					cache.insert((idx, group_idx, group_sz), res);
					return res;
				}
				_ => unreachable!(),
			}
		}

		if
		// we are in a group
		group_sz > 0
		// the current group is the last one
		&& group_idx == groups.len() - 1
		// it is of the expected size
		&& groups[group_idx] == group_sz
		// or we aren't in a group and we have the corrent amount of groups
		|| group_sz == 0 && group_idx == groups.len()
		{
			// That means we're in a corrent permutation
			1
		} else {
			0
		}
	}

	let mut cache = HashMap::new();
	recur(s.as_bytes(), groups, 0, 0, 0, &mut cache)
}

fn part1(input: &str) -> Result<usize> {
	let s = std::time::Instant::now();
	let res = to_lines(input)
		.map(|line| {
			let (chars, nums) = line.split_once(' ').unwrap();
			let nums: Vec<usize> = nums.split(',').map(|n| n.parse().unwrap()).collect();
			test_str(chars, &nums)
		})
		.sum();
	println!("part 1 took {}ms", s.elapsed().as_millis());
	Ok(res)
}

fn part2(input: &str) -> Result<usize> {
	let s = std::time::Instant::now();
	let res = to_lines(input)
		.map(|line| {
			let (c, nums) = line.split_once(' ').unwrap();
			let mut nums: Vec<usize> = nums.split(',').map(|n| n.parse().unwrap()).collect();
			let n = nums.clone();
			let mut chars = c.to_string();
			for _ in 0..4 {
				nums.extend_from_slice(&n);
				chars.push('?');
				chars.push_str(c);
			}
			test_str(&chars, &nums)
		})
		.sum();
	println!("part 2 took {}ms", s.elapsed().as_millis());
	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 21,
	part2 => (EX_INPUT) 525152
}

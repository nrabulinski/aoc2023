use std::{
	collections::{HashMap, VecDeque},
	ops::BitOr,
};

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day20");

const NONEXISTENT_MODULE: usize = 9999;
const RX_MODULE: usize = 99999;

#[derive(Debug, Clone, Copy)]
enum Mod {
	Broadcaster,
	FlipFlop,
	Conjunction(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
	Low,
	High,
}

fn parse(input: &str) -> (Vec<(Mod, Vec<usize>, Vec<usize>)>, usize, usize) {
	let map: HashMap<_, _> = to_lines(input)
		.enumerate()
		.map(|(i, line)| {
			let (from, to) = line.split_once(" -> ").unwrap();
			let targets: Vec<_> = to.split(", ").collect();
			(&from[1..], (i, targets))
		})
		.collect();

	assert!(map.len() <= 64);

	let mut res = Vec::with_capacity(map.len());
	let mut start_idx = 0;
	let mut rx_idx = 0;

	for line in to_lines(input) {
		let (name, _) = line.split_once(" -> ").unwrap();

		let targets: Vec<_> = map
			.get(&name[1..])
			.unwrap()
			.1
			.iter()
			.map(|target| {
				if *target == "rx" {
					rx_idx = res.len();
					RX_MODULE
				} else {
					map.get(target)
						.map(|target| target.0)
						.unwrap_or(NONEXISTENT_MODULE)
				}
			})
			.collect();

		let typ = if name == "broadcaster" {
			start_idx = res.len();
			Mod::Broadcaster
		} else {
			match name.as_bytes()[0] {
				b'%' => Mod::FlipFlop,
				b'&' => Mod::Conjunction(0),
				_ => unreachable!(),
			}
		};

		res.push((typ, Vec::new(), targets));
	}

	// Plug all inputs into their outputs so the outputs remember where the pulses are coming from.
	// This whole ordeal is so that the borrow checker doesn't complain about mutating `res`
	// while it's being borrowed for the iteration.
	// Yes, it could be done better, no I do not care.
	for i in 0..res.len() {
		for target_idx in 0..res[i].2.len() {
			let j = res[i].2[target_idx];
			if matches!(j, NONEXISTENT_MODULE | RX_MODULE) {
				continue;
			}
			res[j].1.push(i);
		}
	}

	(res, start_idx, rx_idx)
}

fn part1(input: &str) -> Result<u64> {
	let (mut arr, start_idx, _) = parse(input);
	let mut state = 0u64;
	let mut total_low = 0;
	let mut total_high = 0;

	for _ in 0..1000 {
		let mut queue = VecDeque::new();
		queue.push_back((start_idx, Pulse::Low, NONEXISTENT_MODULE));

		while let Some((idx, pulse, from)) = queue.pop_front() {
			match pulse {
				Pulse::Low => total_low += 1,
				Pulse::High => total_high += 1,
			}
			if matches!(idx, NONEXISTENT_MODULE | RX_MODULE) {
				continue;
			}
			let (typ, inputs, outputs) = &mut arr[idx];

			match typ {
				Mod::Broadcaster => outputs
					.iter()
					.for_each(|&target_idx| queue.push_back((target_idx, pulse, idx))),
				Mod::FlipFlop => {
					if pulse == Pulse::High {
						continue;
					}
					let mask = 1 << idx;
					state ^= mask;
					let is_off = state & mask == 0;
					outputs.iter().for_each(|&target_idx| {
						queue.push_back((
							target_idx,
							if is_off { Pulse::Low } else { Pulse::High },
							idx,
						))
					});
				}
				Mod::Conjunction(ref mut mem) => {
					let all_inputs = inputs.iter().map(|&idx| 1 << idx).fold(0, u64::bitor);
					let mask = 1 << from;
					if pulse == Pulse::High {
						*mem |= mask;
					} else {
						*mem &= !mask;
					}
					let all_were_on = *mem == all_inputs;
					outputs.iter().for_each(|&target_idx| {
						queue.push_back((
							target_idx,
							if all_were_on { Pulse::Low } else { Pulse::High },
							idx,
						))
					});
				}
			}
		}
	}

	Ok(total_low * total_high)
}

fn part2(input: &str) -> Result<usize> {
	let (arr, start_idx, rx_idx) = parse(input);

	// This only works because we need to notice that those inputs don't depend on each other in any way.
	// I wasn't smart enough to figure this out myself so thanks to @tumdum and @zsacul.
	let res = arr[rx_idx]
		.1
		.iter()
		.map(|&input| {
			let mut arr = arr.clone();
			let mut state = 0u64;

			for i in 1.. {
				let mut queue = VecDeque::new();
				queue.push_back((start_idx, Pulse::Low, NONEXISTENT_MODULE));

				while let Some((idx, pulse, from)) = queue.pop_front() {
					if matches!(idx, NONEXISTENT_MODULE | RX_MODULE) {
						continue;
					}
					if idx == rx_idx && from == input && pulse == Pulse::High {
						return i;
					}
					let (typ, inputs, outputs) = &mut arr[idx];

					match typ {
						Mod::Broadcaster => outputs
							.iter()
							.for_each(|&target_idx| queue.push_back((target_idx, pulse, idx))),
						Mod::FlipFlop => {
							if pulse == Pulse::High {
								continue;
							}
							let mask = 1 << idx;
							state ^= mask;
							let is_off = state & mask == 0;
							outputs.iter().for_each(|&target_idx| {
								queue.push_back((
									target_idx,
									if is_off { Pulse::Low } else { Pulse::High },
									idx,
								))
							});
						}
						Mod::Conjunction(ref mut mem) => {
							let all_inputs = inputs.iter().map(|&idx| 1 << idx).fold(0, u64::bitor);
							let mask = 1 << from;
							if pulse == Pulse::High {
								*mem |= mask;
							} else {
								*mem &= !mask;
							}
							let all_were_on = *mem == all_inputs;
							outputs.iter().for_each(|&target_idx| {
								queue.push_back((
									target_idx,
									if all_were_on { Pulse::Low } else { Pulse::High },
									idx,
								))
							});
						}
					}
				}
			}
			unreachable!()
		})
		.fold(1, lcm);

	fn gcd(a: usize, b: usize) -> usize {
		if b == 0 {
			a
		} else {
			gcd(b, a % b)
		}
	}

	fn lcm(a: usize, b: usize) -> usize {
		a * b / gcd(a, b)
	}

	Ok(res)
}

// TODO: Multiple examples in aoc-lib macro
#[allow(dead_code)]
static EX_INPUT_1: &str = r#"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;
#[allow(dead_code)]
const EX_INPUT_1_SOL: u64 = 32000000;

#[allow(dead_code)]
static EX_INPUT_2: &str = r#"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;
#[allow(dead_code)]
const EX_INPUT_2_SOL: u64 = 11687500;

aoc! {
	INPUT:
	part1 => (EX_INPUT_2) crate::EX_INPUT_2_SOL,
	part2 => (INPUT) 244465191362269
}

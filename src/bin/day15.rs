use aoc_lib::{aoc, color_eyre::eyre::Result};

static INPUT: &str = include_str!("../../inputs/day15");

fn part1(input: &str) -> Result<u64> {
	let res = input
		.trim()
		.split(',')
		.map(|chars| {
			chars
				.as_bytes()
				.iter()
				.fold(0u64, |acc, &curr| ((acc + curr as u64) * 17) % 256)
		})
		.sum();

	Ok(res)
}

fn part2(input: &str) -> Result<usize> {
	const NEW_VEC: Vec<(Vec<u8>, usize)> = Vec::new();
	let mut b = [NEW_VEC; 256];

	for chars in input.trim().split(',') {
		let mut chars = chars.as_bytes().iter().peekable();
		let label: Vec<_> = std::iter::from_fn(|| chars.next_if(|x| x.is_ascii_alphabetic()))
			.copied()
			.collect();
		let idx = label
			.iter()
			.fold(0u8, |acc, &curr| acc.wrapping_add(curr).wrapping_mul(17)) as usize;

		match chars.next().unwrap() {
			b'-' => {
				if let Some(inner_idx) = b[idx].iter().position(|(v, _)| v == &label) {
					b[idx].remove(inner_idx);
				}
			}
			b'=' => {
				let val = chars.fold(0, |acc, &curr| acc * 10 + (curr - b'0') as usize);
				if let Some(inner_idx) = b[idx].iter().position(|(v, _)| v == &label) {
					b[idx][inner_idx].1 = val;
				} else {
					b[idx].push((label, val));
				}
			}
			_ => unreachable!(),
		}
	}

	let res = b
		.iter()
		.enumerate()
		.map(|(i, b)| {
			b.iter()
				.enumerate()
				.map(|(j, (_, v))| (i + 1) * (j + 1) * v)
				.sum::<usize>()
		})
		.sum();

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 1320,
	part2 => (EX_INPUT) 145
}

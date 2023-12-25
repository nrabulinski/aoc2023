use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};
use z3::{
	ast::{Ast, Int},
	Config, Context, SatResult, Solver,
};

static INPUT: &str = include_str!("../../inputs/day24");

#[derive(Debug, Clone, Copy)]
struct Line {
	end: bool,
	vertex: (f64, f64),
	a: f64,
	b: f64,
}

impl Line {
	fn goes_to_x(&self, x: f64) -> bool {
		if self.end {
			x <= self.vertex.0
		} else {
			x >= self.vertex.0
		}
	}
}

fn part1(input: &str) -> Result<i64> {
	#[cfg(test)]
	const MIN_POS: f64 = 7.;
	#[cfg(test)]
	const MAX_POS: f64 = 27.;

	#[cfg(not(test))]
	const MIN_POS: f64 = 200000000000000.;
	#[cfg(not(test))]
	const MAX_POS: f64 = 400000000000000.;

	fn parse(s: &str) -> (f64, f64) {
		let mut i = s.split(", ");
		(
			i.next().unwrap().parse().unwrap(),
			i.next().unwrap().parse().unwrap(),
		)
	}

	let lines: Vec<_> = to_lines(input)
		.map(|line| {
			let (pos, vel) = line.split_once(" @ ").unwrap();
			let (x, y) = parse(pos);
			let (dx, dy) = parse(vel);
			let a = dy / dx;
			let b = y - a * x;
			let end = dx < 0.;
			Line {
				end,
				vertex: (x, y),
				a,
				b,
			}
		})
		.collect();

	let mut res = 0;

	for i in 0..lines.len() - 1 {
		let a = lines[i];
		for j in i + 1..lines.len() {
			let b = lines[j];

			if a.a == b.a {
				if a.b == b.b {
					todo!()
				}
			} else {
				let range = MIN_POS..=MAX_POS;
				let x = (b.b - a.b) / (a.a - b.a);
				let y = a.a * x + a.b;
				if range.contains(&x) && range.contains(&y) && a.goes_to_x(x) && b.goes_to_x(x) {
					res += 1;
				}
			}
		}
	}

	Ok(res)
}

fn part2(input: &str) -> Result<i64> {
	fn parse(s: &str) -> (i64, i64, i64) {
		let mut i = s.split(", ");
		(
			i.next().unwrap().parse().unwrap(),
			i.next().unwrap().parse().unwrap(),
			i.next().unwrap().parse().unwrap(),
		)
	}

	let cfg = Config::new();
	let ctx = Context::new(&cfg);
	let s = Solver::new(&ctx);

	let x = Int::new_const(&ctx, "x");
	let y = Int::new_const(&ctx, "y");
	let z = Int::new_const(&ctx, "z");
	let dx = Int::new_const(&ctx, "dx");
	let dy = Int::new_const(&ctx, "dy");
	let dz = Int::new_const(&ctx, "dz");

	for (i, line) in to_lines(input).enumerate() {
		let (pos, vel) = line.split_once(" @ ").unwrap();
		let pos = parse(pos);
		let vel = parse(vel);

		let t = Int::new_const(&ctx, format!("t_{i}").as_str());

		s.assert(&(&x + &dx * &t)._eq(&(pos.0 + vel.0 * &t)));
		s.assert(&(&y + &dy * &t)._eq(&(pos.1 + vel.1 * &t)));
		s.assert(&(&z + &dz * &t)._eq(&(pos.2 + vel.2 * &t)));
	}

	assert_eq!(s.check(), SatResult::Sat);

	let m = s.get_model().unwrap();
	let x = m.get_const_interp(&x).unwrap().as_i64().unwrap();
	let y = m.get_const_interp(&y).unwrap().as_i64().unwrap();
	let z = m.get_const_interp(&z).unwrap().as_i64().unwrap();

	Ok(x + y + z)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3
"#;

// TODO: Common parsing for both parts
aoc! {
	INPUT:
	part1 => (EX_INPUT) 2,
	part2 => (EX_INPUT) 47
}

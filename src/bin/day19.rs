use std::{
	collections::{HashMap, VecDeque},
	str::FromStr,
};

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day19");

#[derive(Debug, Clone)]
enum Res {
	Accept,
	Reject,
	Send(String),
}

impl Res {
	fn for_str(s: &str) -> Res {
		match s {
			"A" => Res::Accept,
			"R" => Res::Reject,
			s => Res::Send(s.to_string()),
		}
	}
}

#[derive(Debug, Clone, Copy)]
struct ValRange {
	gt: u64,
	lt: u64,
}

impl ValRange {
	const MIN: u64 = 1;
	const MAX: u64 = 4000;

	fn new_less_than(val: u64) -> Self {
		Self {
			lt: val,
			..Self::default()
		}
	}

	fn new_greater_than(val: u64) -> Self {
		Self {
			gt: val,
			..Self::default()
		}
	}

	fn combine(&self, other: &Self) -> Option<ValRange> {
		let gt = self.gt.max(other.gt);
		let lt = self.lt.min(other.lt);
		if gt >= lt {
			None
		} else {
			Some(ValRange { gt, lt })
		}
	}

	fn count(&self) -> u64 {
		self.lt - self.gt - 1
	}

	fn value(&self) -> u64 {
		assert_eq!(self.gt + 1, self.lt - 1);
		self.gt + 1
	}

	fn reverse(&self) -> Self {
		let d = Self::default();
		let gt = if self.lt == d.lt { d.gt } else { self.lt - 1 };
		let lt = if self.gt == d.gt { d.lt } else { self.gt + 1 };
		ValRange { gt, lt }
	}
}

impl Default for ValRange {
	fn default() -> Self {
		ValRange {
			gt: Self::MIN - 1,
			lt: Self::MAX + 1,
		}
	}
}

#[derive(Debug, Clone, Copy, Default)]
struct Obj {
	x: ValRange,
	m: ValRange,
	a: ValRange,
	s: ValRange,
}

impl Obj {
	fn combine_prop(self, prop: u8, range: ValRange) -> Option<Obj> {
		Some(match prop {
			b'x' => Obj {
				x: self.x.combine(&range)?,
				..self
			},
			b'm' => Obj {
				m: self.m.combine(&range)?,
				..self
			},
			b'a' => Obj {
				a: self.a.combine(&range)?,
				..self
			},
			b's' => Obj {
				s: self.s.combine(&range)?,
				..self
			},
			_ => unreachable!(),
		})
	}
}

impl FromStr for Obj {
	type Err = ();

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		let mut res = Obj::default();
		let line = s.trim();

		for prop in line[1..line.len() - 1].split(',') {
			let (name, val) = prop.split_once('=').unwrap();
			let name = name.as_bytes()[0];
			let val: u64 = val.parse().unwrap();
			let range = ValRange {
				gt: val - 1,
				lt: val + 1,
			};
			res = res.combine_prop(name, range).unwrap();
		}

		Ok(res)
	}
}

type RangeWithTarget = (Option<(u8, ValRange)>, Res);
fn parse_ins(line: &str) -> (String, Vec<RangeWithTarget>) {
	let line = line.trim();
	let idx = line.find('{').unwrap();
	let name = &line[..idx];
	let ins = &line[idx + 1..line.len() - 1];

	let ins: Vec<_> = ins
		.split(',')
		.map(|instr| {
			if let Some(idx) = instr.find(':') {
				let res = Res::for_str(&instr[idx + 1..]);
				let cmd = &instr[..idx];

				let f = if let Some((prop, val)) = cmd.split_once('>') {
					let prop = prop.as_bytes()[0];
					let val = val.parse().unwrap();
					(prop, ValRange::new_greater_than(val))
				} else if let Some((prop, val)) = cmd.split_once('<') {
					let prop = prop.as_bytes()[0];
					let val = val.parse().unwrap();
					(prop, ValRange::new_less_than(val))
				} else {
					unreachable!()
				};
				(Some(f), res)
			} else {
				let res = Res::for_str(instr);
				(None, res)
			}
		})
		.collect();

	(name.to_string(), ins)
}

fn part1(input: &str) -> Result<u64> {
	let (ins, objs) = {
		let mut i = input.trim().split("\n\n");
		(i.next().unwrap(), i.next().unwrap())
	};

	let ins: HashMap<_, _> = to_lines(ins).map(parse_ins).collect();

	let res = to_lines(objs)
		.map(Obj::from_str)
		.filter_map(|obj| {
			let obj = obj.unwrap();
			let mut curr = "in";

			'outer: loop {
				let tests = ins.get(curr).unwrap();

				for (test, target) in tests {
					if test
						.map(|(prop, range)| obj.combine_prop(prop, range).is_none())
						.unwrap_or(false)
					{
						continue;
					}
					match target {
						Res::Send(next) => {
							curr = next;
							continue 'outer;
						}
						Res::Accept => {
							return Some(
								obj.x.value() + obj.m.value() + obj.a.value() + obj.s.value(),
							)
						}
						Res::Reject => return None,
					}
				}
				unreachable!()
			}
		})
		.sum();

	Ok(res)
}

fn part2(input: &str) -> Result<u64> {
	let ins = input.trim().split("\n\n").next().unwrap();
	let ins: HashMap<_, _> = to_lines(ins)
		.map(|line| {
			let line = line.trim();
			let idx = line.find('{').unwrap();
			let name = &line[..idx];
			let ins = &line[idx + 1..line.len() - 1];

			let ins: Vec<_> = ins
				.split(',')
				.map(|instr| {
					if let Some(idx) = instr.find(':') {
						let res = Res::for_str(&instr[idx + 1..]);
						let cmd = &instr[..idx];

						let f = if let Some((prop, val)) = cmd.split_once('>') {
							let prop = prop.as_bytes()[0];
							let val = val.parse().unwrap();
							(prop, ValRange::new_greater_than(val))
						} else if let Some((prop, val)) = cmd.split_once('<') {
							let prop = prop.as_bytes()[0];
							let val = val.parse().unwrap();
							(prop, ValRange::new_less_than(val))
						} else {
							unreachable!()
						};
						(Some(f), res)
					} else {
						let res = Res::for_str(instr);
						(None, res)
					}
				})
				.collect();

			(name.to_string(), ins)
		})
		.collect();

	let mut queue = VecDeque::new();
	queue.push_back(("in", Obj::default()));

	let mut res = 0;
	'outer: while let Some((curr_ins, mut obj)) = queue.pop_front() {
		for (ins, target) in ins.get(curr_ins).unwrap() {
			let new_obj = if let Some((prop, range)) = ins {
				let Some(new_obj) = obj.combine_prop(*prop, *range) else {
					continue 'outer;
				};
				let Some(next_obj) = obj.combine_prop(*prop, range.reverse()) else {
					continue 'outer;
				};
				obj = next_obj;
				new_obj
			} else {
				obj
			};

			match target {
				Res::Accept => {
					res += new_obj.x.count()
						* new_obj.m.count() * new_obj.a.count()
						* new_obj.s.count()
				}
				Res::Reject => continue,
				Res::Send(next) => queue.push_back((next.as_str(), new_obj)),
			}
		}
	}

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 19114,
	part2 => (EX_INPUT) 167409079868000
}

use aoc_lib::{aoc, color_eyre::eyre::Result, map_with_idx, to_lines};

static INPUT: &str = include_str!("../../inputs/day02");

fn str_to_game(s: &str) -> [i64; 3] {
	s.split(", ").fold([0, 0, 0], |[r, g, b], curr| {
		let (v, c) = curr.split_once(' ').unwrap();
		if c == "red" {
			[r + v.parse::<i64>().unwrap(), g, b]
		} else if c == "green" {
			[r, g + v.parse::<i64>().unwrap(), b]
		} else if c == "blue" {
			[r, g, b + v.parse::<i64>().unwrap()]
		} else {
			panic!()
		}
	})
}

fn parse_line(line: &str) -> [i64; 3] {
	let (_, games_s) = line.split_once(": ").unwrap();
	let games = games_s.split("; ").map(str_to_game);
	games
		.reduce(|prev, curr| map_with_idx(prev, |i, e| e.max(curr[i])))
		.unwrap()
}

fn part1(input: &str) -> Result<i64> {
	let allowed_cubes = [12, 13, 14];
	let ans = to_lines(input)
		.enumerate()
		.map(|(i, line)| {
			let id = i + 1;
			let cubes_used = parse_line(line);
			(id as i64, cubes_used)
		})
		.filter(|(_, cubes_used)| {
			cubes_used
				.iter()
				.copied()
				.zip(allowed_cubes)
				.all(|(used, allowed)| used <= allowed)
		})
		.map(|(id, _)| id)
		.sum();
	Ok(ans)
}

fn part2(input: &str) -> Result<i64> {
	let ans = to_lines(input)
		.map(parse_line)
		.map(|game| game.into_iter().product::<i64>())
		.sum();
	Ok(ans)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 8,
	part2 => (EX_INPUT) 2286
}

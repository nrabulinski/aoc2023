use std::{collections::HashSet, convert::identity, ops::Range};

use aoc_lib::{aoc, color_eyre::eyre::Result, rangemap::RangeMap, regex::Regex};

static INPUT: &str = include_str!("../../inputs/day03");

fn idx_to_pos(idx: usize, line_len: usize) -> (usize, usize) {
    let x = idx % (line_len + 1);
    let y = idx / (line_len + 1);
    (x, y)
}

fn pos_to_idx(x: usize, y: usize, line_len: usize) -> usize {
    y * (line_len + 1) + x
}

fn part1(input: &str) -> Result<i64> {
    let input = input.trim();
    let line_length = input.lines().next().unwrap().len();
    let line_cnt = input.len() / line_length;
    let re = Regex::new(r"\d+").unwrap();
    let check_symbol = |idx| {
        let b = input.as_bytes()[idx];
        b != b'.' && (b < b'0' || b > b'9')
    };
    let res = re
        .find_iter(input)
        .filter(|m| {
            let Range { start, end } = m.range();
            let start_pos = idx_to_pos(start, line_length);
            let end_pos = idx_to_pos(end - 1, line_length);

            let at_left = start_pos.0 == 0;
            let at_right = end_pos.0 == line_length - 1;
            let at_top = start_pos.1 == 0;
            let at_bot = start_pos.1 == line_cnt - 1;

            let above = (!at_top)
                .then(|| (start_pos.0..=end_pos.0).map(|x| (x, start_pos.1 - 1)))
                .into_iter()
                .flatten();
            let below = (!at_bot)
                .then(|| (start_pos.0..=end_pos.0).map(|x| (x, start_pos.1 + 1)))
                .into_iter()
                .flatten();
            let left = (!at_left).then(|| (start_pos.0 - 1, start_pos.1));
            let right = (!at_right).then(|| (end_pos.0 + 1, end_pos.1));
            let top_left = (!at_top && !at_left).then(|| (start_pos.0 - 1, start_pos.1 - 1));
            let top_right = (!at_top && !at_right).then(|| (end_pos.0 + 1, end_pos.1 - 1));
            let bot_left = (!at_bot && !at_left).then(|| (start_pos.0 - 1, start_pos.1 + 1));
            let bot_right = (!at_bot && !at_right).then(|| (end_pos.0 + 1, end_pos.1 + 1));

            let pos_to_check = above
                .chain(below)
                .chain(left)
                .chain(right)
                .chain(top_left)
                .chain(top_right)
                .chain(bot_left)
                .chain(bot_right);

            pos_to_check
                .map(|(x, y)| pos_to_idx(x, y, line_length))
                .map(check_symbol)
                .any(identity)
        })
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .sum();

    Ok(res)
}

fn part2(input: &str) -> Result<i64> {
    let input = input.trim();
    let line_length = input.lines().next().unwrap().len();
    let line_cnt = input.len() / line_length;
    let re = Regex::new(r"\d+").unwrap();
    let mut num_map: RangeMap<_, i64> = re
        .find_iter(input)
        .map(|m| (m.range(), m.as_str().parse().unwrap()))
        .collect();

    let gears = input
        .as_bytes()
        .into_iter()
        .enumerate()
        .filter(|(_, &b)| b == b'*');

    let mut res = 0;
    for (gear, _) in gears {
        let pos = idx_to_pos(gear, line_length);
        let at_left = pos.0 == 0;
        let at_right = pos.0 == line_length - 1;
        let at_top = pos.1 == 0;
        let at_bot = pos.1 == line_cnt - 1;

        let top = (!at_top).then(|| (pos.0, pos.1 - 1));
        let bot = (!at_bot).then(|| (pos.0, pos.1 + 1));
        let left = (!at_left).then(|| (pos.0 - 1, pos.1));
        let right = (!at_right).then(|| (pos.0 + 1, pos.1));
        let top_left = (!at_top && !at_left).then(|| (pos.0 - 1, pos.1 - 1));
        let top_right = (!at_top && !at_right).then(|| (pos.0 + 1, pos.1 - 1));
        let bot_left = (!at_bot && !at_left).then(|| (pos.0 - 1, pos.1 + 1));
        let bot_right = (!at_bot && !at_right).then(|| (pos.0 + 1, pos.1 + 1));

        let pos_to_check = top
            .into_iter()
            .chain(bot)
            .chain(left)
            .chain(right)
            .chain(top_left)
            .chain(top_right)
            .chain(bot_left)
            .chain(bot_right);

        let part_nums: HashSet<_> = pos_to_check
            .map(|(x, y)| pos_to_idx(x, y, line_length))
            .filter_map(|idx| num_map.get_key_value(&idx))
            .map(|(r, &v)| (r.clone(), v as i64))
            .collect();

        if part_nums.len() != 2 {
            continue;
        }

        let mut ratio = 1;
        for (r, v) in part_nums {
            num_map.remove(r);
            ratio *= v;
        }

        res += ratio;
    }

    Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 4361,
    part2 => (EX_INPUT) 467835
}

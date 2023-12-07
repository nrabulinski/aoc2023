use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day01");

fn map_digit(s: &str, dict: &[&str]) -> Option<i64> {
    dict.into_iter()
        .position(|d| s.starts_with(d))
        .map(|pos| pos as i64 + 1)
}

fn to_digits(line: &str, include_words: bool) -> Vec<i64> {
    const DIGITS: &[&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    const DIGIT_WORDS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    (0..line.len())
        .filter_map(|idx| {
            map_digit(&line[idx..], DIGITS).or(include_words
                .then(|| map_digit(&line[idx..], DIGIT_WORDS))
                .flatten())
        })
        .collect()
}

fn common(input: &str, include_words: bool) -> i64 {
    let lines = to_lines(input);

    lines
        .map(|line| {
            let digits = to_digits(line, include_words);
            let fst = digits.first().expect("line should have at least one digit");
            let lst = digits.last().expect("line should have at least one digit");
            fst * 10 + lst
        })
        .sum()
}

fn part1(input: &str) -> Result<i64> {
    Ok(common(input, false))
}

fn part2(input: &str) -> Result<i64> {
    Ok(common(input, true))
}

#[allow(dead_code)]
static EX_INPUT_1: &str = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
#[allow(dead_code)]
static EX_INPUT_2: &str = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT_1) 142,
    part2 => (EX_INPUT_2) 281
}

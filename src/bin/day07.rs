use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day07");

fn parse_hand<const N: usize>(
	order: [u8; N],
	cards: impl IntoIterator<Item = u8>,
) -> (Vec<usize>, [u8; N]) {
	let mut cnt = [0; N];
	let cards: Vec<_> = cards
		.into_iter()
		.map(|card| order.iter().position(|&r| r == card).unwrap())
		.collect();
	cards.iter().for_each(|&card| cnt[card] += 1);
	(cards, cnt)
}

fn part1(input: &str) -> Result<i64> {
	const CARD_ORDER: [u8; 13] = [
		b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A',
	];

	let mut hands: Vec<_> = to_lines(input)
		.map(|line| {
			let (cards, bid) = line.split_once(' ').unwrap();
			let bid: i64 = bid.parse().unwrap();
			let (cards, cnt) = parse_hand(CARD_ORDER, cards.as_bytes().iter().copied());
			let mut cnt: Vec<_> = cnt.into_iter().filter(|&c| c != 0).collect();
			cnt.sort_unstable_by(|a, b| b.cmp(a));
			(cnt, cards, bid)
		})
		.collect();

	hands.sort_unstable_by(|(ha, ca, _), (hb, cb, _)| match ha.cmp(hb) {
		std::cmp::Ordering::Equal => ca.cmp(cb),
		r => r,
	});

	let res = hands
		.into_iter()
		.enumerate()
		.map(|(rank, (_, _, bid))| (rank as i64 + 1) * bid)
		.sum();

	Ok(res)
}

fn part2(input: &str) -> Result<i64> {
	const CARD_ORDER: [u8; 13] = [
		b'J', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'Q', b'K', b'A',
	];

	let mut hands: Vec<_> = to_lines(input)
		.map(|line| {
			let (cards, bid) = line.split_once(' ').unwrap();
			let bid: i64 = bid.parse().unwrap();
			let (cards, mut cnt) = parse_hand(CARD_ORDER, cards.as_bytes().iter().copied());
			let joker_cnt = cnt[0];
			cnt[0] = 0;
			let mut cnt: Vec<_> = cnt.into_iter().filter(|&c| c != 0).collect();
			if cnt.is_empty() {
				cnt.push(joker_cnt);
			} else {
				cnt.sort_unstable_by(|a, b| b.cmp(a));
				cnt[0] += joker_cnt;
			}
			(cnt, cards, bid)
		})
		.collect();

	hands.sort_unstable_by(|(ha, ca, _), (hb, cb, _)| match ha.cmp(hb) {
		std::cmp::Ordering::Equal => ca.cmp(cb),
		r => r,
	});

	let res = hands
		.into_iter()
		.enumerate()
		.map(|(rank, (_, _, bid))| (rank as i64 + 1) * bid)
		.sum();

	Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 6440,
	part2 => (EX_INPUT) 5905
}

use std::collections::{HashMap, HashSet};

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day25");

// TODO: Solve this properly
fn part1(input: &str) -> Result<usize> {
	let mut graph = HashMap::<_, Vec<_>>::new();

	// need to remove:
	// xvp - zpc
	// vfs - dhl
	// pbq - nzn
	//
	// found this out by generating a graph with graphviz
	to_lines(input).for_each(|line| {
		let (from, to) = line.split_once(':').unwrap();
		to.split_ascii_whitespace()
			.filter(|&node| {
				!matches!(
					(from, node),
					("xvp", "zpc")
						| ("zpc", "xvp") | ("vfs", "dhl")
						| ("dhl", "vfs") | ("pbq", "nzn")
						| ("nzn", "pbq")
				)
			})
			.for_each(|node| {
				graph.entry(from).or_default().push(node);
				graph.entry(node).or_default().push(from);
			});
	});

	fn dfs<'a>(curr: &'a str, seen: &mut HashSet<&'a str>, graph: &HashMap<&'a str, Vec<&'a str>>) {
		seen.insert(curr);

		for conn in &graph[curr] {
			if !seen.contains(conn) {
				dfs(conn, seen, graph);
			}
		}
	}

	let mut loop1 = HashSet::new();
	dfs("xvp", &mut loop1, &graph);

	let mut loop2 = HashSet::new();
	dfs("zpc", &mut loop2, &graph);

	Ok(loop1.len() * loop2.len())
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"#;

aoc! {
	INPUT:
	part1 => (EX_INPUT) 54
}

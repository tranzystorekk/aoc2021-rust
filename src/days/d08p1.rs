use aoc_utils::BufferedInput;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<String> {
    let input = BufferedInput::parse_args("Day 8: Seven Segment Search - Part 1")?;

    input
        .unwrapped_lines()
        .flat_map(|l| {
            let (a, b, c, d) = scan_fmt!(&l, "{*[a-g ]} | {} {} {} {}", _, _, _, _).unwrap();

            [a, b, c, d]
        })
        .collect()
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        input
            .into_iter()
            .filter(|out| [2, 3, 4, 7].contains(&out.len()))
            .count()
    });
}

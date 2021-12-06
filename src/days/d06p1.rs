use aoc_utils::BufferedInput;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<u32> {
    let input = BufferedInput::parse_args("Day 6: Lanternfish - Part 1")?;

    let line = input.unwrapped_lines().next().unwrap();

    line.split(',').map(|n| n.parse().unwrap()).collect()
}

fn step_day(mut timers: Vec<u32>) -> Vec<u32> {
    let n_spawned = timers.iter_mut().fold(0, |spawn, timer| {
        if timer == &0 {
            *timer = 6;
            return spawn + 1;
        }

        *timer -= 1;
        spawn
    });

    timers.extend(itertools::repeat_n(8, n_spawned));

    timers
}

#[anyhoo::anyhoo]
fn main() {
    let mut input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        for _ in 0..80 {
            input = step_day(input);
        }

        input.len()
    });
}

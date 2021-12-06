use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<usize> {
    let input = BufferedInput::parse_args("Day 6: Lanternfish - Part 2")?;

    let line = input.unwrapped_lines().next().unwrap();

    line.split(',').map(|n| n.parse().unwrap()).collect()
}

const N_PHASES: usize = 9;
type Phases = [usize; N_PHASES];

fn init_phases(initial: Vec<usize>) -> Phases {
    let mut result = [0; N_PHASES];

    for phase in initial {
        result[phase] += 1;
    }

    result
}

fn step_day(mut phases: Phases) -> Phases {
    // remember how many fish have spawned this cycle
    let spawned = phases[0];

    // shift each phase
    for (curr, prev) in (0..N_PHASES).tuple_windows() {
        phases[curr] = phases[prev];
    }

    // calculate spawned fish and fish on new cycle
    phases[N_PHASES - 1] = spawned;
    phases[6] += spawned;

    phases
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut phases = init_phases(input);

        for _ in 0..256 {
            phases = step_day(phases);
        }

        phases.into_iter().sum::<usize>()
    });
}

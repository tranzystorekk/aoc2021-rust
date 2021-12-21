use aoc_utils::BufferedInput;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> (u32, u32) {
    let input = BufferedInput::parse_args("Day 21: Dirac Dice - Part 1")?;

    let mut lines = input.unwrapped_lines();

    let pos_p1 = lines
        .next()
        .map(|l| scan_fmt!(&l, "Player 1 starting position: {d}", _).unwrap())
        .unwrap();
    let pos_p2 = lines
        .next()
        .map(|l| scan_fmt!(&l, "Player 2 starting position: {d}", _).unwrap())
        .unwrap();

    (pos_p1, pos_p2)
}

#[derive(Debug)]
struct DiracDice<I> {
    die: I,
    player_one: (u32, u32),
    player_two: (u32, u32),
    roll_count: u32,
    game_over: bool,
}

#[derive(Debug)]
struct DeterministicDie<const N: u32> {
    current: u32,
}

impl<const N: u32> DeterministicDie<N> {
    fn new() -> Self {
        Self { current: 0 }
    }
}

impl<const N: u32> Iterator for DeterministicDie<N> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current + 1;

        self.current = (self.current + 1) % N;

        Some(value)
    }
}

impl<I: Iterator<Item = u32>> DiracDice<I> {
    fn new<T: IntoIterator<IntoIter = I>>(p1_start: u32, p2_start: u32, die: T) -> Self {
        Self {
            die: die.into_iter(),
            player_one: (p1_start - 1, 0),
            player_two: (p2_start - 1, 0),
            roll_count: 0,
            game_over: false,
        }
    }

    fn play_round(&mut self) {
        if self.game_over {
            return;
        }

        let die = self.die.by_ref();

        let roll_p1: u32 = die.take(3).sum();
        let (pos, score) = &mut self.player_one;

        *pos = (*pos + roll_p1) % 10;
        *score += *pos + 1;

        self.roll_count += 3;

        if *score >= 1000 {
            self.game_over = true;
            return;
        }

        let roll_p2: u32 = die.take(3).sum();
        let (pos, score) = &mut self.player_two;

        *pos = (*pos + roll_p2) % 10;
        *score += *pos + 1;

        self.roll_count += 3;
        
        if *score >= 1000 {
            self.game_over = true;
        }
    }

    fn secret(&self) -> Option<u32> {
        if !self.game_over {
            return None;
        }

        let (p1, p2) = (self.player_one.1, self.player_two.1);
        let losing = std::cmp::min(p1, p2);

        Some(losing * self.roll_count)
    }
}

type Die = DeterministicDie<100>;
type Game = DiracDice<Die>;

#[anyhoo::anyhoo]
fn main() {
    let (p1, p2) = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let die = Die::new();
        let mut game = Game::new(p1, p2, die);

        let mut game_iter = std::iter::from_fn(|| {
            game.play_round();
            game.secret()
        });

        loop {
            if let Some(v) = game_iter.next() {
                return v;
            }
        }
    });
}

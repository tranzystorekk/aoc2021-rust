use aoc_utils::BufferedInput;
use scan_fmt::scan_fmt;

const DISTRIBUTION: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[anyhoo::anyhoo]
fn parse_input() -> (u64, u64) {
    let input = BufferedInput::parse_args("Day 21: Dirac Dice - Part 2")?;

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

#[derive(Clone, Copy, Debug)]
enum Player {
    One,
    Two,
}

#[derive(Clone, Copy, Debug)]
struct GameState {
    next_player: Player,
    p1: (u64, u64),
    p2: (u64, u64),
}

// (state, move, universes, split_universes)
type Searchspace = Vec<(GameState, u64, u64, u64)>;

impl Player {
    fn advance(&mut self) {
        *self = match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        };
    }
}

impl GameState {
    fn make_move(&mut self, mov: u64) -> Option<Player> {
        let (pos, score) = match self.next_player {
            Player::One => &mut self.p1,
            Player::Two => &mut self.p2,
        };

        *pos = (*pos + mov) % 10;
        *score += *pos + 1;

        let result = (*score >= 21).then(|| self.next_player);

        self.next_player.advance();

        result
    }
}

fn init_searchspace(p1: u64, p2: u64) -> Searchspace {
    let initial = GameState {
        next_player: Player::One,
        p1: (p1 - 1, 0),
        p2: (p2 - 1, 0),
    };

    DISTRIBUTION
        .into_iter()
        .map(|(mov, universes)| (initial, mov, universes, 1))
        .collect()
}

fn play_game(p1: u64, p2: u64) -> u64 {
    let mut searchspace = init_searchspace(p1, p2);
    let mut p1_wins = 0;
    let mut p2_wins = 0;

    while let Some((mut state, mov, new_universes, current_universes)) = searchspace.pop() {
        let maybe_winner = state.make_move(mov);
        let split_universes = new_universes * current_universes;

        match maybe_winner {
            Some(Player::One) => {
                p1_wins += split_universes;
            }
            Some(Player::Two) => {
                p2_wins += split_universes;
            }
            None => {
                let next_states = DISTRIBUTION
                    .into_iter()
                    .map(|(next_mov, universes)| (state, next_mov, universes, split_universes));

                searchspace.extend(next_states);
            }
        }
    }

    std::cmp::max(p1_wins, p2_wins)
}

#[anyhoo::anyhoo]
fn main() {
    let (p1, p2) = parse_input()?;

    aoc_utils::measure_and_print(|| play_game(p1, p2));
}

use colored::Colorize;

pub struct Tracker {
    pub total_games: usize,
    pub wins: usize,
}

impl Tracker {
    pub fn new() -> Tracker {
        Tracker {
            total_games: 0,
            wins: 0,
        }
    }

    pub fn add_game(&mut self, win: bool) {
        self.total_games += 1;
        if win {
            self.wins += 1;
        }
    }

    pub fn print_summary(&self) {
        println!(
            "You played {} games and won {} of them ({:.2}%) !",
            format!("{}", self.total_games).blue(),
            format!("{}", self.wins).green(),
            (self.wins as f64 * 100.) / self.total_games as f64
        );
    }
}

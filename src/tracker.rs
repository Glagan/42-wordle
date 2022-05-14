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
        let win_rate = (self.wins as f64 * 100.) / self.total_games as f64;
        print!(
            "You played {} games and won {} of them ({:.2}%) ! ",
            format!("{}", self.total_games).blue(),
            format!("{}", self.wins).green(),
            win_rate
        );
        if win_rate < 1. {
            print!("{}", "Ouch.".red());
        } else if win_rate < 50. {
            print!("{}", "Good job !".blue());
        } else if win_rate < 100. {
            print!("{}", "You're great !".green());
        } else {
            print!(
                "{}{}{}{}{}{}{}",
                "P".on_blue(),
                "E".on_green(),
                "R".on_yellow(),
                "F".on_purple(),
                "E".on_cyan(),
                "C".on_green(),
                "T".on_blue()
            );
        }
        println!();
    }
}

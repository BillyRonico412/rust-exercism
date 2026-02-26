use std::{cmp::Ordering, collections::HashMap, fmt::Display};

struct Team {
    name: String,
    win: u32,
    draw: u32,
    loss: u32,
    points: u32,
}

#[derive(Default)]
struct Tournament {
    team_map: HashMap<String, Team>,
}

#[derive(Clone, Copy)]
enum MatchResult {
    Win,
    Draw,
    Loss,
}

impl MatchResult {
    fn complementary(&self) -> Self {
        match self {
            Self::Win => Self::Loss,
            Self::Loss => Self::Win,
            Self::Draw => Self::Draw,
        }
    }
}

impl Team {
    fn new(team_name: &str) -> Self {
        Team {
            name: String::from(team_name),
            draw: 0,
            loss: 0,
            win: 0,
            points: 0,
        }
    }
    fn add_result(&mut self, match_result: MatchResult) {
        match match_result {
            MatchResult::Win => {
                self.win += 1;
                self.points += 3;
            }
            MatchResult::Draw => {
                self.draw += 1;
                self.points += 1
            }
            MatchResult::Loss => self.loss += 1,
        }
    }
    fn display_header() -> String {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            "Team", "MP", "W", "D", "L", "P"
        )
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name,
            self.draw + self.loss + self.win,
            self.win,
            self.draw,
            self.loss,
            self.points
        )
    }
}

impl Tournament {
    fn add_result(&mut self, team_1: &str, team_2: &str, result: MatchResult) {
        self.get_team(team_1).add_result(result);
        self.get_team(team_2).add_result(result.complementary());
    }
    fn get_team(&mut self, team_name: &str) -> &mut Team {
        self.team_map
            .entry(String::from(team_name))
            .or_insert(Team::new(team_name))
    }
    fn tally(&self) -> String {
        format!("{}{}", Team::display_header(), self)
    }
}

impl Display for Tournament {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut team_vec = self.team_map.values().collect::<Vec<_>>();
        team_vec.sort_by(|&a, &b| match b.points.cmp(&a.points) {
            ordering @ (Ordering::Greater | Ordering::Less) => ordering,
            _ => a.name.cmp(&b.name),
        });
        let tournament_format = team_vec
            .iter()
            .map(|&team| format!("\n{}", team))
            .collect::<String>();
        write!(f, "{}", tournament_format)
    }
}

pub fn tally(match_results: &str) -> String {
    let mut tournament = Tournament::default();
    match_results.split('\n').for_each(|match_item| {
        let mut match_item_split = match_item.split(';');
        if let (Some(team_1), Some(team_2), Some(match_result)) = (
            match_item_split.next(),
            match_item_split.next(),
            match_item_split.next(),
        ) {
            let match_result: MatchResult = match match_result {
                "win" => MatchResult::Win,
                "loss" => MatchResult::Loss,
                "draw" => MatchResult::Draw,
                _ => panic!(),
            };
            tournament.add_result(team_1, team_2, match_result);
        }
    });
    tournament.tally()
}

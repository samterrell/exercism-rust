use std::{collections::HashMap, io::BufRead};

struct TeamStats<'a> {
    name: &'a str,
    wins: u16,
    draws: u16,
    losses: u16,
}

enum WLD {
    Win,
    Loss,
    Draw,
}

struct State<'a> {
    stats: HashMap<&'a str, TeamStats<'a>>,
}

impl<'a> TeamStats<'a> {
    fn new(name: &'a str) -> Self {
        TeamStats {
            name,
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    fn record(&mut self, result: WLD) -> () {
        match result {
            WLD::Win => self.wins += 1,
            WLD::Loss => self.losses += 1,
            WLD::Draw => self.draws += 1,
        }
    }
}

impl<'a> State<'a> {
    fn new() -> Self {
        State {
            stats: HashMap::new(),
        }
    }

    fn record(&mut self, name: &'a str, result: WLD) -> &mut Self {
        self.stats
            .entry(name)
            .or_insert_with(|| TeamStats::new(name))
            .record(result);
        self
    }

    fn parse(&mut self, line: &'a str) -> Result<(), ()> {
        let fields: Vec<&str> = line.split(&[';']).collect();
        if fields.len() == 3 {
            let left = fields[0];
            let right = fields[1];
            match fields[2] {
                "win" => self.record(&left, WLD::Win).record(&right, WLD::Loss),
                "loss" => self.record(&left, WLD::Loss).record(&right, WLD::Win),
                "draw" => self.record(&left, WLD::Draw).record(&right, WLD::Draw),
                _ => return Err(()),
            };
            Ok(())
        } else {
            Err(())
        }
    }
}

impl ToString for State<'_> {
    fn to_string(&self) -> String {
        let results = &mut vec![];
        for v in self.stats.values() {
            results.push((
                v.name,
                v.wins + v.draws + v.losses,
                v.wins,
                v.draws,
                v.losses,
                v.wins * 3 + v.draws,
            ))
        }
        results.sort_by(|l, r| r.5.cmp(&l.5).then(l.0.cmp(&r.0)));
        let mut result = "Team                           | MP |  W |  D |  L |  P".to_string();
        for (team, mp, w, d, l, p) in results {
            result.push_str(
                &format!(
                    "\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                    team, mp, w, d, l, p
                )
                .to_string(),
            );
        }
        result
    }
}

pub fn tally(match_results: &str) -> String {
    let mut state = State::new();
    for line in match_results.lines() {
        if let Err(()) = state.parse(line) {
            println!("Bad line: {}", line);
        }
    }
    state.to_string()
}

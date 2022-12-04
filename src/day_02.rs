#[derive(PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn read_rps(line: &str) -> Option<RPS> {
    match line {
        "A" => Some(RPS::Rock),
        "B" => Some(RPS::Paper),
        "C" => Some(RPS::Scissors),
        "X" => Some(RPS::Rock),
        "Y" => Some(RPS::Paper),
        "Z" => Some(RPS::Scissors),
        _ => None,
    }
}

fn parse_line(line: &str) -> Option<(RPS, RPS)> {
    let mut parts = line.split(" ");

    let them = parts.next().and_then(read_rps)?;
    let us = parts.next().and_then(read_rps)?;

    return Some((them, us));
}

fn show_rps(rps: RPS) -> &'static str {
    match rps {
        RPS::Rock => "🪨",
        RPS::Paper => "📄",
        RPS::Scissors => "✂️",
    }
}

fn rps_score(rps: RPS) -> i32 {
    match rps {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn matchup_score((them, us): (RPS, RPS)) -> i32 {
    if them == us {
        return 3;
    }

    return match (them, us) {
        (RPS::Rock, RPS::Paper) => 6,
        (RPS::Paper, RPS::Scissors) => 6,
        (RPS::Scissors, RPS::Rock) => 6,
        _ => 0,
    };
}

pub fn task_02_1() -> String {
    let input = include_str!("../inputs/02/input.txt");
    let strategy = input.split("\n").filter_map(parse_line);

    let strategy_score: i32 = strategy
        .map(|matchup| matchup_score(matchup) + rps_score(matchup.1))
        .sum();

    return strategy_score.to_string();
}

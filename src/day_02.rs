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

pub fn task_02_1() -> String {
    let input = include_str!("../inputs/02/input.txt");
    let strategy = input.split("\n").filter_map(parse_line);

    let items = strategy.map(|(them, us)| format!("{} {}", show_rps(them), show_rps(us)));
    let mut stuff = String::from("");

    for item in items {
        stuff = format!("{}\n{}", stuff, item);
    }

    return stuff;
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_move(input: &str) -> Option<Move> {
    let parts: Vec<_> = input.split(" ").collect();

    let word_move = parts.get(0)?;
    let count = parts.get(1)?;
    let word_from = parts.get(2)?;
    let from = parts.get(3)?;
    let word_to = parts.get(4)?;
    let to = parts.get(5)?;

    if *word_move != "move" || *word_from != "from" || *word_to != "to" {
        return None;
    }

    let from: usize = from.parse().ok()?;
    let to: usize = to.parse().ok()?;

    return Some(Move {
        from: from - 1,
        to: to - 1,
        count: count.parse().ok()?,
    });
}

type Stacks = Vec<Vec<char>>;

fn parse_stacks(input: &str) -> Option<Stacks> {
    let mut symbol_rows: Stacks = Vec::new();

    // parse lines
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars().skip(1).step_by(4) {
            row.push(c);
        }
        symbol_rows.push(row);
    }
    let column_count = symbol_rows.pop()?.len();

    // transpose
    let mut symbol_cols: Stacks = Vec::new();
    for column_index in 0..column_count {
        let col: Vec<char> = symbol_rows
            .iter()
            .map(|row| row.get(column_index).unwrap_or(&' ').clone())
            .filter(|c| c.ne(&' '))
            .collect();

        symbol_cols.push(col);
    }

    return Some(symbol_cols);
}

fn stack_tops(stacks: Stacks) -> String {
    return stacks.iter().filter_map(|stack| stack.first()).collect();
}

fn parse_input(input: &str) -> Option<(Stacks, Vec<Move>)> {
    let input_parts: Vec<_> = input.split("\n\n").collect();
    let stack_input = input_parts.get(0)?;
    let move_input = input_parts.get(1)?;

    let stacks = parse_stacks(stack_input)?;
    let moves: Vec<_> = move_input.split("\n").filter_map(parse_move).collect();

    return Some((stacks, moves));
}

fn execute_moves(stacks: Stacks, moves: Vec<Move>) -> Stacks {
    let mut restacking = stacks;

    for Move { from, to, count } in moves.iter() {
        let mut new_stacking: Stacks = Vec::new();

        for (i, stack) in restacking.iter().enumerate() {
            if i.eq(from) {
                // Push line with skipped symbols
                new_stacking.push(stack.iter().skip(*count).map(|c| *c).collect());
            } else if i.eq(to) {
                // Push line with additional symbols
                let mut column: Vec<char> = restacking
                    .get(*from)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .take(*count)
                    .map(|c| *c)
                    .collect();
                column.append(&mut stack.clone());
                new_stacking.push(column);
            } else {
                new_stacking.push(stack.to_vec());
            }
        }

        restacking = new_stacking;
    }

    return restacking;
}

pub fn task_05_1() -> String {
    let input = include_str!("../inputs/05/input.txt");
    let (stacks, moves) = parse_input(input).unwrap();
    let moved_stacks = execute_moves(stacks, moves);

    return format!("{:?}", stack_tops(moved_stacks));
}

pub fn task_05_2() -> String {
    let input = include_str!("../inputs/05/input.txt");

    return input.to_string();
}

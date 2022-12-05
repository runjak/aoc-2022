fn parse_range(input: &str) -> Option<(i32, i32)> {
    let items: Vec<_> = input.split("-").collect();

    let start = items.get(0)?.parse::<i32>().ok()?;
    let end = items.get(1)?.parse::<i32>().ok()?;

    return Some((start, end));
}

fn parse_shifts(input: &str) -> Option<((i32, i32), (i32, i32))> {
    let items: Vec<_> = input.split(",").collect();

    let first_range = parse_range(items.get(0)?)?;
    let second_range = parse_range(items.get(1)?)?;

    return Some((first_range, second_range));
}

fn range_contains(range: (i32, i32), element: i32) -> bool {
    element >= range.0 && element <= range.1
}

fn fully_contains(shifts: ((i32, i32), (i32, i32))) -> bool {
    let first_contains_second =
        range_contains(shifts.0, shifts.1 .0) && range_contains(shifts.0, shifts.1 .1);
    let second_contains_first =
        range_contains(shifts.1, shifts.0 .0) && range_contains(shifts.1, shifts.0 .1);

    return first_contains_second || second_contains_first;
}

pub fn task_04_1() -> String {
    let input = include_str!("../inputs/04/input.txt");
    let shiftss = input.split("\n").filter_map(parse_shifts);

    let fully_contained_count = shiftss.filter(|shifts| fully_contains(*shifts)).count();

    return fully_contained_count.to_string();
}

pub fn task_04_2() -> String {
    let input = include_str!("../inputs/04/input.txt");

    return String::from(input);
}

fn lines_as_numbers(input: &str) -> Vec<i32> {
    return input
        .split("\n")
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();
}

fn elve_calories(input: &str) -> Vec<i32> {
    return input
        .split("\n\n")
        .map(|elve| lines_as_numbers(elve).into_iter().sum())
        .collect();
}

pub fn task_01_1() -> String {
    let input = include_str!("../inputs/01/input.txt");
    let calories = elve_calories(input);

    let max_calories = calories.into_iter().max().unwrap_or(0);

    return max_calories.to_string();
}

pub fn task_01_2() -> String {
    let input = include_str!("../inputs/01/input.txt");

    let mut calories = elve_calories(input);
    calories.sort();

    let foo: i32 = calories[calories.len() - 3..calories.len()]
        .into_iter()
        .sum();

    return foo.to_string();
}

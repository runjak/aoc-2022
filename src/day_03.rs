use std::collections::HashSet;

fn common_item(rucksack: &str) -> Option<String> {
    if rucksack.is_empty() {
        return None;
    }

    let (left, right) = rucksack.split_at(rucksack.len() / 2);
    let right_set: HashSet<_> = right.chars().collect();

    return left
        .chars()
        .find(|item| right_set.contains(&item))
        .map(|char| char.to_string());
}

fn item_priority(item: String) -> Option<i32> {
    let small_a = "a".as_bytes().get(0)?;
    let small_z = "z".as_bytes().get(0)?;
    let big_a = "A".as_bytes().get(0)?;
    let big_z = "Z".as_bytes().get(0)?;

    let code = item.as_bytes().get(0)?;

    if code >= small_a && code <= small_z {
        return Some(i32::from(code - small_a + 1));
    }

    if code >= big_a && code <= big_z {
        return Some(i32::from(code - big_a + 27));
    }

    return None;
}

pub fn task_03_1() -> String {
    let input = include_str!("../inputs/03/input.txt");
    let common_items = input.split("\n").filter_map(common_item);

    let priority_sum: i32 = common_items.filter_map(item_priority).sum();

    return format!("{}", priority_sum);
}

pub fn task_03_2() -> String {
    return String::from("to be implemented.");
}

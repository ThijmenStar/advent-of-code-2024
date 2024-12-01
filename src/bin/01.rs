use std::collections::HashMap;
use std::str::FromStr;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for i in input.lines() {
        let mut entries = i.split_whitespace();
        list1.push(u32::from_str(entries.next().unwrap()).expect("failed to parse to u32"));
        list2.push(u32::from_str(entries.next().unwrap()).expect("failed to parse to u32"));
    }

    list1.sort();
    list2.sort();

    let mut solution = 0;

    for (i, k) in list1.iter().zip(list2.iter()) {
        solution += i.abs_diff(*k);
    }

    Some(solution)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list: Vec<u32> = Vec::new();
    let mut map: HashMap<u32, u32> = HashMap::new();

    for i in input.lines() {
        let mut entries = i.split_whitespace();
        list.push(u32::from_str(entries.next().unwrap()).expect("failed to parse to u32"));

        *map.entry(u32::from_str(entries.next().unwrap()).expect("failed to parse to u32"))
            .or_insert(0) += 1;
    }

    let mut solution = 0;

    for i in list {
        solution += i * map.get(&i).unwrap_or(&0);
    }

    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

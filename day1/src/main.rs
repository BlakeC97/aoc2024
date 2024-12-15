use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::zip;
use anyhow::Result;

fn part1(left_ids: &[i64], right_ids: &[i64]) -> i64 {
    zip(left_ids, right_ids)
        .fold(0, |acc, (left, right)| {
            acc + (left - right).abs()
        })
}

fn part2(left_ids: &[i64], right_ids: &[i64]) -> i64 {
    let mut frequencies: HashMap<i64, i64> = HashMap::with_capacity(left_ids.len());
    for item in right_ids {
        match frequencies.entry(*item) {
            Entry::Vacant(v) => { v.insert(1); }
            Entry::Occupied(mut e) => { *e.get_mut() += 1; }
        }
    }
    
    left_ids
        .iter()
        .fold(0, |acc, item| {
            acc + (item  * frequencies.get(item).unwrap_or(&0))
        })
}

fn main() -> Result<()> {
    let input = include_str!("../data/input.txt");
    let mut left_ids: Vec<i64> = Vec::new();
    let mut right_ids: Vec<i64> = Vec::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        left_ids.push(iter.next().expect("Will always have a first element").parse()?);
        right_ids.push(iter.next().expect("Will always have a second element").parse()?);
    }
    left_ids.sort();
    right_ids.sort();

    let total_distance = part1(&left_ids, &right_ids);
    let similarity_score = part2(&left_ids, &right_ids);
    println!("Total distance (day 1): {total_distance}");
    println!("Similarity score (day 2): {similarity_score}");

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9 ,3];
        left.sort();
        right.sort();

        assert_eq!(part1(&left, &right), 11);
    }

    #[test]
    fn test_part2() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9 ,3];
        left.sort();
        right.sort();

        assert_eq!(part2(&left, &right), 31);
    }
}
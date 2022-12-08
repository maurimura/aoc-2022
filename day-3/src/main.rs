use std::collections::HashSet;
use std::{borrow::BorrowMut, fs};
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let binding = &fs::read("./day-3/input")?;
    let content = String::from_utf8_lossy(binding);
    let content: Vec<&str> = content
        .split("\n")
        .filter(|word| !word.is_empty())
        .collect();

    let mut lowercase = ('a'..='z').collect::<Vec<_>>();
    let mut uppercase = ('A'..='Z').collect::<Vec<_>>();
    lowercase.append(uppercase.borrow_mut());
    let case_sensitive_alphabet = lowercase;

    let content_part_2 = content.clone();

    let total = content
        .iter()
        .map(|rucksacks| {
            let len = rucksacks.len();
            let first_sack = &rucksacks[0..len / 2];
            let second_sack = &rucksacks[len / 2..len];

            let unique = first_sack
                .chars()
                .filter(|char| {
                    second_sack
                        .chars()
                        .filter(|maybe_repeated_char| char == maybe_repeated_char)
                        .collect::<Vec<_>>()
                        .len()
                        > 0
                })
                .collect::<HashSet<_>>();
            let local_total = unique
                .iter()
                .map(|item| {
                    match case_sensitive_alphabet
                        .iter()
                        .position(|priority| priority == item)
                    {
                        Some(priority) => {
                            println!("{} {}", item, priority + 1);
                            priority + 1
                        }
                        None => 0,
                    }
                })
                .reduce(|acc, current| acc + current);

            match local_total {
                Some(local_total) => local_total,
                None => 0,
            }
        })
        .reduce(|acc, partial_total| acc + partial_total);
    println!("{:?}", total);

    let mut part_2_total = 0;
    let mut contetn_part_2_iter = content_part_2.iter();
    loop {
        match contetn_part_2_iter.next() {
            Some(first) => match contetn_part_2_iter.next() {
                Some(second) => match contetn_part_2_iter.next() {
                    Some(third) => {
                        let unique = first
                            .chars()
                            .filter(|char| {
                                let is_in_second = second
                                    .chars()
                                    .filter(|second| char == second)
                                    .collect::<Vec<_>>()
                                    .len()
                                    > 0;
                                let is_in_third = third
                                    .chars()
                                    .filter(|third| char == third)
                                    .collect::<Vec<_>>()
                                    .len()
                                    > 0;
                                is_in_second && is_in_third
                            })
                            .collect::<Vec<_>>();

                        let unique = unique.iter().collect::<HashSet<_>>();

                        let local_total = unique
                            .iter()
                            .map(|item| {
                                match case_sensitive_alphabet
                                    .iter()
                                    .position(|priority| priority.cmp(item).is_eq())
                                {
                                    Some(priority) => {
                                        println!("{} {}", item, priority + 1);
                                        priority + 1
                                    }
                                    None => 0,
                                }
                            })
                            .reduce(|acc, current| acc + current);
                        match local_total {
                            Some(local_total) => {
                                println!("Local total {}", local_total);
                                part_2_total += local_total
                            }
                            None => (),
                        }
                    }
                    None => break,
                },
                None => break,
            },
            None => break,
        };
    }

    println!("{}", part_2_total);
    Ok(())
}

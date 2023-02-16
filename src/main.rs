use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::vec::Vec;

#[allow(dead_code)]
fn day1() {
    let contents = fs::read_to_string("src/input.txt").expect("couldnt read the file");
    let mut calories = Vec::new();

    calories.push(0);
    let lines = contents.split('\n');
    let mut current_elf = calories.len() - 1;
    for line in lines {
        match line.parse::<i32>() {
            Ok(number) => calories[current_elf] = calories.last().unwrap() + number,
            Err(_) => calories.push(0)
        }
        current_elf = calories.len() - 1;
    }
    match calories.iter().max() {
        Some(max) => println!("Part 1: {max}"),
        None => println!("Part 1: Empty"),
    }
    calories.sort();
    calories.reverse();
    println!("{}", calories[0]+calories[1]+calories[2]);
}

#[allow(dead_code)]
fn day2() {
    // A, X -> Rock 1
    // B, Y -> Paper 2
    // C, Z -> Scissors 3
    //
    // A beats C is beaten by Y draws with X
    let mut values = HashMap::new();
    values.insert("X", 1);
    values.insert("Y", 2);
    values.insert("Z", 3);

    let mut beats = HashMap::new();
    beats.insert("A", "Z");
    beats.insert("B", "X");
    beats.insert("C", "Y");

    let mut loses = HashMap::new();
    loses.insert("A", "Y");
    loses.insert("B", "Z");
    loses.insert("C", "X");

    let mut draws = HashMap::new();
    draws.insert("A", "X");
    draws.insert("B", "Y");
    draws.insert("C", "Z");

    let contents = fs::read_to_string("src/input.txt").expect("couldnt read the file");
    let lines: Vec<&str> = contents.trim().split('\n').collect();
    let mut scores = Vec::new();

    for line in lines {
        let hands: Vec<&str> = line.split(' ').collect();
        let mut score = 0;
        if hands[1] == "X" {
            score += values[beats[hands[0]]];
        } else if hands[1] == "Z" {
            score += values[loses[hands[0]]] + 6;
        } else {
            score += values[draws[hands[0]]] + 3;
        }

        scores.push(score);
    }
    let total_score: i32 = scores.iter().sum();
    println!("{total_score}");
}

#[allow(dead_code)]
fn day3() {
    let contents = fs::read_to_string("src/input.txt").expect("couldnt read the file");
    let rucksacks: Vec<&str> = contents.trim().split('\n').collect();
    // let mut priorities = Vec::new();

    // for rucksack in rucksacks {
    //     let items: Vec<char> = rucksack.chars().collect();
    //     let sack_load = items.len();
    //     let first_slot_types = HashSet::<&char>::from_iter(&items[0..sack_load/2]);
    //     let second_slot_types = HashSet::from_iter(&items[sack_load/2..=items.len()-1]);

    //     let mut error_type = first_slot_types.intersection(&second_slot_types);
    //     let error_type = error_type.next().unwrap().clone();
    //     let char_value = *error_type as u32;
    //     let mut value = 0;
    //     if char_value > 96 && char_value < 123 {
    //         value = char_value - 96;
    //     } else if char_value > 64 && char_value < 91 {
    //         value = char_value - 64 + 26;
    //     }
    //     priorities.push(value);
    // }
    let mut group_priorities = Vec::new();
    for group in rucksacks.chunks(3) { 
        let group_items: Vec<Vec<char>> = group.iter().map(|rucksack| rucksack.chars().collect()).collect();
        let types_1 = HashSet::<&char>::from_iter(&group_items[0]);
        let types_2 = HashSet::<&char>::from_iter(&group_items[1]);
        let types_3 = HashSet::<&char>::from_iter(&group_items[2]);

        let tmp_inter: Vec<&char> = types_1.intersection(&types_2).cloned().collect();
        let tmp_inter: HashSet<&char> = HashSet::from_iter(tmp_inter);
        let mut full_inter = tmp_inter.intersection(&types_3);
        let badge = <&char>::clone(full_inter.next().unwrap());
        let char_value = *badge as u32;
        let mut value = 0;
        if char_value > 96 && char_value < 123 {
            value = char_value - 96;
        } else if char_value > 64 && char_value < 91 {
            value = char_value - 64 + 26;
        }
        group_priorities.push(value);
    }
    let sum: u32 = group_priorities.iter().sum();
    println!("{sum}");
}

fn main() {
    day3();
}

use std::collections::{HashMap, HashSet};

fn main() -> Result<(), std::io::Error>{
    let now = std::time::Instant::now();
    let file_location = "day_14.txt";
    let mut pairs_hash = HashMap::new();
    let contents: String = std::fs::read_to_string(file_location)?;
    let vec = contents.split("\n\n")
                        .collect::<Vec<&str>>();
    let start = vec[0].clone().chars().collect::<Vec<char>>();
    let pairs = vec[1].split("\n").collect::<Vec<&str>>();
    let pairs = pairs.iter().map(|x| x.split(" -> ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    for pair in pairs {
        pairs_hash.insert(pair[0].to_string(), pair[1]);
    }
    
    part_2(pairs_hash, start);

    println!("time in milliseconds: {}", now.elapsed().as_millis());
    Ok(())
}

fn part_1(pairs_hash: HashMap<String, &str>, start: Vec<char>) {
    let mut polymer: Vec<char> = start.clone();
    let mut counts = HashMap::new();
    for i in 1..11 {
        println!("step: {}", i);
        polymer = advance_polymer(&pairs_hash, polymer);
    }
    polymer.iter().for_each(|x| {
        let counter = counts.entry(x).or_insert(0);
        *counter += 1;
    });
    let mut min = std::i32::MAX;
    let mut max = std::i32::MIN;
    for (_, value) in counts {
        if value > max {
            max = value;
        }
        if value < min {
            min = value;
        }
    }
    println!("{}", max-min);
}

fn advance_polymer(pairs_hash: &HashMap<String, &str>, start: Vec<char>) -> Vec<char> {
    let mut new_polymer: Vec<char> = Vec::new();
    for (i, char) in start.iter().enumerate() {
        if i == start.len() - 1 {
            new_polymer.push(char.clone());
            continue;
        }
        let next_char = start[i + 1];
        let combo = format!("{}{}", char, next_char);
        new_polymer.push(char.clone());
        new_polymer.push(pairs_hash.get(&combo).unwrap().chars().next().unwrap());
    }
    drop(start);
    new_polymer
}

fn part_2(pairs_hash: HashMap<String, &str>, start: Vec<char>) {
    let mut polymer_pairs: HashMap<String,i64> = HashMap::new();
    let mut counts: HashMap<char,i64> = HashMap::new();
    for char in &start {
        let counter = counts.entry(*char).or_insert(0);
        *counter += 1;
    }
    for (i, char) in start.iter().enumerate() {
        if i == start.len() - 1 {
            continue;
        }
        let next_char = start[i + 1];
        let combo = format!("{}{}", char, next_char);
        let counter = polymer_pairs.entry(combo).or_insert(0);
        *counter += 1;
    }
    
    for i in 1..41 {
        println!("step: {}", i);
        polymer_pairs = advance_polymer_pairs(&pairs_hash, polymer_pairs, &mut counts);
    }
 
    let mut min = std::i64::MAX;
    let mut max = std::i64::MIN;
    for (_, value) in counts {
        if value > max {
            max = value;
        }
        if value < min {
            min = value;
        }
    }
    println!("{}", max-min);
}

fn advance_polymer_pairs(pairs_hash: &HashMap<String, &str>, polymer_pairs: HashMap<String, i64>, counts: &mut HashMap<char, i64>) -> HashMap<String, i64> {
    let mut new_polymer_pairs: HashMap<String, i64> = HashMap::new();
    for (pair, count) in &polymer_pairs {
        let char_to_be_inserted = pairs_hash.get(pair).unwrap().chars().next().unwrap();
        let char_counter: &mut i64 = counts.entry(char_to_be_inserted).or_insert(0);
        *char_counter += count;
        let first_of_pair = pair.chars().next().unwrap();
        let second_of_pair = pair.chars().nth(1).unwrap();
        let new_pair_1 = format!("{}{}", first_of_pair, char_to_be_inserted);
        let new_pair_2 = format!("{}{}", char_to_be_inserted, second_of_pair);
        let first_counter: &mut i64 = new_polymer_pairs.entry(new_pair_1).or_insert(0);
        *first_counter += count;
        let second_counter: &mut i64 = new_polymer_pairs.entry(new_pair_2).or_insert(0);
        *second_counter += count;
    }
    new_polymer_pairs
}
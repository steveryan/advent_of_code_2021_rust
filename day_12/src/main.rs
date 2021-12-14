use std::collections::{HashMap, HashSet};

fn main() -> Result<(), std::io::Error>{
    let now = std::time::Instant::now();
    let file_location = "day_12.txt";
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let contents: String = std::fs::read_to_string(file_location)?;
    let vec = contents
                            .split("\n")
                            .collect::<Vec<&str>>();
    let vec_of_vecs = vec
                                      .iter()
                                        .map(|x| x.split("-")
                                        .collect::<Vec<&str>>())
                                        .collect::<Vec<Vec<&str>>>();
    for vec in &vec_of_vecs {
        let first = vec[0];
        let second = vec[1];
        let first_set = map.entry(first).or_insert(HashSet::new());
        first_set.insert(second);
        let second_set = map.entry(second).or_insert(HashSet::new());
        second_set.insert(first);
    }
    part_2(map);
    println!("time in milliseconds: {}", now.elapsed().as_millis());
    Ok(())
}

fn part_2(mut map: HashMap<&str, HashSet<&str>>) -> () {
    let mut paths = 0;
    let start = "start";
    let mut visited = HashMap::new();
    visited.insert(start, 1);
    explore_path_from(start, map, &mut paths, visited);
    println!("paths: {}", paths);
}

fn explore_path_from(start: &str, mut map: HashMap<&str, HashSet<&str>>, mut paths: &mut i32, mut visited: HashMap<&str,i32>) -> () {
    let choices = &map[start];
    for choice in choices {
        
        let num_times_visited = visited.get(choice);
        if choice.to_lowercase() == choice.to_string() && num_times_visited.is_some() && visited.values().collect::<Vec<&i32>>().iter().any(|x| x > &&1) {
            continue;
        }
        if *choice == "start" {
            continue;
        }
        if *choice == "end" {
            *paths += 1;
        } else {
            let mut new_visited = visited.clone();
            if choice.to_lowercase() == choice.to_string() {
                let counter = new_visited.entry(choice).or_insert(0);
                *counter += 1;
            }
            explore_path_from(choice, map.clone(), paths, new_visited)
        }
    }
}
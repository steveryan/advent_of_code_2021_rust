use std::collections::{HashMap, HashSet};

fn main() -> Result<(), std::io::Error>{
    let now = std::time::Instant::now();
    let file_location = "day_15.txt";
    let contents: String = std::fs::read_to_string(file_location)?;
    let vec = contents.split('\n').collect::<Vec<&str>>();
    let grid = vec.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let grid = grid.iter().map(|x| x.iter().map(|y| y.to_digit(10).unwrap() as u64).collect::<Vec<u64>>()).collect::<Vec<Vec<u64>>>();
    
    part_2(grid);
    println!("time in seconds: {}", now.elapsed().as_secs());
    Ok(())
}

fn part_1(mut grid: Vec<Vec<u64>>) {
    let mut explored: HashSet<(usize,usize)> = HashSet::new();
    let mut queue = HashSet::new();
    let mut queue_with_dist = HashSet::new();
    let mut distance = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let coords = (i, j);
            queue.insert(coords);
            distance.insert(coords, std::u64::MAX);
        }
    }
    distance.insert((0,0), 0);
    queue_with_dist.insert((0,0));
    println!("prep work done. Starting graph search now");
    run_path_search_iteratively((0,0), &mut grid, &mut explored, &mut queue, &mut distance, &mut queue_with_dist);
}

fn get_lowest_distance(distance: &mut HashMap<(usize,usize), u64>, _queue: &mut HashSet<(usize,usize)>, queue_with_dist: &mut HashSet<(usize,usize)>) -> (usize,usize) {
    let mut lowest_distance = std::u64::MAX;
    let mut lowest_coords = (0,0);
    for coords in queue_with_dist.iter() {
        let distance = distance.get(coords).unwrap();
        if *distance < lowest_distance {
            lowest_distance = *distance;
            lowest_coords = *coords;
        }
    }
    lowest_coords
}

fn path_search_from(mut coords: (usize,usize), mut grid: &mut Vec<Vec<u64>>, explored: &mut HashSet<(usize,usize)>, queue: &mut HashSet<(usize,usize)>, mut distance: &mut HashMap<(usize,usize), u64>, mut queue_with_dist: &mut HashSet<(usize,usize)>) -> (usize,usize) {
    update_distances(&mut coords, &mut grid, &mut distance, &mut queue_with_dist);
    explored.insert(coords);
    queue.remove(&coords);
    queue_with_dist.remove(&coords);
    get_lowest_distance(distance, queue, queue_with_dist)
}

fn run_path_search_iteratively(coords: (usize,usize), mut grid: &mut Vec<Vec<u64>>, mut explored: &mut HashSet<(usize,usize)>, mut queue: &mut HashSet<(usize,usize)>, mut distance: &mut HashMap<(usize,usize), u64>, mut queue_with_dist: &mut HashSet<(usize,usize)>){
    let mut next_coords = coords;
    while next_coords != (grid.len()-1, grid[0].len()-1) {
        next_coords = path_search_from(next_coords, &mut grid, &mut explored, &mut queue, &mut distance, &mut queue_with_dist);
    }
    println!("found path");
    println!("{:?}", distance.get(&next_coords).unwrap());
}

fn update_distances(coords: &mut (usize,usize), grid: &mut Vec<Vec<u64>>, distance: &mut HashMap<(usize,usize), u64>, queue_with_dist: &mut HashSet<(usize,usize)>) {
    if coords.0 > 0 {
        let key = (coords.0 - 1, coords.1);
        let new_dist: u64 = distance[coords] + grid[coords.0 - 1][coords.1];
        if new_dist < distance[&key] {
            distance.insert(key, new_dist);
            queue_with_dist.insert(key);
        }
    }
    if coords.1 > 0 {
        let key = (coords.0, coords.1 - 1);
        let new_dist: u64 = distance[coords] + grid[coords.0][coords.1 - 1];
        if new_dist < distance[&key] {
            distance.insert(key, new_dist);
            queue_with_dist.insert(key);
        }
    }
    if coords.0 < grid.len() - 1 {
        let key = (coords.0 + 1, coords.1);
        let new_dist: u64 = distance[coords] + grid[coords.0 + 1][coords.1];
        if new_dist < distance[&key] {
            distance.insert(key, new_dist);
            queue_with_dist.insert(key);
        }
    }
    if coords.1 < grid[0].len() - 1 {
        let key = (coords.0, coords.1 + 1);
        let new_dist: u64 = distance[coords] + grid[coords.0][coords.1 + 1];
        if new_dist < distance[&key] {
            distance.insert(key, new_dist);
            queue_with_dist.insert(key);
        }
    }
}

fn part_2(grid: Vec<Vec<u64>>) {
    let mut new_grid: Vec<Vec<u64>> = Vec::new();
    for row in grid.iter() {
        let mut new_row = Vec::new();
        let row_plus_1 = row.clone().iter().map(|x| {
            let mut new_x = x + 1;
            if new_x > 9 {
                new_x = 1;
            } 
            new_x
        }).collect::<Vec<u64>>();
        let row_plus_2 = row_plus_1.clone().iter().map(|x| {
            let mut new_x = x + 1;
            if new_x > 9 {
                new_x = 1;
            } 
            new_x
        }).collect::<Vec<u64>>();
        let row_plus_3 = row_plus_2.clone().iter().map(|x| {
            let mut new_x = x + 1;
            if new_x > 9 {
                new_x = 1;
            } 
            new_x
        }).collect::<Vec<u64>>();
        let row_plus_4 = row_plus_3.clone().iter().map(|x| {
            let mut new_x = x + 1;
            if new_x > 9 {
                new_x = 1;
            } 
            new_x
        }).collect::<Vec<u64>>();
        new_row.extend(row);
        new_row.extend(row_plus_1);
        new_row.extend(row_plus_2);
        new_row.extend(row_plus_3);
        new_row.extend(row_plus_4);
        new_grid.push(new_row);
    }
    let mut even_newer_grid: Vec<Vec<u64>> = Vec::new();
    even_newer_grid.extend(new_grid.clone());
    for i in 1..5 {
        let mut grid_to_add: Vec<Vec<u64>> = Vec::new();
        for j in 0..new_grid.len() {
            let row= new_grid[j].iter().map(|x| {
                let mut new_x = x + i ;
                if new_x > 9 {
                    new_x -= 9;
                } 
                new_x
            }).collect::<Vec<u64>>();
            grid_to_add.push(row);
        }
        even_newer_grid.extend(grid_to_add);
    }
    println!("finished creating grid {} x {}", even_newer_grid.len(), even_newer_grid[0].len());
    part_1(even_newer_grid);
}
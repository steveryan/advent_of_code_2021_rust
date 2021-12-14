use std::collections::{HashMap, HashSet};

fn main() -> Result<(), std::io::Error>{
    let now = std::time::Instant::now();
    let file_location = "day_13.txt";
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let contents: String = std::fs::read_to_string(file_location)?;
    let vec = contents
                            .split("\n\n")
                            .collect::<Vec<&str>>();
    let grid = vec[0].clone().split("\n").collect::<Vec<&str>>();
    let instructions =vec[1].split("\n").collect::<Vec<&str>>();

    let grid = grid.iter().map(|x| x.split(",").collect::<Vec<&str>>().iter().map(|y| y.parse().unwrap()).collect()).collect::<Vec<Vec<i32>>>();
    let grid = grid.iter().map(|x| (x[0],x[1])).collect::<Vec<(i32,i32)>>();
    let mut grid_set = HashSet::new();
    for item in grid {
        grid_set.insert(item);
    } 
    let instructions = instructions.iter().map(|x| x.split("fold along ").collect::<Vec<&str>>()[1]).collect::<Vec<&str>>();
    let instructions = instructions.iter().map(|x| x.split("=").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let instructions = instructions.iter().map(|x| (x[0],x[1].parse().unwrap())).collect::<Vec<(&str,i32)>>();
    

    part_1(grid_set, instructions);

    println!("time in milliseconds: {}", now.elapsed().as_millis());
    Ok(())
}

fn part_1(mut grid_set: HashSet<(i32,i32)>, instructions: Vec<(&str,i32)>) {
    for instruction in instructions {
        let first_axis = instruction.0;
        let first_value = instruction.1;
        let mut points = HashSet::new();
        for (x,y) in grid_set.clone() {
            if first_axis == "x" {
                if x < first_value {
                    points.insert((x,y));
                } else if x > first_value {
                    let new_x = 2*first_value - x;
                    points.insert((new_x,y));
                }
            } else {
                if y < first_value {
                    points.insert((x,y));
                } else if y > first_value {
                    let new_y = 2*first_value - y;
                    points.insert((x,new_y));
                }
            }
        }
        grid_set = points;
    }
    // let vec = Vec::from_iter(grid_set.iter().cloned());
    // let mut vec_of_vecs = vec.iter().map(|x| vec![x.0,x.1]).collect::<Vec<Vec<i32>>>();
    // vec_of_vecs.sort();
    // vec_of_vecs.sort_by(|x,y| x[1].cmp(&y[1]));
    let mut graph = vec![vec![" "; 39]; 7];
    for point in grid_set {
        graph[point.1 as usize][point.0 as usize] = "X";
    }
    graph.iter().for_each(|x| {
        println!("{:?}\n", x);
    })
    
}
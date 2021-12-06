use std::collections::HashMap;

fn main() -> Result<(), std::io::Error>{
    let file_location = "problem_5.txt";
    let contents: String = std::fs::read_to_string(file_location)?;
    let mut grid: HashMap<String,i32> = HashMap::new();
    let mut count_of_2_vents = 0;
    let vec = contents
                            .split("\n")
                            .collect::<Vec<&str>>();
    let vec_of_vecs = vec
                                        .iter()
                                        .map(|x| x.split(" -> ")
                                        .collect::<Vec<&str>>())
                                        .collect::<Vec<Vec<&str>>>();
    
    part_2(vec_of_vecs, grid, count_of_2_vents);
    Ok(())
}

fn part_1(vec_of_vecs: Vec<Vec<&str>>, mut grid: HashMap<String, i32>, mut count_of_2_vents: i32){
    for vent_lines in vec_of_vecs{
        let first = vent_lines[0].to_string();
        let second = vent_lines[1].to_string();
        let first_point = first.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let second_point = second.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let x1 = first_point[0];
        let y1 = first_point[1];
        let x2 = second_point[0];
        let y2 = second_point[1];
        if x1 != x2 && y1 != y2 && (x1-x2).abs() != (y1-y2).abs(){
            continue
        } else if x1 == x2 && y1 <= y2{
            for i in y1..y2+1{
                let counter = grid.entry(format!("{} {}", x1, i)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        } else if x1 == x2 && y1 > y2{
            for i in y2..y1+1{
                let counter = grid.entry(format!("{} {}", x1, i)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        } else if y1 == y2 && x1 <= x2{
            for i in x1..x2+1{
                let counter = grid.entry(format!("{} {}", i, y1)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        } else if y1 == y2 && x1 > x2{
            for i in x2..x1+1{
                let counter = grid.entry(format!("{} {}", i, y1)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        } 
    }

    println!("{}", count_of_2_vents);
}

fn part_2(vec_of_vecs: Vec<Vec<&str>>, mut grid: HashMap<String, i32>, mut count_of_2_vents: i32){
    for vent_lines in vec_of_vecs{
        let first = vent_lines[0].to_string();
        let second = vent_lines[1].to_string();
        let first_point = first.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let second_point = second.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let x1 = first_point[0];
        let y1 = first_point[1];
        let x2 = second_point[0];
        let y2 = second_point[1];
        if x1 != x2 && y1 != y2 && (x1-x2).abs() != (y1-y2).abs(){
            continue
        } else if x1 == x2 && y1 <= y2{
            for i in y1..y2+1{
                let counter = grid.entry(format!("{} {}", x1, i)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        } else if x1 == x2 && y1 > y2{
            for i in y2..y1+1{
                let counter = grid.entry(format!("{} {}", x1, i)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        } else if y1 == y2 && x1 <= x2{
            for i in x1..x2+1{
                let counter = grid.entry(format!("{} {}", i, y1)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        } else if y1 == y2 && x1 > x2{
            for i in x2..x1+1{
                let counter = grid.entry(format!("{} {}", i, y1)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        } else if x1 < x2 && y1 < y2{
            let length_of_line = (x2-x1).abs();
            for i in 0..length_of_line+1{
                let counter = grid.entry(format!("{} {}", x1+i, y1+i)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        } else if x1 < x2 && y1 > y2{
            let length_of_line = (x2-x1).abs();
            for i in 0..length_of_line+1{
                let counter = grid.entry(format!("{} {}", x1+i, y1-i)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        } else if x1 > x2 && y1 < y2{
            let length_of_line = (x1-x2).abs();
            for i in 0..length_of_line+1{
                let counter = grid.entry(format!("{} {}", x1-i, y1+i)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        } else if x1 > x2 && y1 > y2{
            let length_of_line = (x1-x2).abs();
            for i in 0..length_of_line+1{
                let counter = grid.entry(format!("{} {}", x1-i, y1-i)).or_insert(0);
                *counter += 1;
                if *counter == 2 as i32{
                    count_of_2_vents += 1;
                }
            }
        }
    }

    println!("{}", count_of_2_vents);
}
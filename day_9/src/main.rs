use std::vec;


fn main() -> Result<(), std::io::Error> {
    let now = std::time::Instant::now();
    let file_location = "day_9.txt";
    let contents: String = std::fs::read_to_string(file_location)?;
    let vec = contents.split("\n").collect::<Vec<&str>>();
    let vec_of_vec: Vec<Vec<u32>> = vec.iter().map(|x| x.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>()).collect();
    part_1(&vec_of_vec);
    Ok(())
}

fn part_1(vec_of_vec: &Vec<Vec<u32>>) {
    let mut low_points = vec![];
    let mut risk_score = 0;
    for (i,row )in vec_of_vec.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if j > 0 && val >= &row[j-1] {
                continue;
            } 
            if i > 0 && val >= &vec_of_vec[i-1][j] {
                continue;
            }
            if j < row.len() - 1 && val >= &row[j+1] {
                continue;
            }
            if i < vec_of_vec.len() - 1 && val >= &vec_of_vec[i+1][j] {
                continue;
            }
            println!("{} is a low point", val);
            let tuple = (i, j);
            low_points.push(tuple);
            risk_score += val + 1;
        }
    }
    println!("{}", risk_score);
    part_2(&low_points, &vec_of_vec);
}

fn part_2(low_points: &Vec<(usize, usize)>, vec_of_vec: &Vec<Vec<u32>>) {
    let mut basin_sizes: Vec<i32> = vec![];
    for low_point in low_points {
        let i = low_point.0;
        let j = low_point.1;
        let mut basin_points = vec![];
        basin_points.push(low_point.clone());
        let points = get_basin_for_point(i, j, &vec_of_vec, basin_points.clone());
        let unique_points = points.iter().collect::<std::collections::HashSet<&(usize, usize)>>();
        basin_sizes.push(unique_points.len() as i32);
    }
    basin_sizes.sort();
    let top_3 = basin_sizes.iter().rev().take(3).collect::<Vec<&i32>>();
    println!("{}", top_3[0] * top_3[1] * top_3[2]);

}

fn get_basin_for_point(i: usize, j: usize, vec_of_vec: &Vec<Vec<u32>>, mut basin_points: Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    if j > 0 && vec_of_vec[i][j-1] < 9 && !basin_points.contains(&(i,j-1)) {
        let new_point = (i, j-1);
        basin_points.push(new_point.clone());
        basin_points.append(&mut get_basin_for_point(i, j-1, &vec_of_vec, basin_points.clone()));
    } 
    if i > 0 && vec_of_vec[i-1][j] < 9 && !basin_points.contains(&(i-1,j)) {
        let new_point = (i-1, j);
        basin_points.push(new_point.clone());
        basin_points.append(&mut get_basin_for_point(i-1, j, &vec_of_vec, basin_points.clone()));
    }
    if j < vec_of_vec[0].len() - 1 && vec_of_vec[i][j+1] < 9 && !basin_points.contains(&(i,j+1)) {
        let new_point = (i, j+1);
        basin_points.push(new_point.clone());
        basin_points.append(&mut get_basin_for_point(i, j+1, &vec_of_vec, basin_points.clone()));
    }
    if i < vec_of_vec.len() - 1 && vec_of_vec[i+1][j] < 9 && !basin_points.contains(&(i+1,j)) {
        let new_point = (i+1, j);
        basin_points.push(new_point.clone());
        basin_points.append(&mut get_basin_for_point(i+1, j, &vec_of_vec, basin_points.clone()));
    }
    basin_points.clone()
}
fn main()->Result<(), std::io::Error> {
    let file_location = "problem_3.txt";
    let contents = std::fs::read_to_string(file_location)?;
    let vec = contents.split("\n").collect::<Vec<&str>>();
    part_2(&vec);
    Ok(())
}

fn part_1(vec: &Vec<&str>) -> () {
    let vec_of_vec = vec.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut counts: Vec<i32> = vec![0; vec_of_vec[0].len()];
    let max_binary_num: i64 = vec!["1"; counts.len()].into_iter().collect::<Vec<&str>>().join("").parse::<i64>().unwrap();
    // let intvec = vec.into_iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    for i in 0..vec_of_vec.len() {
        for j in 0..vec_of_vec[i].len() {
            if vec_of_vec[i][j] == '1' {
                counts[j] += 1;
            }
        }
    }
    
    for i in 0..counts.len() {
        if counts[i] >=((vec.len()/2) as usize).try_into().unwrap() {
            counts[i] = 1;
        } else {
            counts[i] = 0;
        }
    }

    let gamma_binary = counts.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    let gamma = isize::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon_binary = (max_binary_num - gamma_binary.parse::<i64>().unwrap()).to_string();
    let epsilon = isize::from_str_radix(&epsilon_binary, 2).unwrap();
    let result = gamma * epsilon;
    println!("{}", result);
}
fn part_2(vec: &Vec<&str>) -> () {
    let mut oxygen_vecs = vec.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut carbon_vecs = oxygen_vecs.clone();
    for i in 0..oxygen_vecs[0].len() {
        let most_common = get_most_common_char(&oxygen_vecs, i);
        oxygen_vecs = oxygen_vecs.into_iter().filter(|x| x[i] == most_common).collect();
        if oxygen_vecs.len() == 1 {
            break;
        }
    }
    for i in 0..carbon_vecs[0].len() {
        let most_common = get_least_common_char(&carbon_vecs, i);
        carbon_vecs = carbon_vecs.into_iter().filter(|x| x[i] == most_common).collect();
        if carbon_vecs.len() == 1 {
            break;
        }
    }
    let oxygen_binary = oxygen_vecs[0].clone().into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    let oxygen = isize::from_str_radix(&oxygen_binary, 2).unwrap();
    let carbon_binary = carbon_vecs[0].clone().into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    let carbon = isize::from_str_radix(&carbon_binary, 2).unwrap();
    let result = oxygen * carbon;
    println!("{}", result);
}

fn get_most_common_char(vec: &Vec<Vec<char>>, index: usize) -> char {
    let mut count: i32 = 0;
    for i in 0..vec.len() {
        if vec[i][index] == '1' {
            count += 1;
        }
    }
    if count as f32 >= (vec.len() as f32/2.0) {
        return '1';
    } else {
        return '0';
    }
}
fn get_least_common_char(vec: &Vec<Vec<char>>, index: usize) -> char {
    let mut count: i32 = 0;
    for i in 0..vec.len() {
        if vec[i][index] == '1' {
            count += 1;
        }
    }
    if count as f32 >= (vec.len() as f32/2.0) {
        return '0';
    } else {
        return '1';
    }
}
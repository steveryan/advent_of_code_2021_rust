
fn main() -> Result<(), std::io::Error> {
    let now = std::time::Instant::now();
    let file_location = "day_10.txt";
    let contents: String = std::fs::read_to_string(file_location)?;
    let vec = contents.split("\n").collect::<Vec<&str>>();
    let vec_of_vecs = vec.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    part_1(&vec_of_vecs);
    println!("time in microseconds: {}", now.elapsed().as_micros());
    Ok(())
}

fn part_1(vec_of_vecs: &Vec<Vec<char>>) {
    let mut result: i64 = 0;
    let mut incompletes = vec![];
    for vec in vec_of_vecs {
        let mut last_opening_char = Vec::new();
        let mut corrupted = false;
        for char in vec {
            match char {
                '(' => last_opening_char.push('('),
                '[' => last_opening_char.push('['),
                '{' => last_opening_char.push('{'),
                '<' => last_opening_char.push('<'),
                ')'|'}'|']'|'>' => {
                    let last_char = last_opening_char.pop();
                    if last_char.is_none() {break;}
                    let last_char = last_char.unwrap();
                    if last_char == '(' && *char == ')' {
                        continue;
                    } else if last_char == '[' && *char == ']' {
                        continue;
                    } else if last_char == '{' && *char == '}' {
                        continue;
                    } else if last_char == '<' && *char == '>' {
                        continue;
                    } else {
                        corrupted = true;
                        match char {
                            ')' => result += 3,
                            ']' => result += 57,
                            '}' => result += 1197,
                            '>' => result += 25137,
                            _ => (),
                        }
                        break;
                    }
                }
                _ => (),
            }
        }
        if !corrupted && !last_opening_char.is_empty() {
            incompletes.push(last_opening_char);
        }
    }
    part_2(incompletes);
}

fn part_2(mut incompletes: Vec<Vec<char>>) {
    let mut result_vec = Vec::new();
    for incomplete in incompletes.iter_mut() {
        let mut result: i64 = 0;
        incomplete.reverse();
        for char in incomplete {
            match char {
                '(' => result = (result*5) + 1,
                '[' => result = (result*5) + 2,
                '{' => result = (result*5) + 3,
                '<' => result = (result*5) + 4,
                _ => (),
            }
        }
        result_vec.push(result);
    } 
    result_vec.sort();
    let mid = result_vec.len() / 2;
    println!("{}", result_vec[mid]);
}
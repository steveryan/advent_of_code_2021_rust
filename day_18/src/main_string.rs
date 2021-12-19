use std::char;

fn main() -> Result<(), std::io::Error>{
    let now = std::time::Instant::now();
    let file_location = "day_18.txt";
    let contents: String = std::fs::read_to_string(file_location)?;
    let snail_numbers = contents.split('\n').collect::<Vec<&str>>();
    // println!("snail numbers: {:?}", snail_numbers);

    let mut answer: String = "".to_string();
    for number in snail_numbers {
        if answer == "" {
            answer = number.to_string();
        } else {
            answer = format!("[{},{}]", answer, number);
        } 
        // println!("initial answer: {}", answer);
        loop {
            let initial_answer = answer.clone();
            
            // answer = remove_double_brackets(answer);
            answer = explode_answer(answer);
            if answer != initial_answer {
                continue
            } 
            answer = split_answer(answer);
            if answer == initial_answer {
                break;
            }
        }
        println!("answer: {}", answer);
    }
    println!("answer: {}", answer);

    println!("time in seconds: {}", now.elapsed().as_secs());
    Ok(())
}

fn explode_answer(answer: String) -> String {
    let mut left_count = 0;
    let mut left_index = 0;
    let mut left_num_index = 0;
    let mut left_num_chars = vec![];
    let mut left_num = 0;
    let mut right_num_index = 0;
    let mut right_num_chars = vec![];
    let mut right_num = 0;
    let mut right_index = 0;

    for (i,char) in answer.chars().enumerate() {
        if char == '[' {
            left_count += 1;
        }
        if char == ']' {
            left_count -= 1;
        }
        if left_count >= 5  && answer.chars().nth(i+1).unwrap().is_numeric() {
            left_index = i;
            left_num_index = left_index + 1;
            let mut current_index = left_num_index;
            while answer.chars().nth(current_index).unwrap().is_numeric() {
                left_num_chars.push(answer.chars().nth(current_index).unwrap());
                left_num_index += 1;
                current_index += 1;
            }
            left_num = left_num_chars.iter().collect::<String>().parse::<i32>().unwrap();
            right_num_index = left_num_index + 1;
            let mut current_index = right_num_index;
            while answer.chars().nth(current_index).unwrap().is_numeric() {
                right_num_chars.push(answer.chars().nth(current_index).unwrap());
                right_num_index += 1;
                current_index += 1;
            }
            right_index = right_num_index;
            right_num = right_num_chars.iter().collect::<String>().parse::<i32>().unwrap();
            break;
        }

    }
    if right_index != 0 {
        let mut new_answer = answer.clone().chars().collect::<Vec<char>>();
        new_answer.drain(left_index..right_index+1);
        // println!("new answer:     {}", new_answer.iter().collect::<String>());
        new_answer.insert(left_index, '0');
        // println!("new answer:     {}", new_answer.iter().collect::<String>());
        let temp = add_to_num_to_left(new_answer, left_num, left_index-1);
        new_answer = temp.0;
        let chars_inserted = temp.1;
        // println!("new answer:     {}", new_answer.iter().collect::<String>());
        // println!("righ_num:       {}", right_num);
        new_answer = add_to_num_to_right(new_answer, right_num, left_index+1+chars_inserted);
        // println!("after explode:  {}", new_answer.iter().collect::<String>());
        return new_answer.iter().collect::<String>()
    }
    answer
}

fn split_answer(answer: String) -> String {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut number_chars = vec![];

    for (i,char) in answer.chars().enumerate() {
        if char.is_numeric() && i+1 < answer.len()-1 && answer.chars().nth(i+1).unwrap().is_numeric() {
            left_index = i;
            let mut current_index = i;
            while answer.chars().nth(current_index).unwrap().is_numeric() {
                number_chars.push(answer.chars().nth(current_index).unwrap());
                right_index =current_index;
                current_index += 1;
            }
            break;
        }

    }
    if right_index != 0 {
        let number = number_chars.iter().collect::<String>().parse::<i32>().unwrap();
        let left_num = (number as f64/2.0).floor() as i32;
        let right_num = (number as f64/2.0).ceil() as i32;
        let mut left_num_chars = left_num.to_string().chars().collect::<Vec<char>>();
        let mut right_num_chars = right_num.to_string().chars().collect::<Vec<char>>();
        let mut chars_to_add = vec!['['];
        chars_to_add.append(&mut left_num_chars);
        chars_to_add.push(',');
        chars_to_add.append(&mut right_num_chars);
        chars_to_add.push(']');
        // println!("splitting in progress");
        let mut new_answer = answer.clone().chars().collect::<Vec<char>>();
        // println!("new answer: {:?}", new_answer.iter().collect::<String>());
        new_answer.drain(left_index..right_index+1);
        for (i,char) in chars_to_add.iter().enumerate() {
            new_answer.insert(left_index+i, *char);
        }
        // println!("after split:    {}", new_answer.iter().collect::<String>());
        return new_answer.iter().collect::<String>()
    }
    answer
}

// fn remove_double_brackets(mut answer: String) -> String {
//     let mut new_answer = answer.clone();
//     let mut left_index = 0;
//     let mut right_index = 0;
//     for (i,char) in answer.chars().enumerate() {
//         if char == '[' && i+6 < answer.len() && 
//         answer.chars().nth(i+1).unwrap() == '[' && 
//         answer.chars().nth(i+2).unwrap().is_numeric() && 
//         answer.chars().nth(i+3).unwrap() == ',' &&
//         answer.chars().nth(i+4).unwrap().is_numeric() &&
//         answer.chars().nth(i+5).unwrap() == ']' &&
//         answer.chars().nth(i+6).unwrap() == ']' {
//             left_index = i;
//             right_index = i+6;
//             break;
//         }
//     }
//     if right_index != 0 {
//         answer.remove(left_index);
//         answer.remove(right_index-1);
//     }

//     answer
// }

fn add_to_num_to_left(answer: Vec<char>, num: i32, left_index: usize) -> (Vec<char>, usize) {
    let mut new_answer = answer.clone();
    let mut index: i32 = left_index as i32;
    let mut chars_inserted = 0;
    while index >= 0 {
        if answer[index as usize].is_numeric() {
            new_answer.remove(index as usize);
            let new_value = (answer[index as usize].to_digit(10).unwrap() as i32 + num).to_string();
            chars_inserted = new_value.len();
            for (i,char) in new_value.chars().enumerate() {
                new_answer.insert(index as usize+i , char);
            }
            break;
        }
        index -= 1;
    }
    (new_answer, chars_inserted)
}

fn add_to_num_to_right(answer: Vec<char>, num: i32, right_index: usize) -> Vec<char> {
    let mut new_answer = answer.clone();
    let mut index = right_index;
    while index <= answer.len() - 1 {
        if answer[index].is_numeric() {
            new_answer.remove(index);
            let new_value = (answer[index].to_digit(10).unwrap() as i32 + num).to_string();
            for (i,char) in new_value.chars().enumerate() {
                new_answer.insert(index+i, char);
            }
            break;
        }
        index += 1;
    }
    new_answer
}



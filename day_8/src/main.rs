use std::collections::HashMap;

fn main() -> Result<(), std::io::Error>{
    let now = std::time::Instant::now();
    let file_location = "day_8.txt";
    let contents: String = std::fs::read_to_string(file_location)?;
    let vec = contents.split("\n").collect::<Vec<&str>>();
    let output = vec.iter().map(|x| x.split(" | ").collect::<Vec<&str>>()[1]).collect::<Vec<&str>>();
    part_2(&vec);
    println!("Time elapsed: {} ms", now.elapsed().as_millis());
    Ok(())
}

fn part_1(output: &Vec<&str>) {
    let mut counts = HashMap::new();
    for line in output {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        for word in words {
            match word.chars().collect::<Vec<char>>().len() {
                2 => {
                    let count = counts.entry(1).or_insert(0);
                    *count += 1;
                },
                3 => {
                    let count = counts.entry(7).or_insert(0);
                    *count += 1;
                },
                4 => {
                    let count = counts.entry(4).or_insert(0);
                    *count += 1;
                },
                7 => {
                    let count = counts.entry(8).or_insert(0);
                    *count += 1;
                },
                _ => {
                    continue;
                },
            }
        }
    }
    println!("{:?}", counts.values().sum::<i32>());
}

fn part_2(vec: &Vec<&str>) {
    let mut final_count: i64 = 0;
    for line in vec {
        let output = line.split(" | ").collect::<Vec<&str>>()[1];
        let input = line.split(" | ").collect::<Vec<&str>>()[0];
        let mut chars_1: Vec<char> = Vec::new();
        let mut chars_2: Vec<char> = Vec::new();
        let mut chars_3: Vec<char> = Vec::new();
        let mut chars_4: Vec<char> = Vec::new();
        let mut chars_5: Vec<char> = Vec::new();
        let mut chars_6: Vec<char> = Vec::new();
        let mut chars_7: Vec<char> = Vec::new();
        let mut chars_8: Vec<char> = Vec::new();
        let mut chars_9: Vec<char> = Vec::new();
        let mut chars_0: Vec<char> = Vec::new();
        let words = input.split_whitespace().collect::<Vec<&str>>();
        for word in &words {
            match word.chars().collect::<Vec<char>>().len() {
                2 => {
                    chars_1 = word.chars().collect::<Vec<char>>();
                },
                3 => {
                    chars_7 = word.chars().collect::<Vec<char>>();
                },
                4 => {
                    chars_4 = word.chars().collect::<Vec<char>>();
                },
                7 => {
                    chars_8 = word.chars().collect::<Vec<char>>();
                },
                _ => {
                    continue;
                },
            }
        }
        let temp_vec_1 = words.clone();
        let temp_vec_2 = words.clone();
        let six_nine_or_zero: Vec<&&str> = temp_vec_1.iter().filter(|word| word.chars().collect::<Vec<char>>().len() == 6).collect::<Vec<&&str>>();
        let two_three_or_five: Vec<&&str> = temp_vec_2.iter().filter(|word| word.chars().collect::<Vec<char>>().len() == 5).collect::<Vec<&&str>>();
        let mut nine_or_zero: Vec<&&str> = six_nine_or_zero.clone();
        let mut two_or_five: Vec<&&str> = two_three_or_five.clone();

        for (i,word) in six_nine_or_zero.iter().enumerate() {
            if !word.chars().collect::<Vec<char>>().contains(&chars_7[0]) || !word.chars().collect::<Vec<char>>().contains(&chars_7[1]) || !word.chars().collect::<Vec<char>>().contains(&chars_7[2]) {
                chars_6 = word.chars().collect::<Vec<char>>();
                nine_or_zero.retain(|x| x != word);
            }
        }
        for (i,word) in two_three_or_five.iter().enumerate() {
            if word.chars().collect::<Vec<char>>().contains(&chars_7[0]) && word.chars().collect::<Vec<char>>().contains(&chars_7[1]) && word.chars().collect::<Vec<char>>().contains(&chars_7[2]) {
                chars_3 = word.chars().collect::<Vec<char>>();
                two_or_five.retain(|x| x != word);
            }
        }
        let mut zero = nine_or_zero.clone();
        for (i,word) in two_or_five.iter().enumerate() {
            let mut count = 0;
            for char in word.chars() {
                if chars_4.contains(&char) {
                    count += 1;
                }
            }
            if count == 2 {
                chars_2 = word.chars().collect::<Vec<char>>();
            } else if count == 3 {
                chars_5 = word.chars().collect::<Vec<char>>();
            } else {
                println!("something has gone very wrong")
            }
        }
        for (i,word) in nine_or_zero.iter().enumerate() {
            if word.chars().collect::<Vec<char>>().contains(&chars_5[0]) && word.chars().collect::<Vec<char>>().contains(&chars_5[1]) && word.chars().collect::<Vec<char>>().contains(&chars_5[2]) && word.chars().collect::<Vec<char>>().contains(&chars_5[3]) && word.chars().collect::<Vec<char>>().contains(&chars_5[4]) {
                chars_9 = word.chars().collect::<Vec<char>>();
                zero.retain(|x| x != word);
                chars_0 = zero[0].chars().collect::<Vec<char>>();
            }
        }

        let words = output.split_whitespace().collect::<Vec<&str>>();
        let mut nums = Vec::new();
        for word in &words {
            if word.chars().collect::<Vec<char>>().len() == 2 {
                nums.push("1");
            } else if word.chars().collect::<Vec<char>>().len() == 3 {
                nums.push("7");
            } else if word.chars().collect::<Vec<char>>().len() == 4 {
                nums.push("4");
            } else if word.chars().collect::<Vec<char>>().len() == 7 {
                nums.push("8");
            } else if word.chars().collect::<Vec<char>>().len() == 6 {
                if word.chars().collect::<Vec<char>>().iter().all(|&item|  chars_6.contains(&item)) {
                    nums.push("6");
                } else if word.chars().collect::<Vec<char>>().iter().all(|&item|  chars_9.contains(&item)) {
                    nums.push("9");
                } else if word.chars().collect::<Vec<char>>().iter().all(|&item|  chars_0.contains(&item)) {
                    nums.push("0");
                }
            } else if word.chars().collect::<Vec<char>>().len() == 5 {
                if word.chars().collect::<Vec<char>>().iter().all(|&item|  chars_2.contains(&item)) {
                    nums.push("2");
                } else if word.chars().collect::<Vec<char>>().iter().all(|&item|  chars_3.contains(&item)) {
                    nums.push("3");
                } else if word.chars().collect::<Vec<char>>().iter().all(|&item|  chars_5.contains(&item)) {
                    nums.push("5");
                }
            } else {
                println!("something has gone very wrong")
            }
            
        }
        final_count += nums.join::<&str>("").parse::<i64>().unwrap()
    }
    println!("{}", final_count);
}
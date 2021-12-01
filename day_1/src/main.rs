fn main() -> Result<(), std::io::Error>{
    let file_location = "problem_1.txt";
    let contents = std::fs::read_to_string(file_location)?;
    let vec = contents.split("\n").collect::<Vec<&str>>();
    let mut current_depth: Option<i32> = None;
    let mut increase_count = 0;
    for line in vec {
        let depth = line.parse::<i32>().unwrap();
        if current_depth.is_some() {
            let current = current_depth.unwrap();
            if current < depth {
                increase_count += 1;
            }
        }
        current_depth = Some(depth);
    }
    println!("{}", increase_count);
    Ok(())
}

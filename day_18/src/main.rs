
fn main() -> Result<(),std::io::Error>{
    let now = std::time::Instant::now();
    let file_location = "day_18.txt";
    let mut contents: String = std::fs::read_to_string(file_location)?;
    contents = "{".to_string() + &contents + "}";
    let deserialized = json::parse(&contents).unwrap();
    println!("{}", deserialized);
    let snail_numbers = contents.split('\n').collect::<Vec<&str>>();
    // println!("snail numbers: {:?}", snail_numbers);
    Ok(())
}
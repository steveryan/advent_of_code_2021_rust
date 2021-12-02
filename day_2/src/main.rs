fn main()->Result<(), std::io::Error> {
    let file_location = "problem_2.txt";
    let contents = std::fs::read_to_string(file_location)?;
    let vec = contents.split("\n").collect::<Vec<&str>>();
    let mut depth = 0;
    let mut horizontal_sum = 0;
    let mut aim = 0;

    for line in &vec {
        let line_vec = line.split(" ").collect::<Vec<&str>>();
        let direction = line_vec[0];
        let distance = line_vec[1].parse::<i32>().unwrap();
        if direction == "forward" {
            horizontal_sum += distance;
            depth += aim * distance;
        } else if direction == "down" {
            aim += distance;
        } else if direction == "up" {
            aim -= distance;
        }
    }

    let distance = depth.abs() * horizontal_sum.abs();



    println!("{:?}" , distance);
    Ok(())
}

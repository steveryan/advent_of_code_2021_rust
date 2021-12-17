use std::collections::HashSet;

fn main() {
    let now = std::time::Instant::now();
    let x_min = 209;
    let x_max = 238;
    let y_min = -86;
    let y_max = -59;
    let x_velocity_range = Vec::from_iter(15..x_max+1);
    let y_velocity_range = Vec::from_iter(y_min..y_min*-1);
    let mut max_y = 0;
    let mut velocities_that_successfully_reached_target = HashSet::new();
    for x in x_velocity_range {
        for y in &y_velocity_range {
            let mut x_position = 0;
            let mut x_velocity = x.clone();
            let mut y_position = 0;
            let mut y_velocity = y.clone();
            let mut max_y_for_iteration = 0;
            loop {
                x_position += x_velocity;
                y_position += y_velocity;
                if y_position > max_y_for_iteration {
                    max_y_for_iteration = y_position;
                }
                if x_position >= x_min && x_position <= x_max && y_position >= y_min && y_position <= y_max {
                    velocities_that_successfully_reached_target.insert((x, y));
                    if max_y_for_iteration > max_y {
                        max_y = max_y_for_iteration;
                    }
                    break;
                }
                if x_position > x_max || y_position < y_min {
                    break;
                }
                x_velocity -= 1;
                if x_velocity <= 0 {
                    x_velocity = 0;
                }
                y_velocity -= 1;
            }
        }
    }
    println!("Time taken: {}", now.elapsed().as_micros());
    println!("Max y: {}", max_y);
    println!("number of velocities that successfully reached target: {}", velocities_that_successfully_reached_target.len());
}

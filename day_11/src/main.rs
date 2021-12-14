fn main() -> Result<(), std::io::Error> {
    let now = std::time::Instant::now();
    let file_location = "day_11.txt";
    let contents: String = std::fs::read_to_string(file_location)?;
    let vec = contents.split("\n").collect::<Vec<&str>>();
    let vec_of_vecs = vec.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut vec_of_octopi = vec_of_vecs.iter().map(|x| x.iter().map(|y| y.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    part_1(vec_of_octopi);
    println!("time in microseconds: {}", now.elapsed().as_micros());
    Ok(())
}

fn part_1(mut vec_of_octopi: Vec<Vec<u32>>) {
    let mut flash_count = 0;
    for i in 1..500 {
        let mut flashed = std::collections::HashSet::new();
        for (i,row) in vec_of_octopi.iter_mut().enumerate() {
            for (j,octopus) in row.iter_mut().enumerate() {
                *octopus += 1;
                if *octopus > 9 {
                    flashed.insert((i,j));
                    flash_count += 1;
                }
            }
        }
        let mut first_flashes = flashed.clone();
        handle_flashed(&mut vec_of_octopi, &mut flashed, &mut first_flashes, &mut flash_count);
        if vec_of_octopi.iter().all(|row| row.iter().all(|octo| *octo == 0)) {
            println!("found one! {}", i);
            break
        }
    }
    println!("total flash count: {}", flash_count);
}

fn handle_flashed(mut vec_of_octopi: &mut Vec<Vec<u32>>, mut flashed: &mut std::collections::HashSet<(usize, usize)>, mut difference: &mut std::collections::HashSet<(usize, usize)>, mut flash_count: &mut i64)  {
    let initial_flashed = flashed.clone();
    for (i,j) in difference.clone() {
        if i > 0 {
            vec_of_octopi[i-1][j] += 1;
            if vec_of_octopi[i-1][j] > 9 {
                let res = flashed.insert(((i-1),j));
                if res {*flash_count += 1;}
            }
        }
        if i < (vec_of_octopi.len() - 1) {
            vec_of_octopi[i+1][j] += 1;
            if vec_of_octopi[i+1][j] > 9 {
                let res = flashed.insert(((i+1),j));
                if res {*flash_count += 1;}            
            }
        }
        if j > 0 {
            vec_of_octopi[i][j-1] += 1;
            if vec_of_octopi[i][j-1] > 9 {
                let res = flashed.insert((i,(j-1)));
                if res {*flash_count += 1;}
            }
        }
        if j < (vec_of_octopi[0].len() - 1 ) {
            vec_of_octopi[i][j+1] += 1;
            if vec_of_octopi[i][j+1] > 9 {
                let res = flashed.insert((i,(j+1)));
                if res {*flash_count += 1;}
            }
        }
        if i > 0 && j > 0 {
            vec_of_octopi[i-1][j-1] += 1;
            if vec_of_octopi[i-1][j-1] > 9 {
                let res = flashed.insert(((i-1),(j-1)));
                if res {*flash_count += 1;}
            }
        }
        if i > 0 && j < (vec_of_octopi[0].len() - 1) {
            vec_of_octopi[i-1][j+1] += 1;
            if vec_of_octopi[i-1][j+1] > 9 {
                let res = flashed.insert(((i-1),(j+1)));
                if res {*flash_count += 1;}
            }
        }
        if i < (vec_of_octopi.len() - 1) && j > 0 {
            vec_of_octopi[i+1][j-1] += 1;
            if vec_of_octopi[i+1][j-1] > 9 {
                let res = flashed.insert(((i+1),(j-1)));
                if res {*flash_count += 1;}
            }
        }
        if i < (vec_of_octopi.len() - 1) && j < (vec_of_octopi[0].len() - 1) {
            vec_of_octopi[i+1][j+1] += 1;
            if vec_of_octopi[i+1][j+1] > 9 {
                let res = flashed.insert(((i+1),(j+1)));
                if res {*flash_count += 1;}
            }
        }
    }

    let diff = flashed.difference(&initial_flashed).cloned().collect::<Vec<(usize, usize)>>();
    let mut diff: std::collections::HashSet<(usize,usize)> = std::collections::HashSet::from_iter(diff);
    if !diff.is_empty() {
        handle_flashed(&mut vec_of_octopi, &mut flashed, &mut diff, &mut flash_count)
    }
    for (i,j) in flashed.clone() {
        vec_of_octopi[i][j] = 0;
    }
}

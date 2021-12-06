#[derive(Debug)]
struct Fish {
    counter: i32
}

impl Fish {
    fn new(counter: i32) -> Fish {
        Fish { counter }
    }

    fn age(&mut self) {
        if self.counter == 0 {
            self.counter = 6;
        } else {
            self.counter -= 1;
        }
    }
}

fn main() {
    let mut fish_born = 0;
    // let initial = vec![3,5,3,1,4,4,5,5,2,1,4,3,5,1,3,5,3,2,4,3,5,3,1,1,2,1,4,5,3,1,4,5,4,3,3,4,3,1,1,2,2,4,1,1,4,3,4,4,2,4,3,1,5,1,2,3,2,4,4,1,1,1,3,3,5,1,4,5,5,2,5,3,3,1,1,2,3,3,3,1,4,1,5,1,5,3,3,1,5,3,4,3,1,4,1,1,1,2,1,2,3,2,2,4,3,5,5,4,5,3,1,4,4,2,4,4,5,1,5,3,3,5,5,4,4,1,3,2,3,1,2,4,5,3,3,5,4,1,1,5,2,5,1,5,5,4,1,1,1,1,5,3,3,4,4,2,2,1,5,1,1,1,4,4,2,2,2,2,2,5,5,2,4,4,4,1,2,5,4,5,2,5,4,3,1,1,5,4,5,3,2,3,4,1,4,1,1,3,5,1,2,5,1,1,1,5,1,1,4,2,3,4,1,3,3,2,3,1,1,4,4,3,2,1,2,1,4,2,5,4,2,5,3,2,3,3,4,1,3,5,5,1,3,4,5,1,1,3,1,2,1,1,1,1,5,1,1,2,1,4,5,2,1,5,4,2,2,5,5,1,5,1,2,1,5,2,4,3,2,3,1,1,1,2,3,1,4,3,1,2,3,2,1,3,3,2,1,2,5,2];
    let initial = vec![3,4,3,1,2];
    let mut fish_vec = initial.iter().map(|&x| Fish::new(x)).collect::<Vec<Fish>>();
    // println!("Initial state: {:?}", fish_vec);
    for i in 1..257 {
        println!("{}", i);
        for _ in 0..fish_born {
            let new_fish = Fish::new(9);
            fish_vec.push(new_fish);
        }
        fish_born = 0;
        for fish in &mut fish_vec {
            fish.age();
            if fish.counter == 0 {
                fish_born += 1;
            }
        }
        // println!("After {} days: {:?}", i, fish_vec);
    }
    println!("{:?}", fish_vec.len());
}

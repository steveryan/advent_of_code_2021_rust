use std::iter::Enumerate;

use json::{self, JsonValue};
#[derive(Debug)]
pub struct SnailNumber {
    first: Box<snvalue>,
    second: Box<snvalue>,
    exploded: bool
}

impl SnailNumber {
    pub fn add(self, other: SnailNumber) -> SnailNumber {
        SnailNumber { 
            first: Box::new(snvalue::SnailNumber(self)),
            second: Box::new(snvalue::SnailNumber(other)),
            exploded: false
        }
    }

    pub fn mark_explosion(mut self, level: i32) -> SnailNumber {
        let mut explosion_happened = false;
        self.first = match *self.first {
            snvalue::SnailNumber(sn) => {
                Box::new(snvalue::SnailNumber(sn.mark_explosion(level+1)))
            },
            snvalue::Value(n) => {
                if level >= 4{
                    self.exploded = true;
                    explosion_happened = true;
                }
                Box::new(snvalue::Value(n))
            }
        };
        if !explosion_happened {
            self.second = match *self.second {
                snvalue::SnailNumber(sn) => {
                    Box::new(snvalue::SnailNumber(sn.mark_explosion(level+1)))
                },
                snvalue::Value(n) => {
                    if level >= 4{
                        self.exploded = true;
                        explosion_happened = true;
                    }
                    Box::new(snvalue::Value(n))
                }
            };
        }
        self
    }

    pub fn perform_explosion(self) -> (Option<Box<snvalue>>, Option<Box<snvalue>>, Option<Box<snvalue>>, Option<Box<snvalue>>) {
        let mut first_value = None;
        let mut second_value = None;
        let mut add_to_left = None;
        let mut add_to_right = None;
        let mut explosion_happened = false;
        match *self.first {
            snvalue::SnailNumber(sn) => {
                if sn.exploded {
                    add_to_left = Some(sn.first);
                    add_to_right = Some(sn.second);
                    explosion_happened = true;
                } else {
                    let result = sn.perform_explosion();
                    let new_first = result.0;
                    let new_second = result.1;
                    add_to_left = result.2;
                    add_to_right = result.3;
                    if new_second.is_none() {
                        first_value = new_first;
                    } else {
                        first_value = Some(Box::new(snvalue::SnailNumber(SnailNumber {
                            first: new_first.unwrap(),
                            second: new_second.unwrap(),
                            exploded: false
                        })));
                    }
                }
            },
            snvalue::Value(n) => {
                first_value = Some(Box::new(snvalue::Value(n)));
            }
        }
        if explosion_happened {
            match *self.second {
                snvalue::SnailNumber(sn) => {
                    sn.add_to_left(add_to_right);
                    add_to_right = None;
                    first_value = Some(sn.first);
                    second_value = Some(sn.second);
                },
                snvalue::Value(n) => {
                    let add_to_n = 0;
                    if add_to_right.is_some() {
                        match *add_to_right.unwrap() {
                            snvalue::Value(x) => {
                                add_to_n = x;
                                add_to_right = None;
                            },
                            snvalue::SnailNumber(sn) => {
                                unreachable!()
                            }
                        }
                    }
                    first_value = Some(Box::new(snvalue::Value(n+add_to_n)));
                }
            }
        } else {
            match *self.second {
                snvalue::SnailNumber(sn) => {
                    if sn.exploded {
                        add_to_left = Some(sn.first);
                        add_to_right = Some(sn.second);
                        explosion_happened = true;
                    } else {
                        let result = sn.perform_explosion();
                        let new_first = result.0;
                        let new_second = result.1;
                        add_to_left = result.2;
                        add_to_right = result.3;
                        if new_second.is_none() {
                            second_value = new_first;
                        } else {
                            second_value = Some(Box::new(snvalue::SnailNumber(SnailNumber {
                                first: new_first.unwrap(),
                                second: new_second.unwrap(),
                                exploded: false
                            })));
                        }
                    }
                },
                snvalue::Value(n) => {
                    second_value = Some(Box::new(snvalue::Value(n)));
                }
            }
            if explosion_happened && first_value.is_some(){
                let to_match = first_value.unwrap();
                match *to_match {
                    snvalue::SnailNumber(sn) => {
                        sn.add_to_right(add_to_left);
                        add_to_left = None;
                        first_value = Some(sn.first);
                        second_value = Some(sn.second);
                    },
                    snvalue::Value(n) => {
                        let add_to_n = 0;
                        if add_to_left.is_some() {
                            match *add_to_left.unwrap() {
                                snvalue::Value(x) => {
                                    add_to_n = x;
                                    add_to_left = None;
                                },
                                snvalue::SnailNumber(sn) => {
                                    unreachable!()
                                }
                            }
                        }
                        first_value = Some(Box::new(snvalue::Value(n+add_to_n)));
                    }
                }
            }
        }
        (first_value, second_value, add_to_left, add_to_right)
    }

}

#[derive(Debug)]
pub enum snvalue {
    SnailNumber(SnailNumber),
    Value(i32),
}
fn main() -> Result<(),std::io::Error>{
    let now = std::time::Instant::now();
    let file_location = "day_18.txt";
    let mut contents: String = std::fs::read_to_string(file_location)?;
    let snail_numbers = contents.split('\n').collect::<Vec<&str>>();
    // let mut snail_numbers_vec = vec![];
    for number in snail_numbers {
        let content = "{\"snail_numbers\": ".to_string() + &number + "}";
        let deserialized = json::parse(&content).unwrap();
        let snail_num = get_snail_num(deserialized["snail_numbers"].clone());
        println!("{:?}", snail_num);
    }
    
    // println!("{:?}", snail_numbers_vec);
    // println!("snail numbers: {:?}", snail_numbers);
    Ok(())
}

fn get_snail_num(input: JsonValue) -> SnailNumber {
    let first_val = if input[0].is_array() {
        snvalue::SnailNumber(get_snail_num(input[0].clone()))
    } else {
        snvalue::Value(input[0].as_i32().unwrap())
    };
    let second_val = if input[1].is_array() {
        snvalue::SnailNumber(get_snail_num(input[1].clone()))
    } else {
        snvalue::Value(input[1].as_i32().unwrap())
    };
    SnailNumber {
        first: Box::new(first_val),
        second: Box::new(second_val),
        exploded: false
    }
}
use std::collections::HashMap;

fn main() {
    let input = "D2FE28";
    let binary = get_binary(input);
    println!("{}", binary);
    part_1(binary);
}

fn part_1(binary: String) {
    let packet_version = from_binary(binary[0..3].to_string());
    let packet_type = from_binary(binary[3..6].to_string());
    let remaining = binary[6..].to_string();
    if packet_type == "4" {
        handle_packet_4(remaining);
    } else {
        handle_packet(remaining);
    }
    println!("Packet version: {}", packet_version);
    println!("Packet type: {}", packet_type);
}

fn get_binary(input: &str) -> String {
    let mut conv = HashMap::new();
    let mut new_string = String::new();
    conv.insert('0', "0000");
    conv.insert('1', "0001");
    conv.insert('2', "0010");
    conv.insert('3', "0011");
    conv.insert('4', "0100");
    conv.insert('5', "0101");
    conv.insert('6', "0110");
    conv.insert('7', "0111");
    conv.insert('8', "1000");
    conv.insert('9', "1001");
    conv.insert('A', "1010");
    conv.insert('B', "1011");
    conv.insert('C', "1100");
    conv.insert('D', "1101");
    conv.insert('E', "1110");
    conv.insert('F', "1111");
    for c in input.chars() {
        new_string.push_str(&conv[&c]);
    }
    new_string
}

fn from_binary(input: String) -> String {
    let mut conv: HashMap<&str, char> = HashMap::new();
    let input_str: &str = &input;
    conv.insert("000", '0');
    conv.insert("001", '1');
    conv.insert("010", '2');
    conv.insert("011", '3');
    conv.insert("100", '4');
    conv.insert("101", '5');
    conv.insert("110", '6');
    conv.insert("111", '7');
    conv.insert("0000", '0');
    conv.insert("0001", '1');
    conv.insert("0010", '2');
    conv.insert("0011", '3');
    conv.insert("0100", '4');
    conv.insert("0101", '5');
    conv.insert("0110", '6');
    conv.insert("0111", '7');
    conv.insert("1000", '8');
    conv.insert("1001", '9');
    conv.insert("1010", 'A');
    conv.insert("1011", 'B');
    conv.insert("1100", 'C');
    conv.insert("1101", 'D');
    conv.insert("1110", 'E');
    conv.insert("1111", 'F');
    conv[input_str].to_string()
}

fn handle_packet_4(input: String) {
    let mut literal_binary = String::new();
    let mut inputs = input.chars()
    .collect::<Vec<char>>()
    .chunks(5)
    .map(|c| c.iter().collect::<String>())
    .collect::<Vec<String>>();
    if inputs.last().unwrap().chars().all(|c| c == '0') {
        inputs.remove(inputs.len() - 1);
    } 
    for input in &inputs {
        if input.chars().collect::<Vec<char>>().first().unwrap() == &'0' {
            literal_binary.push_str(&input[1..]);
        } else {
            literal_binary.push_str(&input[1..]);
        }
    };
    println!("{:?}", isize::from_str_radix(&literal_binary, 2).unwrap());
}

fn handle_packet(input: String) {
    
}




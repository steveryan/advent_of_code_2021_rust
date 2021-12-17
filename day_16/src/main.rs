use std::collections::HashMap;

fn main() {
    let input = "620080001611562C8802118E34";
    let binary = get_binary(input);
    part_1(binary);
}

fn part_1(binary: String) {
    let mut version_count: i64 = 0;
    generic_handle_packet(binary, &mut version_count);
    println!("Part 1: {}", version_count);
}

fn generic_handle_packet(remaining: String, version_count: &mut i64) -> String {
    println!("Called generic handle packet with: {}", remaining);
    if remaining.len() < 3 { 
        return "".to_string() 
    }
    let mut packet_version = from_binary(remaining[0..3].to_string());
    *version_count += packet_version.parse::<i64>().unwrap();
    println!("version count: {}", version_count);
    let packet_type = from_binary(remaining[3..6].to_string());
    let mut remaining = remaining[6..].to_string();
    
    if packet_type == "4" {
        remaining = handle_packet_4(remaining, version_count)
    } else {
        remaining = handle_packet(remaining, version_count)
    }
    println!("Remaining: {}", remaining);
    return remaining;
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
    conv[input_str].to_string()
}

fn handle_packet_4(input: String, mut version_count: &mut i64) -> String {
    println!("found a literal packet: {}", input);
    let mut literal_binary = String::new();
    let mut ending_index = 0;
    let mut inputs = input.chars()
    .collect::<Vec<char>>()
    .chunks(5)
    .map(|c| c.iter().collect::<String>())
    .collect::<Vec<String>>();
    for (i, input) in inputs.iter().enumerate() {
        if input.chars().collect::<Vec<char>>().first().unwrap() == &'0' {
            literal_binary.push_str(&input[1..]);
            ending_index = i;
            break;
        } else {
            literal_binary.push_str(&input[1..]);
            ending_index = i;
        }
    };
    inputs = inputs[ending_index+1..].to_vec();
    if inputs.join("").chars().collect::<Vec<char>>().len() > 3 {
        return inputs.join("").to_string()
    }
    "".to_string()
}

fn handle_packet(input: String, mut version_count: &mut i64) -> String {
    println!("found an operator packet: {}", input);
    let length_id = input[0..1].to_string();
    let mut remaining = input[1..].to_string();
    if length_id == "0" {
        if remaining.chars().collect::<Vec<char>>().len() < 15 {
            return "".to_string();
        }
        let bits = remaining[0..15].to_string();
        let size = i64::from_str_radix(&bits, 2).unwrap();
        println!("size: {}", size);
        let end: usize = (15 + size).try_into().unwrap();
        let mut remains = remaining[15..end].to_string();
        let to_return = remaining[end..].to_string();
        println!("to return: {}", to_return);
        while remains.chars().collect::<Vec<char>>().len() > 3 {
            remains = generic_handle_packet(remains, &mut version_count);
        }
            if to_return.chars().collect::<Vec<char>>().len() > 3 {
                generic_handle_packet(to_return, &mut version_count);
            }
    } else {
        if remaining.chars().collect::<Vec<char>>().len() < 11 {
            return "".to_string();
        }
        let packets = remaining[0..11].to_string();
        let num_packets = i64::from_str_radix(&packets, 2).unwrap();
        println!("num packets: {}", num_packets);
        remaining = remaining[11..].to_string();
        let mut count_of_packets = 1;
        while count_of_packets < num_packets {
            println!("checking packet {} of {}", count_of_packets, num_packets);
            println!("remaining: {}", remaining);
            remaining = generic_handle_packet(remaining, &mut version_count);
            count_of_packets += 1;
        }
    }
    remaining
}




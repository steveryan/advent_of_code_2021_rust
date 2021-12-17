use std::{collections::HashMap, vec};

fn main() {
    let input = "2052ED9802D3B9F465E9AE6003E52B8DEE3AF97CA38100957401A88803D05A25C1E00043E1545883B397259385B47E40257CCEDC7401700043E3F42A8AE0008741E8831EC8020099459D40994E996C8F4801CDC3395039CB60E24B583193DD75D299E95ADB3D3004E5FB941A004AE4E69128D240130D80252E6B27991EC8AD90020F22DF2A8F32EA200AC748CAA0064F6EEEA000B948DFBED7FA4660084BCCEAC01000042E37C3E8BA0008446D8751E0C014A0036E69E226C9FFDE2020016A3B454200CBAC01399BEE299337DC52A7E2C2600BF802B274C8848FA02F331D563B3D300566107C0109B4198B5E888200E90021115E31C5120043A31C3E85E400874428D30AA0E3804D32D32EED236459DC6AC86600E4F3B4AAA4C2A10050336373ED536553855301A600B6802B2B994516469EE45467968C016D004E6E9EE7CE656B6D34491D8018E6805E3B01620C053080136CA0060801C6004A801880360300C226007B8018E0073801A801938004E2400E01801E800434FA790097F39E5FB004A5B3CF47F7ED5965B3CF47F7ED59D401694DEB57F7382D3F6A908005ED253B3449CE9E0399649EB19A005E5398E9142396BD1CA56DFB25C8C65A0930056613FC0141006626C5586E200DC26837080C0169D5DC00D5C40188730D616000215192094311007A5E87B26B12FCD5E5087A896402978002111960DC1E0004363942F8880008741A8E10EE4E778FA2F723A2F60089E4F1FE2E4C5B29B0318005982E600AD802F26672368CB1EC044C2E380552229399D93C9D6A813B98D04272D94440093E2CCCFF158B2CCFE8E24017CE002AD2940294A00CD5638726004066362F1B0C0109311F00424CFE4CF4C016C004AE70CA632A33D2513004F003339A86739F5BAD5350CE73EB75A24DD22280055F34A30EA59FE15CC62F9500";
    let binary = get_binary(input);
    part_1(binary);
}

fn part_1(binary: String) {
    let mut version_count: i64 = 0;
    generic_handle_packet(binary, &mut version_count);
    println!("Part 1: {}", version_count);
}

fn generic_handle_packet(input: String, mut version_count: &mut i64) -> (String, i64) {
    println!("Called generic handle packet with: {}", input);
    let mut result =0;
    if input.len() < 3 { 
        return ("".to_string() , 0);
    }
    let mut packet_version = from_binary(input[0..3].to_string());
    *version_count += packet_version.parse::<i64>().unwrap();
    println!("version count: {}", version_count);
    let packet_type = from_binary(input[3..6].to_string());
    println!("packet type: {}", packet_type);
    let mut remaining = input[6..].to_string();
    
    if packet_type == "4" {
        let temp = handle_packet_4(remaining, &mut version_count);
        remaining = temp.0;
        result = temp.1;
    } else if remaining[0..1].to_string() == "0" {
        let temp = handle_packet_0(remaining, &mut version_count, packet_type);
        remaining = temp.0;
        result = temp.1;
    } else {
        let temp = handle_packet(remaining, &mut version_count, packet_type);
        remaining = temp.0;
        result = temp.1;
    }
    println!("Remaining: {}", remaining);
    println!("Result: {}", result);
    return (remaining, result)
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

fn handle_packet_4(input: String, mut version_count: &mut i64) -> (String, i64) {
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
    let result = i64::from_str_radix(&literal_binary, 2).unwrap();
    if inputs.join("").chars().collect::<Vec<char>>().len() > 3 {
        let remaining = inputs.join("").to_string();
        return (remaining, result)
    }
    ("".to_string(), result)
}

fn handle_packet(input: String, mut version_count: &mut i64, type_id: String) -> (String, i64) {
    println!("found an operator packet which specifies a number of sub packets: {}", input);
    let mut remaining = input[1..].to_string();
    if remaining.chars().collect::<Vec<char>>().len() < 11 {
        return ("".to_string(), 0)
    }
    let packets = remaining[0..11].to_string();
    let num_packets = i64::from_str_radix(&packets, 2).unwrap();
    println!("num packets: {}", num_packets);
    remaining = remaining[11..].to_string();
    let mut count_of_packets = 1;
    let mut results = vec![];
    while count_of_packets <= num_packets {
        println!("checking packet {} of {}", count_of_packets, num_packets);
        println!("remaining: {}", remaining);
        let temp = generic_handle_packet(remaining, &mut version_count);
        remaining = temp.0;
        results.push(temp.1);
        count_of_packets += 1;
    }
    let mut result: i64 = 0;
    if type_id == "0" {
        result = results.iter().sum::<i64>();
    } else if type_id == "1" {
        result = results.iter().product::<i64>();
    } else if type_id == "2" {
        result = *results.iter().min().unwrap();
    } else if type_id == "3" {
        result = *results.iter().max().unwrap();
    } else if type_id == "5" {
        if results[0] > results[1] {
            result = 1;
        } else {
            result = 0;
        }
    } else if type_id == "6" {
        if results[0] < results[1] {
            result = 1;
        } else {
            result = 0;
        }
    } else if type_id == "7" {
        if results[0] == results[1] {
            result = 1;
        } else {
            result = 0;
        }
    } else {
        println!("unknown type id: {}", type_id);
    }
    (remaining, result)
}

fn handle_packet_0(input: String, mut version_count: &mut i64, type_id: String) -> (String, i64) {
    println!("found an set length operator packet: {}", input);
    let length_id = input[0..1].to_string();
    let mut remaining = input[1..].to_string();
    if remaining.chars().collect::<Vec<char>>().len() < 15 {
        return ("".to_string(), 0)
    }
    let bits = remaining[0..15].to_string();
    let size = i64::from_str_radix(&bits, 2).unwrap();
    println!("size: {}", size);
    let end: usize = (15 + size).try_into().unwrap();
    let mut remains = remaining[15..end].to_string();
    let to_return = remaining[end..].to_string();
    println!("to return: {}", to_return);
    let mut results = vec![];
    while remains.chars().collect::<Vec<char>>().len() > 3 {
        let temp = generic_handle_packet(remains, &mut version_count);
        remains = temp.0;
        results.push(temp.1);
    }
    let mut result: i64 = 0;
    if type_id == "0" {
        result = results.iter().sum::<i64>();
    } else if type_id == "1" {
        result = results.iter().product::<i64>();
    } else if type_id == "2" {
        result = *results.iter().min().unwrap();
    } else if type_id == "3" {
        result = *results.iter().max().unwrap();
    } else if type_id == "5" {
        if results[0] > results[1] {
            result = 1;
        } else {
            result = 0;
        }
    } else if type_id == "6" {
        if results[0] < results[1] {
            result = 1;
        } else {
            result = 0;
        }
    } else if type_id == "7" {
        if results[0] == results[1] {
            result = 1;
        } else {
            result = 0;
        }
    } else {
        println!("unknown type id: {}", type_id);
    }
    (to_return, result)
}


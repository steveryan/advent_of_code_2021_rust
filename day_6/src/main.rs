fn main() {
    let initial = vec![3,5,3,1,4,4,5,5,2,1,4,3,5,1,3,5,3,2,4,3,5,3,1,1,2,1,4,5,3,1,4,5,4,3,3,4,3,1,1,2,2,4,1,1,4,3,4,4,2,4,3,1,5,1,2,3,2,4,4,1,1,1,3,3,5,1,4,5,5,2,5,3,3,1,1,2,3,3,3,1,4,1,5,1,5,3,3,1,5,3,4,3,1,4,1,1,1,2,1,2,3,2,2,4,3,5,5,4,5,3,1,4,4,2,4,4,5,1,5,3,3,5,5,4,4,1,3,2,3,1,2,4,5,3,3,5,4,1,1,5,2,5,1,5,5,4,1,1,1,1,5,3,3,4,4,2,2,1,5,1,1,1,4,4,2,2,2,2,2,5,5,2,4,4,4,1,2,5,4,5,2,5,4,3,1,1,5,4,5,3,2,3,4,1,4,1,1,3,5,1,2,5,1,1,1,5,1,1,4,2,3,4,1,3,3,2,3,1,1,4,4,3,2,1,2,1,4,2,5,4,2,5,3,2,3,3,4,1,3,5,5,1,3,4,5,1,1,3,1,2,1,1,1,1,5,1,1,2,1,4,5,2,1,5,4,2,2,5,5,1,5,1,2,1,5,2,4,3,2,3,1,1,1,2,3,1,4,3,1,2,3,2,1,3,3,2,1,2,5,2];
    let mut fish_hash: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();
    for i in initial {
        let counter = fish_hash.entry(i).or_insert(0);
        *counter += 1;
    }
    for _ in 1..257 {
        let count_8 = fish_hash.get(&8).unwrap_or(&0).clone();
        let count_7 = fish_hash.get(&7).unwrap_or(&0).clone();
        let count_6 = fish_hash.get(&6).unwrap_or(&0).clone();
        let count_5 = fish_hash.get(&5).unwrap_or(&0).clone();
        let count_4 = fish_hash.get(&4).unwrap_or(&0).clone();
        let count_3 = fish_hash.get(&3).unwrap_or(&0).clone();
        let count_2 = fish_hash.get(&2).unwrap_or(&0).clone();
        let count_1 = fish_hash.get(&1).unwrap_or(&0).clone();
        let count_0 = fish_hash.get(&0).unwrap_or(&0).clone();
        fish_hash.insert(7, count_8);
        fish_hash.insert(6, count_7 + count_0);
        fish_hash.insert(5, count_6);
        fish_hash.insert(4, count_5);
        fish_hash.insert(3, count_4);
        fish_hash.insert(2, count_3);
        fish_hash.insert(1, count_2);
        fish_hash.insert(0, count_1);
        fish_hash.insert(8, count_0);
    }
    println!("{}", fish_hash.values().sum::<i64>());
}

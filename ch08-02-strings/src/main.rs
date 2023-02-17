#![allow(unused)]

use std::collections::HashMap;
fn main() {
    let s = String::from("नमस्ते");
    for c in s.chars() {
        print!("{},", c);
    }
    println!();
    for c in s.bytes() {
        print!("{},", c);
    }
    println!();
    println!("{}", s);

    let s = String::from("的发生发射点发撒打发");
    for c in s.chars() {
        print!("{},", c);
    }
    println!();
    for c in s.bytes() {
        print!("{},", c);
    }
    println!();
    println!("{}", s);

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let nums = vec![9, 9, 9, 1, 2, 3, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7, 8, 9, 9,7,5];
    println!("{}", average(&nums));
    println!("{}", median(&nums));
    println!("{:?}", mode(&nums));
}

fn average(nums: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for num in nums.iter() {
        sum += num;
    }

    sum as f64 / nums.len() as f64
}

fn median(nums: &Vec<i32>) -> f64 {
    let mut vec = nums.clone();
    vec.sort();
    let len = vec.len();
    println!("{}", len);
    if len % 2 == 0 {
        return (vec[len / 2] + vec[len / 2 - 1]) as f64 / 2.0;
    } else {
        return vec[len / 2 + 1].into();
    }
}

fn mode(nums: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut map = HashMap::new();

    //先用map统计所有数的个数
    for value in nums {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    //取出个数最多的数
    for (num, count) in map {
        // println!("update:({},{})", *num, count);
        if max_count < count {
            result = Vec::new();
            result.push(*num);
            max_count = count;
        } else if max_count == count {
            result.push(*num);
        }
    }

    result
}

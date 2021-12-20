use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let input = get_numbers();
    p1(&input);
    p2(&input);
}

fn get_numbers() -> Vec<i32> {
    let path = Path::new("D:\\code\\aoc2021\\aoc01\\src\\input.txt");
    let mut file = File::open(path).expect("open failed!");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("read failed!");
    let mut result = vec![];
    for number in s.split("\n") {
        let number = number.trim().parse::<i32>().unwrap();
        result.push(number);
    }
    result
}

fn p1(input: &Vec<i32>) {
    let mut tmp = 0;
    let mut counter = 0;
    for number in input {
        if *number > tmp {
            counter += 1;
        }
        tmp = *number;
    }
    println!("{}", counter - 1);
}

fn p2(input: &Vec<i32>) {
    let mut tmp = 0;
    let mut counter = 0;
    for i in 0..input.len() - 2 {
        let res = input[i] + input[i+1] + input[i+2];
        if res > tmp {
            counter += 1;
        }
        tmp = res;
    }
    println!("{}", counter - 1);
}
use core::panic;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file_path = File::open("day1.test").expect("unable to open the file");

    let (mut left, mut right) = file_reader(file_path).expect("Some error with the file");

    left.sort();
    right.sort();


    let similarity_score: i32 = left
        .iter()
        .map(|&num| {
            let frequency = right.iter().filter(|&&x| x == num).count() as i32;
            num * frequency
        })
        .sum();
    
    println!("{:?}", similarity_score);
}

fn file_reader(file: File) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let reader = BufReader::new(file);

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line?; // Unwrap the result
        let nums: Vec<&str> = line.split_whitespace().collect();
        if nums.len() == 2 {
            let left_num = nums[0].parse::<i32>().unwrap_or_else(|_| {
                panic!("Invalid number in left column: {}", nums[0]);
            });
            let right_num = nums[1].parse::<i32>().unwrap_or_else(|_| {
                panic!("Invalid number in right column: {}", nums[1]);
            });
            left.push(left_num);
            right.push(right_num);
        } else {
            panic!("Each line must contain exactly two numbers: {}", line);
        }
    }

    // Ensure we return the correct type at the end
    Ok((left, right))
}

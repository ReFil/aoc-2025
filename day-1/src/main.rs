use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-1/src/input.txt")
        .expect("Should have been able to read the file");

    let mut input_vec: Vec<i32> = Vec::new();

    for line in contents.lines() {
        if line.trim().starts_with('R') {
            input_vec.push(line.trim().trim_matches('R').parse().expect("Error parsing"));
        }
        else if line.trim().starts_with('L') {
            input_vec.push(-(line.trim().trim_matches('L').parse::<i32>().expect("Error parsing")));
        }
        else {
            println!("Error parsing file");
        }
    }


    let mut passwd = 0;
    let mut dial = 50;

    for element in input_vec{
        dial = dial + element;
        while dial < 0 {
            dial = 100 + dial;
        }
        while dial > 0 {
            dial = dial - 100;
        }
        if dial == 0 {
            passwd+=1;
        }
    }
    println!("{}",passwd);
}

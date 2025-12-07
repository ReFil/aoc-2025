use std::fs;


fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-6/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split("\n");
    let parts_collection = parts.collect::<Vec<&str>>();
    // Split into two ranges
    let part1_split = parts_collection[0].split_whitespace();
    let part2_split = parts_collection[1].split_whitespace();
    let part3_split = parts_collection[2].split_whitespace();
    let part4_split = parts_collection[3].split_whitespace();
    let part5_split = parts_collection[4].split_whitespace();

    let part1_collection = part1_split.collect::<Vec<&str>>();
    let part2_collection = part2_split.collect::<Vec<&str>>();
    let part3_collection = part3_split.collect::<Vec<&str>>();
    let part4_collection = part4_split.collect::<Vec<&str>>();
    let part5_collection = part5_split.collect::<Vec<&str>>();


    if part1_collection.len() != part2_collection.len() &&
    part1_collection.len() != part3_collection.len() &&
    part1_collection.len() != part4_collection.len() {
        println!("Warning, unequal lengths");
    }

    let mut sum:i128 = 0;

    for i in 0..part1_collection.len() {
        if part5_collection[i] == "+" {
            sum += part1_collection[i].parse::<i128>().unwrap() + part2_collection[i].parse::<i128>().unwrap() + part3_collection[i].parse::<i128>().unwrap() + part4_collection[i].parse::<i128>().unwrap();
        }
        else {
            sum += part1_collection[i].parse::<i128>().unwrap() * part2_collection[i].parse::<i128>().unwrap() * part3_collection[i].parse::<i128>().unwrap() * part4_collection[i].parse::<i128>().unwrap();
        }

    }
    println!("{}", sum);
}

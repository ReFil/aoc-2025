use std::fs;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-7/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split("\n");
    let parts_collection = parts.collect::<Vec<&str>>();

    let points_vec: Vec<Point> = 


}
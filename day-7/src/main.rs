use std::fs;


fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-7/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split("\n");
    let parts_collection = parts.collect::<Vec<&str>>();

    let mut beams_vec: Vec<i128> = Vec::new();

    for i in 0..parts_collection[0].len() {
        if parts_collection[0][i..i+1] == *"S" {
            beams_vec.push(1);
        }
        else {beams_vec.push(0);}
    }
    dbg!(&beams_vec);
    let mut split_count = 0;

    for j in 1..parts_collection.len() {
        for i in 0..parts_collection[j].len() {
            if parts_collection[j][i..i+1] == *"^" && beams_vec[i] > 0 {
                if i > 0 {
                   beams_vec[i-1] += beams_vec[i];
                }
                if i < parts_collection[j].len() {
                    beams_vec[i+1] += beams_vec[i];
                }
                beams_vec[i] = 0;
                split_count+=1;
                println!("Split occurred, line {}, position {}, count {}", j, i, split_count);
            }
            //dbg!(&beams_vec);
        }
    }

    println!("Number of end beams {}", split_count);

    let sum: i128 = beams_vec.iter().sum();
    
    println!("The sum of the elements is {}.", sum); 

}
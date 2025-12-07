use std::fs;


fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-7/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split("\n");
    let parts_collection = parts.collect::<Vec<&str>>();

    let mut beams_vec: Vec<i32> = Vec::new();

    for i in 0..parts_collection[0].len() {
        if parts_collection[0][i..i+1] == *"S" {
            beams_vec.push(i.try_into().unwrap());
        }
    }
    dbg!(&beams_vec);
    let mut split_count = 0;

    for j in 1..parts_collection.len() {
        let old_beams_vec = beams_vec;
        beams_vec = Vec::new();
        for i in 0..parts_collection[j].len() {
            if parts_collection[j][i..i+1] == *"^" && old_beams_vec.contains(&(<usize as TryInto<i32>>::try_into(i).unwrap())){
                if i > 0 {
                   beams_vec.push((i-1).try_into().unwrap());
                }
                if i < parts_collection[j].len() {
                    beams_vec.push((i+1).try_into().unwrap());
                }
                split_count+=1;
                println!("Split occurred, line {}, position {}, count {}", j, i, split_count);
            }
            if parts_collection[j][i..i+1] == *"." && old_beams_vec.contains(&(<usize as TryInto<i32>>::try_into(i).unwrap())) {
                beams_vec.push(i.try_into().unwrap());
            } 
            beams_vec.dedup();
            dbg!(&beams_vec);
        }
    }

    println!("{}",beams_vec.len())

}
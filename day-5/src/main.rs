use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-5/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split("\n\n");
    let parts_collection = parts.collect::<Vec<&str>>();
    // Split into two ranges
    let part1_split = parts_collection[0].split('\n');
    let part2_split = parts_collection[1].split('\n');

    let part1_collection = part1_split.collect::<Vec<&str>>();
    let part2_collection = part2_split.collect::<Vec<&str>>();

    let mut fresh_ids = 0;

    let mut id_vec: Vec<i128> = Vec::new();



    for element1 in part2_collection {
        let int_element: i128 = element1.parse().expect("NaN");
       /* if id_vec.contains(&int_element){
            fresh_ids+=1;
        }*/
        'id_iter: for element in &part1_collection {
            let split = element.split('-').collect::<Vec<&str>>();
            //dbg!(split);
            let start: i128 = split[0].parse().expect("NaN");
            let end: i128 = split[1].parse().expect("NaN");

            if int_element >= start && int_element <= end {
                fresh_ids+=1;
                break 'id_iter;
            }

/*
            for i in start..end+1 {
                id_vec.push(i);
            }*/
        }

    }
    println!("{}", fresh_ids);
}

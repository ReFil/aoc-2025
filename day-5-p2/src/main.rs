use std::fs;

#[derive(PartialEq)]
#[derive(Debug)]
enum RangeType {
    RangeStart,
    RangeEnd,
}

#[derive(Debug)]
struct RangeValue {
    value: i128,
    rtype: RangeType,
}

fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-5/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split("\n\n");
    let parts_collection = parts.collect::<Vec<&str>>();
    // Split into two ranges
    let part1_split = parts_collection[0].split('\n');

    let part1_collection = part1_split.collect::<Vec<&str>>();

    let mut range_vector: Vec<RangeValue> = Vec::new();


    for element in &part1_collection {
        let split = element.split('-').collect::<Vec<&str>>();
        //dbg!(split);
        let start: i128 = split[0].parse().expect("NaN");
        let end: i128 = split[1].parse().expect("NaN");

        let start_struct = RangeValue {value: start, rtype: RangeType::RangeStart};
        let end_struct = RangeValue {value: end, rtype: RangeType::RangeEnd};
        
        range_vector.push(start_struct);
        range_vector.push(end_struct);
    }

    range_vector.sort_by_key(|x| (x.value + {if x.rtype == RangeType::RangeStart {0 } else { 1 } }));

    let mut fresh_ids = 0;
    let mut tracking_open = 0;
    let mut start_val = 0;

    for i in 0..range_vector.len() {
        if range_vector[i].rtype == RangeType::RangeStart {
            tracking_open += 1;
            if tracking_open == 1 {
                start_val = range_vector[i].value;
            }
            
        }
        if range_vector[i].rtype == RangeType::RangeEnd {
            tracking_open -= 1;
            if tracking_open == 0 {
                fresh_ids += range_vector[i].value + 1 - start_val;
            }
        }

    }

    println!("{}", fresh_ids);

    //dbg!(range_vector);
}

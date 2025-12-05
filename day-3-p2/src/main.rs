
use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-3/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split('\n');
    let collection = parts.collect::<Vec<&str>>();

    //dbg!(collection);

    let mut sum: i128 = 0;

    for element in collection {

        let char_array = element.as_bytes();
        let len = element.len();
        let mut joltagestr = String::new();
        let mut last_index = 0;
        for i in 0..12 {
            let mut largest = 0;
            for j in last_index..len-(11-i) {
                let dig = char_array[j];
                let dig_value = (dig-48).into();
                //println!("{}", dig1-48);
                if dig_value > largest{
                    largest = dig_value;
                    last_index = j+1;
                }
                
            }
            println!("{}", last_index);
            joltagestr.push_str(&format!("{}", largest));
        }
        println!("{}", joltagestr);
        sum += joltagestr.parse::<i128>().expect("huh");
        
    }
    println!("{}", sum);
}

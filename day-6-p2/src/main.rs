use std::fs;


fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-6/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split("\n");
    let parts_collection = parts.collect::<Vec<&str>>();
    // Split into two ranges

    let mut sum:i128 = 0;
   // dbg!(parts_collection);

    let mut index_vec: Vec<i32> = Vec::new();
    index_vec.push(0);

    for i in 0..parts_collection[0].len() {
        if parts_collection[0].to_string()[i..i+1]== *" " &&
        parts_collection[1].to_string()[i..i+1] == *" " &&
        parts_collection[2].to_string()[i..i+1] == *" " &&
        parts_collection[3].to_string()[i..i+1] == *" " &&
        parts_collection[4].to_string()[i..i+1] == *" " {
            index_vec.push(i.try_into().unwrap());
        }


    }
    index_vec.push((parts_collection[0].len()).try_into().unwrap());
    dbg!(&index_vec);
    for i in 2..index_vec.len() {
        let mut vec: Vec<String> = Vec::new();
        for j in 1..(index_vec[i]-index_vec[i-1]){
            vec.push(String::new());
            for k in 0..4 {
                vec[(j-1) as usize].push_str(&parts_collection[k].to_string()[(index_vec[i-1]+j) as usize..(index_vec[i-1]+j+1) as usize])
            }
        }
        let op = &parts_collection[4].to_string()[(index_vec[i-1]+1) as usize..(index_vec[i-1]+2) as usize];
        dbg!(&vec);
        let mut temp:i128 = vec[0].trim_matches(' ').parse::<i128>().unwrap();
        for i in 1..vec.len() {
            if op == "+" {
                temp += vec[i].trim_matches(' ').parse::<i128>().unwrap();
            }
            else {
                temp *= vec[i].trim_matches(' ').parse::<i128>().unwrap();
            }
        }
        dbg!(&temp);
        sum += temp;
    }
    println!("{}", sum);
}

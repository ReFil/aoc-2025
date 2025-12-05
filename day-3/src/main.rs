use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-3/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split('\n');
    let collection = parts.collect::<Vec<&str>>();

    //dbg!(collection);

    let mut sum = 0;

    for element in collection {
        let mut largest = 0;

        let char_array = element.as_bytes();
        for i in 0..element.len().try_into().unwrap() {
            let dig1 = char_array[i];
            //println!("{}", dig1-48);
            for j in i+1..element.len().try_into().unwrap(){
                let dig2 = char_array[j];
                let digcomb: i32 = ((dig2-48) + ((dig1-48) * 10)).into();
                //println!("{}", digcomb);
                if digcomb > largest {
                    largest = digcomb;
                }
            }
        }
        println!("{}", largest);
        sum += largest;
        
    }
    println!("{}", sum);
}

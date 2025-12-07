use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-4/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split('\n');
    let collection = parts.collect::<Vec<&str>>();
   // dbg!(collection);
   let mut accessible_rolls = 0;
    for i in 0..collection.len() {
        let element = collection[i].as_bytes();
        for j in 0..element.len(){
            if element[j] == 64 {
                let mut surrounding_rolls = 0;
                if i != 0 {
                    let prev_row = collection[i-1].as_bytes();
                    if j != 0 {
                        if prev_row[j-1] == 64{
                            surrounding_rolls +=1;
                        }
                    }
                    if j+1 != element.len() {
                        if prev_row[j+1] == 64{
                            surrounding_rolls +=1;
                        }
                    }
                    if prev_row[j] == 64{
                        surrounding_rolls +=1;
                    }
                        
                }
                if i+1 != collection.len() {
                    let next_row = collection[i+1].as_bytes();
                    if j != 0 {
                        if next_row[j-1] == 64{
                            surrounding_rolls +=1;
                        }
                    }
                    if j+1 != element.len() {
                        if next_row[j+1] == 64{
                            surrounding_rolls +=1;
                        }
                    }
                    if next_row[j] == 64{
                        surrounding_rolls +=1;
                    }
                }
                if j != 0 {
                    if element[j-1] == 64{
                        surrounding_rolls +=1;
                    }
                }
                if j+1 != element.len() {
                    if element[j+1] == 64{
                        surrounding_rolls +=1;
                    }
                }

                if surrounding_rolls < 4 {
                    accessible_rolls+= 1;
                }
            }
        }
    }
    println!("{}",accessible_rolls);
}

use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-4/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split('\n');
    let collection = parts.collect::<Vec<&str>>();

    let mut vec: Vec<Vec<i32>> = Vec::new();
    for i in 0..collection.len() {
        vec.push(Vec::new());
        let element = collection[i].as_bytes();
            for j in 0..element.len(){
                vec[i].push(element[j].into());
            }
    }
    //dbg!(vec);
   
   let mut accessible_rolls = 0;
   let mut prev_accessible_rolls = -1;
   while accessible_rolls > prev_accessible_rolls {
        prev_accessible_rolls = accessible_rolls;
        for i in 0..vec.len() {
            for j in 0..vec[i].len(){
                if vec[i][j] == 64 {
                    let mut surrounding_rolls = 0;
                    if i != 0 {
                        if j != 0 {
                            if vec[i-1][j-1] == 64{
                                surrounding_rolls +=1;
                            }
                        }
                        if j+1 != vec[i].len() {
                            if vec[i-1][j+1] == 64{
                                surrounding_rolls +=1;
                            }
                        }
                        if vec[i-1][j] == 64{
                            surrounding_rolls +=1;
                        }
                            
                    }
                    if i+1 != collection.len() {
                        if j != 0 {
                            if vec[i+1][j-1] == 64{
                                surrounding_rolls +=1;
                            }
                        }
                        if j+1 != vec[i].len() {
                            if vec[i+1][j+1] == 64{
                                surrounding_rolls +=1;
                            }
                        }
                        if vec[i+1][j] == 64{
                            surrounding_rolls +=1;
                        }
                    }
                    if j != 0 {
                        if vec[i][j-1] == 64{
                            surrounding_rolls +=1;
                        }
                    }
                    if j+1 != vec[i].len() {
                        if vec[i][j+1] == 64{
                            surrounding_rolls +=1;
                        }
                    }

                    if surrounding_rolls < 4 {
                        accessible_rolls+= 1;
                        vec[i][j] = 120;
                    }
                }
            }
        }
    }
    println!("{}", accessible_rolls);
}
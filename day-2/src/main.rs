use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/willow/aoc-2025/day-2/src/input.txt")
        .expect("Should have been able to read the file");

    let parts = contents.split(',');
    let collection = parts.collect::<Vec<&str>>();
    //dbg!(collection);

    let mut sum: i128 = 0;

    for element in collection {
        let startend = element.split('-').collect::<Vec<_>>();
        let startint = startend[0].parse::<i128>().expect("Error parsing");
        let endint = startend[1].parse::<i128>().expect("Error parsing");
        for i in startint..endint+1{
            let len = i.to_string().len() ;
           // println!("{}, {}, {}", i, len, get_divisor(len.try_into().unwrap()) );

            if len % 2 == 0 {
                if i % get_divisor(len.try_into().unwrap()) == 0 {
                    sum += i;
                }
            }
            

        }
    }

    println!("{}", sum);

}

fn get_divisor (len:i32) -> i128 {
    let mut divisorstr =  String::new();
    divisorstr.push_str("1");
    if len > 2 {
        for i in 0 .. (len/2) - 1 {
            divisorstr.push_str(&format!("{}", "0"));
        }
    }
    divisorstr.push_str(&format!("{}", "1"));
    divisorstr.parse::<i128>().expect("Error parsing")
}
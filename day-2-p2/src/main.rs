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

            // check for all sets of repeats
            'scansplit: for j in 2..len+1 {
                if (i % get_divisor(len.try_into().unwrap(), j.try_into().unwrap()) == 0) && (len as i32 % j as i32 == 0){
                    println!("{}, {}", i, get_divisor(len.try_into().unwrap(), j.try_into().unwrap()));
                    sum += i;
                    break 'scansplit;
                }
            }     
        }
    }

    println!("{}", sum);

}

fn get_divisor (len:i32, splits:i32) -> i128 {
    let mut divisorstr =  String::new();
    divisorstr.push_str("1");

    for _i in 0..splits-1 {
        if len > 2 {
            for _i in 0 .. (len/splits) - 1 {
                divisorstr.push_str(&format!("{}", "0"));
            }
        }
        divisorstr.push_str(&format!("{}", "1"));
    }
    divisorstr.parse::<i128>().expect("Error parsing")
}
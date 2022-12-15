
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// Day 1 -- Calorie Counting


// Given a file with numbers in the following format, return the sum of the largest list
/*
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
*/

pub fn find_max_sum(filename: &str) -> i32 {

    // open text file in filename
    // let contents: String = fs::read_to_string(filename).expect("failure opening file");
    
    let file = File::open(filename).expect("failure opening file");

    let reader = BufReader::new(file);

    let mut run_sum : i32 = 0;
    let mut max: i32 = 0;
    for line in reader.lines() {
        let string_value : String = line.expect("failure parsing file into lines");
        let string_value = string_value.trim();  
        
        if string_value == "" {
            if run_sum > max {
                max = run_sum;
            }
            run_sum = 0;
            continue;
        }   
        let value : i32 = string_value.parse().expect("non-number found");
        run_sum += value;
    }   


    // println!("{}", contents);

    return max;

}



// #[test]
// fn test_max_sum() {

//     let expected = 24000;
 
//     assert_eq!(expected, find_max_sum("src/day1.txt"));


// }



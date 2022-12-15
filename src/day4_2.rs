use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn overlapping_ranges2(filename : &str) -> i32 {

    // format:
    /*
    54-59,17-62
    20-93,57-92
    6-54,7-54
    3-99,59-98
    5-8,5-8
    89-94,32-89
    1-91,1-90
    */

    let file = File::open(filename).expect("failure opening file");

    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let line = line.expect("failed to parse into lines");

        let line_vec: Vec<&str> = line.split(",").collect();

        let meta_vec: Vec<Vec<&str>> = line_vec.iter().map(|x| x.split("-").collect()).collect();
        // [[num1, num2], [num3, num4]]
        let start_a : i32 = meta_vec[0][0].parse().expect("failed to convert into num");
        let end_a : i32 = meta_vec[0][1].parse().expect("failed to convert into num");
        let start_b : i32 = meta_vec[1][0].parse().expect("failed to convert into num");
        let end_b : i32 = meta_vec[1][1].parse().expect("failed to convert into num");

        // 5-7,7-9

        if (start_a <= start_b && end_a >= start_b) || (start_b <= start_a && end_b >= start_a) {
            count += 1;
        }
    }

    return count;

}


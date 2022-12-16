

// get the total calories among the top three elves

use std::{fs::File, io::BufReader};
use std::io::BufRead;

pub fn total_top_three(filename: &str) -> i32 {

     // open text file in filename
    // let contents: String = fs::read_to_string(filename).expect("failure opening file");
    
    let file = File::open(filename).expect("failure opening file");

    let reader = BufReader::new(file);

    let mut run_sum : i32 = 0;

    let mut max_array: [i32; 3] = [0, 0, 0];

    for line in reader.lines() {
        let string_value : String = line.expect("failure parsing file into lines");
        let string_value = string_value.trim();  

        if string_value == "" {
            max_array = insert_elem(max_array, run_sum);
            run_sum = 0;
            continue;
        }   
        let value : i32 = string_value.parse().expect("non-number found");
        run_sum += value;
    }   


    // println!("{}", contents);

    return max_array.iter().sum();


}

fn insert_elem(arr: [i32; 3], elem : i32) -> [i32; 3] {

    let mut ret_array : [i32; 3] = [arr[0], arr[1], arr[2]];

    let mut i = 0;

    while i < 3 {

        if elem > arr[i] {
            ret_array[i] = elem;
            let mut j = i+1;
            while j < 3 {
                ret_array[j] = arr[j-1];
                j += 1;
            }
            break;
        }
        i += 1;
    }

    return ret_array;
}

#[test]
fn test_insert_elem() {

    let arr = [7, 5, 2];

    let expected = [8, 7, 5];
    let expected2 = [7, 6, 5];
    let expected3 = [7, 5, 3];
    let expected4 = [7, 5, 2];

    assert_eq!(expected, insert_elem(arr, 8));
    assert_eq!(expected2, insert_elem(arr, 6));
    assert_eq!(expected3, insert_elem(arr, 3));
    assert_eq!(expected4, insert_elem(arr, 1));


}
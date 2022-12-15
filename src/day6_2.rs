use std::fs::File;
use std::io::Read;


// how many characters do we need to read before 4 in a row are all different?

pub fn day6main2(filename : &str) -> usize {
    
    let mut f = File::open(filename).expect("failed to open file");

    let mut s = String::new();

    f.read_to_string(&mut s).expect("failed to read into string");

    return detect_unique_sequence(&s);
    
}

fn detect_unique_sequence(s: &String) -> usize {


    for (i,c) in s.chars().enumerate().skip(14) {

        let arr : Vec<char> = (0..14).map(|x| s.chars().nth(i-x).unwrap()).collect();
        println!("Array {:?}", arr);
        
        if no_repeats(arr) {
            return i + 1;
        }
    }

    return 0;

}

fn no_repeats(arr: Vec<char>) -> bool {
    return !(1..14).any(|i| arr[i..].contains(&arr[i - 1]));
}
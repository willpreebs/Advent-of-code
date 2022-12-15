use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::num;
// letters are at line index 1, 5, 9, ...
/*

    [B]             [B] [S]        
    [M]             [P] [L] [B] [J]
    [D]     [R]     [V] [D] [Q] [D]
    [T] [R] [Z]     [H] [H] [G] [C]
    [P] [W] [J] [B] [J] [F] [J] [S]
[N] [S] [Z] [V] [M] [N] [Z] [F] [M]
[W] [Z] [H] [D] [H] [G] [Q] [S] [W]
[B] [L] [Q] [W] [S] [L] [J] [W] [Z]
 1   2   3   4   5   6   7   8   9 

move 3 from 5 to 2
move 5 from 3 to 1
move 4 from 4 to 9
move 6 from 1 to 4
move 6 from 8 to 7
move 5 from 2 to 7
move 1 from 5 to 4
move 11 from 9 to 7
...

*/


// Part 2: Move multiple stacks at once and preserve order.

pub fn find_tops_of_stacks2(filename : &str) -> String {

    let mut stacks : [Vec<String>; 9] = Default::default();

    let f = File::open(filename).expect("failed to open file");

    let reader = BufReader::new(f);

    // stack initialization
    for line in reader.lines() {

        let line = line.expect("failed");

        if line.starts_with(" 1") {
            break;
        }
        let mut i : usize = 1;
    
        while i <= 33 {
            let a : char = line.chars().nth(i).unwrap();
            if a != ' ' {
                stacks[(i - 1) / 4].push(a.to_string());
                //println!("found char: {} in stack: {}", a, (i - 1) / 4);
            }
            i += 4;
        }
    }

    let mut i : usize = 0;

    let mut reversed_stack: [Vec<String>; 9] = Default::default();

    while i < 9 {
        for e in stacks[i].iter().rev() {
            reversed_stack[i].push(String::from(e));
        }
        i += 1;
    } 
    let f = File::open(filename).expect("failed to open file");
    let reader = BufReader::new(f);
    // move instructions loop
    for line in reader.lines() {

        let line = line.expect("failed");

        // go to beginning of move instructions
        if !line.starts_with("m") {
            println!("{}", &line);
            continue;
        }
        
        let parsed_move: Vec<&str> = line.split(" ").collect();
        let mut num_to_move : usize = parsed_move[1].parse().unwrap();
        let src_stack_index: usize = parsed_move[3].parse().unwrap();
        let dst_stack_index: usize = parsed_move[5].parse().unwrap();

        let mut moved_crates : Vec<String> = Default::default();

        while num_to_move > 0 {
            moved_crates.push(reversed_stack[src_stack_index-1].pop().unwrap());
            num_to_move -= 1;
        }
        moved_crates.reverse();
        reversed_stack[dst_stack_index-1].append(&mut moved_crates);

    }

    let mut k = 0;
    let mut return_string : String = String::from("");

    while k < 9 {
        let s = reversed_stack[k].pop().unwrap();
        return_string.push_str(s.as_str());
        k += 1;
    }

    return return_string;


}
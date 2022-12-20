use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;

// finding how many "trees" are visible from the outside

// first idea: take row by row (starting with second row because all trees in first row are automatically visible)
// for each row, if a tree is the tallest from the start of the row, it is visible. Run in each direction
// then, consider each column in the same manner
// O(n)???

pub fn visible_trees(filename : &str) -> i32 {
    
    let mut f = File::open(filename).expect("failed to open file");

    let mut text = String::new();

    let result = f.read_to_string(&mut text);
    //println!("read: {} characters", result.unwrap());

    const ROWS: usize = 99;
    const COLS: usize = 99;

    let mut trees = [[false; ROWS]; COLS];

    // sets first and last rows as true
    trees[0] = [true; COLS];
    trees[ROWS-1] = [true; COLS];

    for i in 0..ROWS {
        
        // sets first and last columns as true
        trees[i][0] = true;
        trees[i][COLS-1] = true;

        // forward loop
        let mut max = text
        .lines().nth(i).unwrap().trim()
        .chars().nth(0).unwrap()
        .to_digit(10).unwrap();

        for j in 0..COLS {

            if i == 0 {
                trees[i][j] = true;
                continue;
            }
            
            let val = text.lines().nth(i).unwrap().trim().chars().nth(j).unwrap().to_digit(10).unwrap();
            if val > max {
                max = val;
                trees[i][j] = true;
            }
        }

        // backward loop
        max = text.lines().nth(i).unwrap().trim().chars().last().unwrap().to_digit(10).unwrap();
        for j in (0..COLS).rev() {
            let val = text.lines().nth(i).unwrap().trim().chars().nth(j).unwrap().to_digit(10).unwrap();
            if val > max {
                // this index is visible
                trees[i][j] = true;
                max = val;
            }
        }
    }

    let mut file = File::open(filename).unwrap();
    let mut text = String::new();

    file.read_to_string(&mut text).unwrap();
    

    for j in (0..COLS) {

        let mut max = text
        .lines().nth(0).unwrap()
        .chars().nth(j).unwrap()
        .to_digit(10).unwrap();

        trees[0][j] = true;

        for i in (0..ROWS) {

            let val = text
            .lines().nth(i).unwrap().trim()
            .chars().nth(j).unwrap()
            .to_digit(10).unwrap();
            if val > max {
                max = val;
                trees[i][j] = true;
            }
        }

        let mut max = text
        .lines().last().unwrap()
        .chars().nth(j).unwrap()
        .to_digit(10).unwrap();

        trees[ROWS-1][j] = true;

        for i in (0..ROWS).rev() {
            let val = text
            .lines().nth(i).unwrap().trim()
            .chars().nth(j).unwrap()
            .to_digit(10).unwrap();
            if val > max {
                max = val;
                trees[i][j] = true;
            }
        }
    }

    let mut count = 0;

    for i in 0..99 {
        for j in 0..99 {
            if trees[i][j] {
                count += 1;
            }
        }
    }

    return count;
}
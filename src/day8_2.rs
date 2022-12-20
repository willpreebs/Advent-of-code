use std::fs::File;
use std::io::Read;



pub fn max_scenic_score(filename: &str) -> u32 {

    let mut f = File::open(filename).unwrap();

    let mut text = String::new();

    let result = f.read_to_string(&mut text);

    const ROWS : usize = 99;
    const COLS : usize = 99;

    let mut max: u32 = 0;
    for i in 0..ROWS {
        for j in 0..COLS {
            let val = get_val_at(&text, i, j);
            let mut left = 1;
            let mut right = 1;
            let mut up = 1;
            let mut down = 1;
            // count how many trees can be seen in each direction

            //left: i, j -> i, j-1 ...
            let mut l = j as i32 - 1;
            while l > 0 && get_val_at(&text, i, l as usize) < val {
                left += 1;
                l -= 1;
            }
            let mut r = j + 1;
            while r < COLS-1 && get_val_at(&text, i, r) < val {
                right += 1;
                r += 1;
            }
            let mut u = i as i32 - 1;
            while u > 0 && get_val_at(&text, u as usize, j) < val {
                up += 1;
                u -= 1;
            }
            let mut d = i + 1;
            while d < ROWS-1 && get_val_at(&text, d, j) < val {
                down += 1;
                d += 1;
            }
            let mut scenic = left * right * up * down;
            if i == 0 || i == ROWS-1 || j == 0 || j == COLS-1 {
                scenic = 0;
            }
            if scenic > max {
                max = scenic;
            }
        }
    }
    return max;
}

fn get_val_at(text: &String, i: usize, j: usize) -> u32 {
    return text.lines().nth(i).unwrap().chars().nth(j).unwrap().to_digit(10).unwrap();
}
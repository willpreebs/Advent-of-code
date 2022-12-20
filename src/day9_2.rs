use std::{fs::File, io::Read, collections::HashSet};

pub fn num_tail_positions(filename: &str) -> usize {

    let mut f = File::open(filename).expect("file not found");
    let mut text = String::new();

    f.read_to_string(&mut text);

    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert((0, 0));

    let mut rope_arr: [(i32, i32); 10] = [(0, 0); 10];
    // rope_arr[0] is head
    // rope_arr[8] is tail (which we track)

    for line in text.lines() {

        let line_vec: Vec<&str> = line.split(" ").collect();
        let direction = line_vec[0];
        let mut magnitude: i32 = line_vec[1].parse().unwrap();

        while magnitude > 0 {
            let (head_x, head_y) = rope_arr[0];
            match direction {
                "U" => rope_arr[0] = (head_x, head_y - 1),
                "D" => rope_arr[0] = (head_x, head_y + 1),
                "L" => rope_arr[0] = (head_x - 1, head_y),
                "R" => rope_arr[0] = (head_x + 1, head_y),
                _ => (),
            }
            magnitude -= 1;
            if is_touching(&rope_arr[0], &rope_arr[1]) {
                continue;
            }
            for i in 1..10 {
                if !is_touching(&rope_arr[i-1], &rope_arr[i]) {
                    rope_arr[i] = move_to_touch(&rope_arr[i-1], &rope_arr[i]);
                }
                else {
                    break;
                }
            }
            set.insert(rope_arr[9]);
        }
    }
    return set.len();
}


fn is_touching(head_coors: &(i32, i32), tail_coors: &(i32, i32)) -> bool {

    let (head_x, head_y) = head_coors;
    let (tail_x, tail_y) = tail_coors;

    return (head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1; 
}

fn move_to_touch(target_coors: &(i32, i32), cur_coors: &(i32, i32)) -> (i32, i32) {

    //If target and cur are in same row or column, then dest will stay in that row/column but one closer to target
    //else if target and cur are not in same row/column, then dest will be a diagonal move away from cur toward dest. 

    let mut dest_coors = (0, 0);

    let (target_x, target_y) = target_coors;
    let (cur_x, cur_y) = cur_coors;

    if target_x == cur_x || target_y == cur_y {
        return ((target_x + cur_x) / 2, (target_y + cur_y) / 2);
    }
    let diff_x = target_x - cur_x;
    let diff_y = target_y - cur_y;

    let mut offset_x = 0;
    let mut offset_y = 0;
    match diff_x > 0 {
        true => offset_x = 1,
        false => offset_x = -1,
    }
    match diff_y > 0 {
        true => offset_y = 1,
        false => offset_y = -1,
    }
    return (cur_x + offset_x, cur_y + offset_y);
}


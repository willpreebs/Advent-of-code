use std::{fs::File, io::Read, collections::HashSet};

pub fn num_tail_positions(filename: &str) -> usize {

    let mut f = File::open(filename).expect("file not found");
    let mut text = String::new();

    f.read_to_string(&mut text);

    // head and tail start as overlapped.
    // coors are (X, Y)
    // U -> -Y
    // D -> +Y
    // L -> -X
    // R -> +X
    let mut head_coors = (0, 0);
    let mut tail_coors = (0, 0);

    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert((0, 0));

    for line in text.lines() {

        // line looks like: D 2
        let line_vec: Vec<&str> = line.split(" ").collect();
        let direction = line_vec[0];
        let mut magnitude: i32 = line_vec[1].parse().unwrap();

        while magnitude > 0 {

            let (head_x, head_y) = head_coors;
            // move head
            match direction {
                "U" => head_coors = (head_x, head_y - 1),
                "D" => head_coors = (head_x, head_y + 1),
                "L" => head_coors = (head_x - 1, head_y),
                "R" => head_coors = (head_x + 1, head_y),
                _ => (),
            }
            //println!("moved head to: {:?}", head_coors);

            let (head_x, head_y) = head_coors;

            magnitude -= 1;
            
            if is_touching(&head_coors, &tail_coors) {
                continue;
            }

            // move tail
            match direction {
                "U" => tail_coors = (head_x, head_y + 1),
                "D" => tail_coors = (head_x, head_y - 1),
                "L" => tail_coors = (head_x + 1, head_y),
                "R" => tail_coors = (head_x - 1, head_y),
                _ => (),
            }  
            // mark tail position
            set.insert(tail_coors);
        }
    }

    return set.len();
}

fn is_touching(head_coors: &(i32, i32), tail_coors: &(i32, i32)) -> bool {

    let (head_x, head_y) = head_coors;
    let (tail_x, tail_y) = tail_coors;

    return (head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1; 
}

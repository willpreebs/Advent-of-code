use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

/*
$ cd /
$ ls
dir gqcclj
dir lmtpm
dir nhqwt
dir qcq
dir vwqwlqrt
$ cd gqcclj
$ ls
62425 dqp.gjm
174181 hrtw.qsd
273712 pflp.mdw
169404 zlthnlhf.mtn
180878 zprprf
$ cd ..
$ cd lmtpm
*/

pub fn directory_sum(filename: &str) -> u32 {

    let f = File::open(filename).expect("failed to open file");
    let reader = BufReader::new(f);

    let mut cur_path : Vec<String> = Vec::new();
    let mut path_hash : HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {

        let line = line.unwrap();
        
        let split = line.split(" ");

        // change directory handling
        if split.clone().nth(0).unwrap() == "$" && split.clone().nth(1).unwrap() == "cd" {
            let cd = split.clone().nth(2).unwrap();
            //println!("cd command found: {}\t Current path: {:?}", line.clone(), cur_path);
            if cd == ".." {
                cur_path.pop();
            }
            else {
                cur_path.push(String::from(cd));
            }
            continue;
        }

        // if ls command or a dir entry, we don't really care about this line, we can just move on
        if split.clone().nth(0).unwrap() == "dir" && split.clone().nth(1).unwrap() == "ls" {
            continue;
        }

        let result = split.clone().nth(0).unwrap().parse::<i32>();
        
        if result.is_ok() {

            let result = result.unwrap();

            for i in 1..cur_path.len()+1 {
                let reduced_path : Vec<String> = cur_path.clone().iter().take(i).map(|x| String::from(x)).collect();
                let path = convert_path_vec_to_string(&reduced_path);
                path_hash = update_path_with_file(path, path_hash, result as u32);
            }
        }
    }

    return path_hash.values().filter(|&&x| x <= 100000).sum();

}


fn convert_path_vec_to_string(path: &Vec<String>) -> String {

    if path.len() == 1 {
        return String::from("/");
    }

    let mut ret_string : String = String::new();

    for s in path.iter().skip(1) {
        ret_string.push_str("/");
        ret_string.push_str(s.as_str());
    }

    return ret_string;

}

fn update_path_with_file(path: String, mut map: HashMap<String, u32>, size: u32) -> HashMap<String, u32> {

    if map.contains_key(&path) {
        let cur_size = map.get(&path).unwrap();
        map.insert(path.clone(), cur_size + size);
        return map;
    }
    else {
        map.insert(path.clone(), size);
    }
    return map;
}

#[test]
fn test_convert_path() {

    let input : Vec<String> = ["/", "lmtpm"].map(String::from).to_vec();
    let input2 : Vec<String> = ["/", "lmtpm", "exexec"].map(String::from).to_vec();
    let expected = String::from("/lmtpm");
    let expected2 = String::from("/lmtpm/exexec");

    assert_eq!(expected, convert_path_vec_to_string(&input));
    assert_eq!(expected2, convert_path_vec_to_string(&input2));

}

#[test]
fn test2() {

    let expected = 95437;
    let result = directory_sum("src/day7_t.txt");

    assert_eq!(expected, result);
}
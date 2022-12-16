// finding common item among groups of three elves and summing their priorities




use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

/*
Determining shared items between rucksacks and the sum of their priorities

A rucksack is defined as a String with upper and lowecase letters associated with items
ex:
vJrwpWtwJgWrhcsFMMfFFhFp

Each rucksack has two compartments holding same number of items



*/


pub fn group_item_priorities(filename : &str) -> i32 {

    let f = File::open(filename).expect("file not found");
    let reader = BufReader::new(f);

    let mut sum = 0;

    let mut group: [String; 3] = ["", "", ""].map(String::from);

    for (i, line) in reader.lines().enumerate() {
        let fill = i % 3;
        group[fill] = line.expect("Failure breaking file into lines").trim().to_string();

        if fill == 2 {
            let badge = get_badge(&group);
            sum += get_priority(&badge.clone());
        }
    }

    return sum;
}

fn get_priority(letter : &String) -> i32 {
    
    assert!(letter.chars().count() == 1);
    let char_letter = letter.chars().next().unwrap();
    //println!("converting character: {} to digits: {}", char_letter.to_string(), char_letter as u32);

    let mut offset : i32 = 96;
    if char_letter.is_uppercase() {
        offset = 38;
    }

    return (char_letter as i32) - offset;

}

fn get_badge(group: &[String; 3]) -> String {

    let (first, second, third) = (group[0].clone(), group[1].clone(), group[2].clone());
    let mut shared : Vec<String> = Vec::new();

    for letter in first.chars() {

        let result = second.chars().find(|&x| x == letter);
        match result {
            Some(_) => {
                let s = letter.to_string();
                if !shared.contains(&s) {
                    shared.push(letter.to_string());
                }
            }
            None => continue,
        }
    }
    for s in shared.iter() {
        let result = third.chars().find(|x| &x.to_string() == s);
        match result {
            Some(_) => {
                return result.unwrap().to_string();
            }
            None => continue,
        }
     }
    return String::from("Err, no badge found");
}

#[test]
fn test_halves() {

    let line = "vJrwpWtwJgWrhcsFMMfFFhFp";

    let expected_first = "vJrwpWtwJgWr";
    let expected_second = "hcsFMMfFFhFp";

    let first_half = &line[0..line.len()/2];
    let second_half = &line[line.len()/2..line.len()];

    assert_eq!(expected_first, first_half);
    assert_eq!(expected_second, second_half);

}

#[test]
fn test_get_priority() {



    let input = String::from("p");

    let expected = 16;


    assert_eq!(expected, get_priority(&input));

}



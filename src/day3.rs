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


pub fn shared_item_priorities(filename : &str) -> i32 {

    let f = File::open(filename).expect("file not found");
    let reader = BufReader::new(f);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.expect("Failure breaking file into lines");
        let line = line.trim();

        let first_half = &line[0..line.len()/2];
        let second_half = &line[line.len()/2..line.len()];
        
        let shared_letters: Vec<String> = get_shared_letters(first_half, second_half);
        sum += get_sum_priorities(&shared_letters);
    }

    return sum;
}

fn get_sum_priorities(letters : &Vec<String>) -> i32 {

    let mut sum = 0;

    for str in letters.iter() {
        sum += get_priority(str);
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

fn get_shared_letters<'a>(first: &'a str, second: &'a str) -> Vec<String> {

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
    return shared;
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
fn test_common() {

    let line = "vJrwpWtwJgWrhcsFMMfFFhFp";

    let first_half = &line[0..line.len()/2];
    let second_half = &line[line.len()/2..line.len()];

    let expected = vec![String::from("p")];

    assert_eq!(expected, get_shared_letters(first_half, second_half));

}

#[test]
fn test_get_priority() {



    let input = String::from("p");

    let expected = 16;


    assert_eq!(expected, get_priority(&input));

}
#[test]
fn test_get_sum_priority() {

    let vector = vec![String::from("p")];
    let expected = 16;

    assert_eq!(expected, get_sum_priorities(&vector));

}
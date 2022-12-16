use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

/*
Rock - Paper - Scissors strategy guide

Find total score of a given strategy guide input:
ex: (Opponent ... You)

Rock Paper  (score is 2 + 6 = 8)
Paper Rock  (score is 1 + 0 = 1)
Scissors Scissors (score is 3 + 3 = 6)

Score is calculated as:
Sum of score of each round, where the score for each round is...

score for shape selected + score for outcome

shapes:
Rock : 1 pt
Paper : 2 pt
Scissors : 3 pt

outcomes:
Loss : 0 pts
Tie : 3 pts
Win : 6 pts
*/

pub fn get_score(filename : &str) -> i32 {

    let f : File = File::open(filename).expect("file does not exist");

    let reader = BufReader::new(f);
    let mut score : i32 = 0;

    for line in reader.lines() {

        let unwrapped_line = line.expect("failure breaking file into lines");

        let line_string : &str = unwrapped_line.trim();

        let str_vec : Vec<&str> = line_string.split(" ").collect();
        let round_score = line_score(&str_vec);
        
        score += round_score;
    }

    return score;
}

fn line_score(str_vec : &Vec<&str>) -> i32 {

    let theirs = str_vec[0];
    let ours = str_vec[1];
    let mut score = 0;
    match ours {
        "X" => {
            score += 1;
            match theirs {
                "A" => score += 3,
                "B" => (),
                "C" => score += 6,
                _ => (),
            }
        },
        "Y" => {
            score += 2;
            match theirs {
                "A" => score += 6,
                "B" => score += 3,
                "C" => (),
                _ => (),
            }
        }
        "Z" => {
            score += 3;
            match theirs {
                "A" => (),
                "B" => score += 6,
                "C" => score += 3,
                _ => (),
            }
        }
        _ => (),
    }

    return score;

}


// #[test]
// fn test_score() {

//     let expected = 15;
//     assert_eq!(expected, get_score("src/day2.txt"));

// }



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

/*
Part 2:

Letters in second column mean we need to:
X = Lose
Y = Tie
Z = Win

*/


pub fn get_score2(filename : &str) -> i32 {

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
// rock beats scissors beats paper beats rock
// A -> C -> B -> A
// A = 1 (Rock)
// B = 2 (Paper)
// C = 3 (Scissors)

    let theirs = str_vec[0];
    let ours = str_vec[1];
    let mut score = 0;
    match ours {
        "X" => {
            // we need to lose
            match theirs {
                "A" => score += 3,
                "B" => score += 1,
                "C" => score += 2,
                _ => (),
            }
        },
        "Y" => {
            // Tie
            score += 3;
            match theirs {
                "A" => score += 1,
                "B" => score += 2,
                "C" => score += 3,
                _ => (),
            }
        }
        "Z" => {
            // Win
            score += 6;
            match theirs {
                "A" => score += 2,
                "B" => score += 3,
                "C" => score += 1,
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
//     assert_eq!(expected, get_score2("src/day2.txt"));

// }



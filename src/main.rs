mod day1;
mod day1_2;
mod day2;
mod day2_2;
mod day3;
mod day3_2;
mod day4;
mod day4_2;
mod day5;
mod day5_2;
mod day6;
mod day6_2;
mod day7;
mod day7_2;
mod day8;
mod day8_2;
mod day9;
mod day9_2;

fn main() {
    
    let max = day1::find_max_sum("src/day1.txt");
    println!("Day 1 result: {}", max);

    let top_three_sum = day1_2::total_top_three("src/day1.txt");
    println!("Day 1 part 2 result: {}", top_three_sum);

    let total_score = day2::get_score("src/day2.txt");
    println!("Day 2 result: {}", total_score);

    let total_score2 = day2_2::get_score2("src/day2.txt");
    println!("Day 2 part 2 result: {}", total_score2);

    let sum_priorities = day3::shared_item_priorities("src/day3.txt");
    println!("Day 3 result: {}", sum_priorities);

    let group_sum = day3_2::group_item_priorities("src/day3.txt");
    println!("Day 3 part 2 result {}", group_sum);

    let num_complete_overlap = day4::overlapping_ranges("src/day4.txt");
    println!("Day 4 result: {}", num_complete_overlap);

    let num_complete_overlap2 = day4_2::overlapping_ranges2("src/day4.txt");
    println!("Day 4 part 2 result: {}", num_complete_overlap2);

    let final_state = day5::find_tops_of_stacks("src/day5.txt");
    println!("Day 5 result: {}", final_state);

    let final_state2 = day5_2::find_tops_of_stacks2("src/day5.txt");
    println!("Day 5 part 2 result: {}", final_state2);

    let sequence_index = day6::day6main("src/day6.txt");
    println!("Day 6 result: {}", sequence_index);

    let sequence_index2 = day6_2::day6main2("src/day6.txt");
    println!("Day 6 part 2 result: {}", sequence_index2);

    let directory_sum = day7::directory_sum("src/day7.txt");
    println!("Day 7 result: {}", directory_sum);

    let directory_sum = day7_2::directory_sum("src/day7.txt");
    println!("Day 7 part 2 result: {}", directory_sum);

    let num_visible_trees = day8::visible_trees("src/day8.txt");
    println!("Day 8 result: {}", num_visible_trees);

    let max_scenic_score = day8_2::max_scenic_score("src/day8.txt");
    println!("Day 8 part 2 result: {}", max_scenic_score);

    let num_tail_positions = day9::num_tail_positions("src/day9.txt");
    println!("Day 9 result: {}", num_tail_positions);

    let num_tail_positions2 = day9_2::num_tail_positions("src/day9.txt");
    println!("Day 9 part 2 result: {}", num_tail_positions2);

}

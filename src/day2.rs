use super::intcode_computer::*;

pub fn day(input: std::string::String) {
    let input_vec = Computer::parse_input(input);

    let mut result1_input = input_vec.clone();
    result1_input[1] = 12;
    result1_input[2] = 2;

    println!(
        "Day 2 Result1: {:?}",
        Computer::simple(vec![0, 0], &result1_input)
    );
    println!("Day 2 Result2: {:?}", part_two(&input_vec));
}

fn part_two(input_vec: &Vec<i64>) -> i64 {
    let part_two_target = 19690720;

    let mut result2_input = input_vec.clone();

    for x in 0..99 {
        for y in 0..99 {
            result2_input[1] = x;
            result2_input[2] = y;

            if Computer::simple(vec![0, 0], &result2_input) == part_two_target {
                return 100 * x + y;
            }
        }
    }

    panic!("No value found")
}

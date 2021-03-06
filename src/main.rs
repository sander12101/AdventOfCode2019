#![allow(non_snake_case)]
#![allow(dead_code)]
use std::fs;
mod intcode_computer;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    day1::day(get_input(1));
    println!("");
    day2::day(get_input(2));
    println!("");
    day3::day(get_input(3));
    println!("");
    day4::day(get_input(4));
    println!("");
    day5::day(get_input(5));
    println!("");
    day6::day(get_input(6));
    println!("");
    day7::day(get_input(7));
    println!("");
    day8::day(get_input(8));
    println!("");
    day9::day(get_input(9));
    println!("");
    day10::day(get_input(10));
    println!("");
    day11::day(get_input(11));
    println!("");
    day12::day(get_input(12));
    println!("");
    day13::day(get_input(13));
    println!("");
    day14::day(get_input(14));
    println!("");
    day15::day(get_input(15));
    println!("");
    day16::day(get_input(16));
    println!("");
    day17::day(get_input(17));
    println!("");
    day18::day(get_input(18));
}

fn get_input(day: usize) -> String {
    let file_path = format!("input/day{}.txt", day);
    fs::read_to_string(file_path)
        .expect("Something went wrong!")
        .trim()
        .to_owned()
}

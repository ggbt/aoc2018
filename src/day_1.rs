use crate::aoc::read_input;

use std::collections::HashSet;

/// https://adventofcode.com/2018/day/1#part1
pub fn part_1() {
    let input = read_input("day_1.input");

    let changes = input.trim().lines();

    let mut frequency = 0_i64;

    for change in changes {
        frequency += change.parse::<i64>().unwrap();
    }

    println!("{}", frequency);
}

/// https://adventofcode.com/2018/day/1#part2
pub fn part_2() {
    let input = read_input("day_1.input");

    let changes = input.trim().lines();
    let mut frequency = 0_i64;

    let mut frequencies = HashSet::new();
    frequencies.insert(0_i64);

    for change in changes.cycle() {
        frequency += change.parse::<i64>().unwrap();

        if !frequencies.insert(frequency) {
            println!("First repeated frequency: {}", frequency);
            break;
        }
    }
}

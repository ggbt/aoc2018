use crate::aoc::read_input;

use std::usize;

/// https://adventofcode.com/2018/day/5#part1
pub fn part_1() {
    let input = read_input("day_5.input").trim().to_owned();

    let polymer = collapse_polymer(input.bytes());

    println!("{}", polymer.len());
}


/// https://adventofcode.com/2018/day/5#part2
pub fn part_2() {
    let input = read_input("day_5.input").trim().to_owned();

    let polymer = collapse_polymer(input.bytes());

    let mut min_polymer_size = usize::MAX;

    for unit in ('a' as u8)..('z' as u8) {
        let new_polymer = polymer.iter().cloned().filter(|&u| {
            u.to_ascii_lowercase() != unit
        });

        let new_polymer_size = collapse_polymer(new_polymer).len();

        if new_polymer_size < min_polymer_size {
            min_polymer_size = new_polymer_size;
        }
    }
    println!("{}", min_polymer_size);
}

fn collapse_polymer(polymer: impl Iterator<Item = u8>) -> Vec<u8> {
    let mut resulting_polymer: Vec<u8> = Vec::new();

    for unit in polymer {
        if let Some(last_unit) = resulting_polymer.last() {
            if last_unit ^ unit == 0b0010_0000 {
                resulting_polymer.pop();
            } else {
                resulting_polymer.push(unit);
            }
        } else {
            resulting_polymer.push(unit);
        }
    }

    resulting_polymer
}

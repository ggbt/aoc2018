use crate::aoc::read_input;

use std::collections::HashMap;

/// https://adventofcode.com/2018/day/2#part1
pub fn part_1() {
    let input = read_input("day_2.1.input");

    let box_ids = input.lines();

    let mut nr2 = 0_i64;
    let mut nr3 = 0_i64;

    for box_id in box_ids {
        let (got2, got3) = got_repeats(box_id);

        if got2 {
            nr2 = nr2 + 1;
        }

        if got3 {
            nr3 = nr3 + 1;
        }
    }

    println!("{}", nr2 * nr3)
}

/// Checks if the given string contains duplicate or triplicate characters.
fn got_repeats(input: &str) -> (bool, bool) {
    let mut map = HashMap::new();

    for ch in input.chars() {
        map.entry(ch)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let mut got2 = false;
    let mut got3 = false;

    for &value in map.values() {
        if value == 2 {
            got2 = true;
        } else if value == 3 {
            got3 = true;
        }

        if got2 && got3 {
            break;
        }
    }

    (got2, got3)
}

/// https://adventofcode.com/2018/day/2#part2
pub fn part_2() {
    let input = read_input("day_2.2.input");

    let box_ids = input.lines();
    let mut done = false;

    for box_id in box_ids.clone() {
        for box_id2 in box_ids.clone() {
            if box_id != box_id2 {
                if let Some(id) = find_prototype_fabric_id(box_id, box_id2) {
                    println!("{}", id);
                    done = true;
                    break;
                }
            }
        }
        if done {
            break;
        }
    }
}

/// If the two strings differ by only 1 character, remove that character and return the resulting string.
fn find_prototype_fabric_id(string1: &str, string2: &str) -> Option<String> {
    let mut index_of_diff = 0;
    let mut got_diff = false;
    let mut got_too_many_diffs = false;

    let zipped_iterator = string1.chars().zip(string2.chars());
    for (i, (c1, c2)) in zipped_iterator.enumerate() {
        if c1 != c2 {
            if got_diff == false {
                got_diff = true;
                index_of_diff = i;
            } else {
                got_too_many_diffs = true;
                break;
            };
        };
    };

    if got_diff && !got_too_many_diffs {
        let mut string = string1.to_string();
        string.remove(index_of_diff);
        Some(string)
    } else {
        None
    }
}

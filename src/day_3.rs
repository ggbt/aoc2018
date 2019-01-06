use crate::aoc::read_input;

use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;
use lazy_static::*;
use std::fmt;

struct Claim {
    id: u32,
    x_range: (u32, u32),
    y_range: (u32, u32)
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: x {}..{} y {}..{}",
               self.id,
               self.x_range.0, self.x_range.1,
               self.y_range.0, self.y_range.1
        )
    }
}

impl Claim {
    fn new (claim_description: &str) -> Claim {
        lazy_static! {
            static ref claims_regex: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }

        let captures = claims_regex.captures(claim_description).unwrap();

        let claim_id: u32 = captures[1].parse().unwrap();

        let start_x: u32 = captures[2].parse().unwrap();
        let size_x: u32 = captures[4].parse().unwrap();
        let end_x: u32 = start_x + size_x;

        let start_y: u32 = captures[3].parse().unwrap();
        let size_y: u32 = captures[5].parse().unwrap();
        let end_y: u32 = start_y + size_y;

        Claim {
            id: claim_id,
            x_range: (start_x, end_x),
            y_range: (start_y, end_y)
        }
    }
}

/// https://adventofcode.com/2018/day/3#part1
pub fn part_1() {
    let input = read_input("day_3.input");

    let claims = input.lines().map(Claim::new);
    let mut claimed_coordinates = HashMap::new();

    for claim in claims {
        for x in claim.x_range.0..claim.x_range.1 {
            for y in claim.y_range.0..claim.y_range.1 {
                let patch = claimed_coordinates.entry((x, y)).or_insert(0);
                *patch += 1;
            }
        }
    }

    let common_patches = claimed_coordinates.values()
        .filter(|patch| **patch > 1)
        .count();

    println!("{}", common_patches);
}

/// https://adventofcode.com/2018/day/3#part2
pub fn part_2() {
    let input = read_input("day_3.input");

    let claims = input.lines().map(Claim::new);
    let mut claimed_coordinates = HashMap::new();

    let mut all_claims = HashSet::new();
    let mut intersecting_claims = HashSet::new();


    for claim in claims {
        all_claims.insert(claim.id);

        for x in claim.x_range.0..claim.x_range.1 {
            for y in claim.y_range.0..claim.y_range.1 {
                let entry = claimed_coordinates.entry((x, y));

                entry.and_modify(|claim_id| {
                    intersecting_claims.insert(*claim_id);
                    intersecting_claims.insert(claim.id);

                    *claim_id = claim.id;
                }).or_insert(claim.id);
            };
        }
    }

    let non_intersecting_claim = all_claims.difference(&intersecting_claims)
        .next().unwrap();
    println!("{}", non_intersecting_claim);
}

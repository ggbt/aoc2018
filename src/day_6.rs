use crate::aoc::read_input;

use std::i32;
use regex::Regex;
use lazy_static::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

/// Returns the point from the HashMap which is closest to the given point
/// Or `None` if there are multiple points at the same distance from the given `Point`.
fn find_closest_point (point: Point, points: &HashMap<Point, i32>) -> Option<Point> {
    let mut smallest_distance: i32 = i32::MAX;
    let mut closest_point: Option<Point> = None;

    let mut nr_of_closest_points = 0;

    for (p, _) in points.iter() {
        let curr_distance = point.distance(p);
        if curr_distance < smallest_distance {
            nr_of_closest_points = 1;

            smallest_distance = curr_distance;
            closest_point = Some(Point { x: p.x, y: p.y });
        } else if curr_distance == smallest_distance {
            nr_of_closest_points = nr_of_closest_points + 1;
        }
    }

    if nr_of_closest_points == 1 {
        closest_point
    } else {
        None
    }
}

/// Returns `true` if the given point is close to all the given points.
fn point_is_close_to_all(point: Point, points: &HashMap<Point, i32>) -> bool {
    let mut total_distance = 0;

    for (p, _) in points.iter() {
        total_distance = total_distance + point.distance(p);
    }

    total_distance < 10000
}

/// Retrieve the input points.
fn get_points() -> (Point, Point, HashMap<Point, i32>) {
    lazy_static! {
        static ref reg: Regex = Regex::new(r"(\d+), (\d+)").unwrap();
    }
    let input = read_input("day_6.input").trim().to_owned();

    let mut min_point = Point { x: i32::MAX, y: i32::MAX };
    let mut max_point = Point { x: i32::MIN, y: i32::MIN };

    let points: HashMap<Point, i32> = input.lines().fold(HashMap::new(), |mut hash_map, line| {
        let captures = reg.captures(line).unwrap();

        let x: i32 = captures[1].parse().unwrap();
        let y: i32 = captures[2].parse().unwrap();

        if x < min_point.x {
            min_point.x = x;
        }

        if y < min_point.y {
            min_point.y = y;
        }

        if x > max_point.x {
            max_point.x = x;
        }

        if y > max_point.y {
            max_point.y = y;
        }

        hash_map.insert(Point { x, y }, 0);
        hash_map
    });

    (min_point, max_point, points)
}

/// https://adventofcode.com/2018/day/6#part1
pub fn part_1() {
    let (min_point, max_point, mut points) = get_points();

    for x in min_point.x..=max_point.x {
        for y in min_point.y..=max_point.y {
            let closest_point = find_closest_point(Point { x, y }, &points);

            if let Some(point) = closest_point {
                points.entry(point).and_modify(|area| *area += 1);
            }
        };
    }

    let mut infinite_points: HashSet<Point> = HashSet::new();

    // top & down edge
    for x in min_point.x - 2..=max_point.x + 2 {
        let y1 = min_point.x - 2;
        let closest_point = find_closest_point(Point { x, y: y1 }, &points);

        if  let Some(point) = closest_point {
            infinite_points.insert(point);
        }

        let y2 = max_point.x + 2;
        let closest_point = find_closest_point(Point { x, y: y2 }, &points);


        if let Some(point) = closest_point {
            infinite_points.insert(point);
        }
    }

    // left & right edge
    for y in min_point.y - 2..=max_point.y + 2 {
        let x1 = min_point.y - 2;
        let closest_point = find_closest_point(Point { x: x1, y }, &points);

        if let Some(point) = closest_point {
            infinite_points.insert(point);
        }

        let x2 = max_point.y + 2;
        let closest_point = find_closest_point(Point { x: x2, y }, &points);

        if let Some(point) = closest_point {
            infinite_points.insert(point);
        }
    }

    for point in infinite_points {
        points.remove(&point);
    }

    let mut max_area = i32::MIN;
    for (_, area) in points {
        if area > max_area {
            max_area = area;
        }
    }

    println!("{}", max_area);
}

/// https://adventofcode.com/2018/day/6#part2
pub fn part_2() {
    let (min_point, max_point, points) = get_points();

    let mut total_close_area = 0;
    for x in (min_point.x - 200)..=(max_point.x + 200) {
        for y in (min_point.y - 200)..=(max_point.y + 200) {
            if point_is_close_to_all(Point { x, y }, &points) {
                total_close_area = total_close_area + 1;
            }
        };
    }

    println!("{}", total_close_area);
}

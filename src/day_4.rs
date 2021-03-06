use crate::aoc::read_input;

use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use chrono::{NaiveDateTime, Timelike};
use regex::Regex;
use lazy_static::*;
use std::cmp;

#[derive(Debug)]
enum Action {
    BeginsShift(u32),
    FallsAsleep(u32),
    WakesUp(u32)
}

#[derive(Debug)]
struct Record {
    date_time: NaiveDateTime,
    action: Action
}

impl Record {
    fn from_str (string: &str) -> Record {
        lazy_static! {
            static ref record_regex: Regex = Regex::new(
                r"\[([^\]]+)] ([^ ]+) #?([^ ]+)"
            ).unwrap();
        }

        let captured_groups = record_regex.captures(string).unwrap();

        let date_time = NaiveDateTime::parse_from_str(
            &captured_groups[1],
            "%Y-%m-%d %H:%M"
        ).unwrap();

        let action = match &captured_groups[2] {
            "Guard" => {
                let guard_id: u32 = captured_groups[3].parse().unwrap();
                Action::BeginsShift(guard_id)
            },
            "falls" => Action::FallsAsleep(date_time.minute()),
            "wakes" => Action::WakesUp(date_time.minute()),

            _ => panic!("invalid input")
        };

        Record {date_time, action}
    }
}

fn calculate_guards_sleep_schedules() -> HashMap<u32, Vec<u32>, RandomState> {
    let mut records: Vec<Record> = read_input("day_4.input")
        .lines()
        .map(Record::from_str)
        .collect();

    records.sort_by(|a, b| a.date_time.cmp(&b.date_time));

    let mut guard_sleep_schedules = HashMap::new();

    let mut current_guard_id: u32 = 0;
    let mut sleep_start_minute: usize = 0;

    for record in records {
        match record.action {
            Action::BeginsShift(guard_id) => {
                guard_sleep_schedules.entry(guard_id).or_insert(vec![0_u32; 59]);
                current_guard_id = guard_id;
            },
            Action::FallsAsleep(minute) => {
                sleep_start_minute = minute as usize;
            },
            Action::WakesUp(minute) => {
                let sleep_schedule = guard_sleep_schedules.get_mut(&current_guard_id).unwrap();
                for i in sleep_start_minute..minute as usize {
                    *sleep_schedule.get_mut(i).unwrap() += 1;
                }
            }
        };
    }

    guard_sleep_schedules
}

/// https://adventofcode.com/2018/day/4#part1
pub fn part_1() {
    let guards_sleep_schedules = calculate_guards_sleep_schedules();

    let mut max_slept_nr_minutes: u32 = 0;
    let mut max_slept_guard_id: u32 = 0;
    let mut max_slept_minute: u32 = 0;

    for (guard_id, sleep_schedule) in guards_sleep_schedules {
        let mut max_slept_times_at_minute: u32 = 0;
        let mut local_max_slept_minute: u32 = 0;
        let mut local_max_slept_nr_minutes: u32 = 0;

        for i in 0..sleep_schedule.len() {
            let slept_times_at_minute = sleep_schedule.get(i).unwrap();
            local_max_slept_nr_minutes += slept_times_at_minute;

            max_slept_times_at_minute = cmp::max(max_slept_times_at_minute, *slept_times_at_minute);
            if max_slept_times_at_minute == *slept_times_at_minute {
                local_max_slept_minute = i as u32;
            }
        }

        if max_slept_nr_minutes < local_max_slept_nr_minutes {
            max_slept_nr_minutes = local_max_slept_nr_minutes;
            max_slept_guard_id = guard_id;
            max_slept_minute = local_max_slept_minute;
        }
    }

    println!("{}", max_slept_guard_id * max_slept_minute);
}

/// https://adventofcode.com/2018/day/4#part2
pub fn part_2() {
    let guards_sleep_schedules = calculate_guards_sleep_schedules();

    let mut max_guard_id: u32 = 0;
    let mut max_slept_minute_value: u32 = 0;
    let mut max_slept_minute_index: u32 = 0;

    for (guard_id, sleep_schedule) in guards_sleep_schedules {
        let mut max_value: u32 = 0;
        let mut index_of_max_value: usize = 0;

        for i in 0..sleep_schedule.len() {
            let value = sleep_schedule.get(i).unwrap();

            if *value > max_value {
                max_value = *value;
                index_of_max_value = i;
            }
        }

        if max_value > max_slept_minute_value {
            max_slept_minute_value = max_value;
            max_guard_id = guard_id;
            max_slept_minute_index = index_of_max_value as u32;
        }
    }

    println!("{}", max_guard_id * max_slept_minute_index);
}

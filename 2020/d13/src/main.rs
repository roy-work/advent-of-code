use std::collections::HashSet;
use std::convert::TryFrom;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

fn parser() -> anyhow::Result<(i64, Vec<Option<i64>>)> {
    let file = BufReader::new(File::open("input")?);
    let mut lines = file.lines();
    let earliest_departure = lines.next().unwrap()?.parse::<i64>().unwrap();
    let second_line = lines.next().unwrap()?;
    let mut schedule = Vec::new();
    for part in second_line.split(',') {
        if part == "x" {
            schedule.push(None);
        } else {
            schedule.push(Some(part.parse::<i64>().unwrap()));
        }
    }
    Ok((earliest_departure, schedule))
}

fn part_a(input: &(i64, Vec<Option<i64>>)) -> i64 {
    let (earliest_departure, schedules) = input;

    let mut first_bus: Option<(i64, i64)> = None;
    for schedule in schedules {
        let schedule = match schedule {
            Some(s) => s,
            None => continue,
        };
        let next_dept = earliest_departure / schedule * schedule;
        let next_dept = if next_dept < *earliest_departure {
            next_dept + schedule
        } else {
            next_dept
        };

        match first_bus {
            Some((_, best_yet)) => {
                if next_dept < best_yet {
                    first_bus = Some((*schedule, next_dept));
                }
            }
            None => {
                first_bus = Some((*schedule, next_dept));
            }
        }
    }

    let (bus_id, bus_time) = first_bus.unwrap();
    let wait = bus_time - *earliest_departure;
    wait * bus_id
}

fn solve_for_coherence(first_skip: i64, second_idx: i64, second_skip: i64) -> i64 {
    let mut n = 0;
    loop {
        if ((first_skip * n + second_idx) % second_skip) == 0 {
            break;
        }
        n += 1;
    }

    println!("coherence({}, {}, {}) => {}", first_skip, second_idx, second_skip, n);
    assert!((first_skip * n + second_idx) % second_skip == 0);
    assert!((first_skip * (n + second_skip) + second_idx) % second_skip == 0);
    assert!((first_skip * (n + second_skip * 2) + second_idx) % second_skip == 0);
    n
}

fn test_solver() {
    let answer = part_b_solve(&vec![Some(7), Some(13), None, None, Some(59), None, Some(31), Some(19)]);
    assert!(answer == 1068781);
}

fn is_t_lined_up(t: i64, bus: &(usize, i64)) -> bool {
    (t + bus.0 as i64) % bus.1 == 0
}

fn find_line_up_and_stride(t_base: i64, t_stride: i64, bus: &(usize, i64)) -> (i64, i64) {
    let mut t = t_base;
    let mut first_line_up = None;
    println!("lining up: {}, {}", t_base, t_stride);
    loop {
        if is_t_lined_up(t, &bus) {
            println!("lines up at {} w/ {:?}", t, &bus);
            match first_line_up {
                Some(l) => return (l, t - l),
                None => first_line_up = Some(t),
            }
        }
        t += t_stride;
    }
}

fn part_b_solve(schedules: &Vec<Option<i64>>) -> i64 {
    let schedules_with_times: Vec<(usize, i64)> = schedules
        .iter()
        .enumerate()
        .filter_map(|(i, s)| s.map(|s| (i, s)))
        .collect::<Vec<_>>();

    let (max_idx, max) = schedules_with_times
        .iter()
        .max_by_key(|(_, s)| *s)
        .unwrap();

    let multiple_of = schedules_with_times[0].1;

    let sorted = {
        let mut s = schedules_with_times.clone();
        s.sort_by_key(|(_, s)| -s);
        s
    };
    let coherence = solve_for_coherence(multiple_of, *max_idx as i64, *max);

    //let mut base_t = max - (max_idx as i64);
    println!("1st point: {}", multiple_of * (coherence + max * 1));
    println!("2nd point: {}", multiple_of * (coherence + max * 2));

    println!("multiple: {}, coherence: {}, max: {}", multiple_of, coherence, max);

    /*
    let mut n = 1;
    let mut times_ok = 0;
    loop {
        //let t = multiple_of * (coherence + max * n);
        let t = n;
        if is_t_lined_up(t, &sorted[0]) {
            println!("lines up at {} w/ {:?}", t, &sorted[1]);
            times_ok += 1;
            if times_ok == 2 {
                break;
            }
        }
        n += 1;
    }
    */
    let (l, s) = find_line_up_and_stride(0, multiple_of, &sorted[0]);
    println!("{:?}", (l, s));
    let (l, s) = find_line_up_and_stride(l, s, &sorted[1]);
    println!("{:?}", (l, s));
    let (l, s) = find_line_up_and_stride(l, s, &sorted[2]);
    println!("{:?}", (l, s));
    let (l, s) = find_line_up_and_stride(l, s, &sorted[3]);
    println!("{:?}", (l, s));

    let mut t = l;
    loop {
        if schedules_with_times.iter().all(|bus| is_t_lined_up(t, bus)) {
            return t;
        }
        t += s;
    }
    panic!();

    // LOL DEAD CODE HERE
    // Things were tried.
    // (Actually, the solving program is in here! Maybe. I don't remember exactly which early
    // version solved it, but I think I manually solved for the line up of the first bus & the
    // largest bus, and let that run in the background. It solved things before I could figure them
    // out for real.)

    let mut last_output = 0;
    let mut n = 1;
    let mut base_t = multiple_of * (coherence + max * n);
    loop {
        let mut good = true;
        for (idx, schedule) in schedules_with_times.iter() {
            let this_t = base_t + (*idx as i64);
            if this_t % schedule != 0 {
                good = false;
                break;
            }
        }

        if good {
            return base_t as i64;
        }

        n += 1;
        base_t = multiple_of * (coherence + max * n);
        //base_t += max * 449 * 37 * 19 * 13 * 17 * 23;
        /*
        loop {
            base_t += max;
            if base_t % multiple_of == 0 {
                break;
            }
        }
        */

        if 100_000_000_000 < base_t - last_output {
            println!("...{}", base_t);
            last_output = base_t;
        }
    }
}


fn part_b(input: &(i64, Vec<Option<i64>>)) -> i64 {
    let (_, schedules) = input;

    test_solver();
    println!("=== TEST RUN PASSED ===");
    part_b_solve(schedules)
}

aoc::aoc!(parser, part_a, part_b);

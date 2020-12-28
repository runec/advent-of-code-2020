use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day13";


pub fn solve() -> Result<(), Box<dyn Error>> {
  let (depart_time, bus_times) = parse_input()?;
  solve_part_one(depart_time, &bus_times);
  solve_part_two(&bus_times);
  return Ok(());
}

fn solve_part_one(depart_time: u128, bus_times: &Vec<Option<u128>>) {
  let (id, bus_time) = bus_times.iter().fold(None, |min, t| {
    match t {
      None => min,
      Some(v1) => {
        let next_time = next_depart_time(*v1, depart_time);
        match min {
          None => Some((*v1, next_time)),
          Some((_, time)) => if next_time < time {
            Some((*v1, next_time))
          } else {
            min
          }
        }
      }
    }
  }).unwrap();

  println!("Part 1: {}", (bus_time - depart_time) * id);
}

fn solve_part_two(bus_times: &Vec<Option<u128>>) {
  let mut increment: u128 = 1;
  let mut time: u128 = 0;
  for i in 0..bus_times.len() {
    let bus_time = bus_times[i];
    match bus_time {
      None => (),
      Some(t) => {
        while time.rem_euclid(t) != (t as i128 - i as i128).rem_euclid(t as i128) as u128 {
          println!("Tst {}, {} ---- {}, {}, {}", time.rem_euclid(t), i, increment, time, t);
          time += increment;
        }
        increment = least_common_multiple(increment, t);
      }
    }
  }
  println!("Part 2: {}", time);
}

fn next_depart_time(bus_time: u128, from: u128) -> u128 {
  divide_round_up(from, bus_time)
}

fn least_common_multiple(a: u128, b: u128) -> u128 {
  let (mut v1, mut v2) = (a, b);
  while v1 != v2 {
    if v1 < v2 {
      v1 += divide_round_up(v2 - v1, a);
    } else {
      v2 += divide_round_up(v1 - v2, b);
    }
  }

  v1
}

fn divide_round_up (a: u128, b: u128) -> u128 {
  (1 + (a - 1) / b) * b
}

fn parse_input() -> Result<(u128, Vec<Option<u128>>), Box<dyn Error>> {
  let mut lines = helpers::read_lines(INPUT_FILE)?;

  let depart_time = lines.next().unwrap()?.parse::<u128>()?;
  let bus_plan_string = lines.next().unwrap()?;


  let times: Vec<Option<u128>> = bus_plan_string.split(',').map(|t| t.parse::<u128>().ok()).collect();
  Ok((depart_time, times))
}

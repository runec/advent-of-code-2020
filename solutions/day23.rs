use crate::helpers;
use std::error::Error;
use std::collections::VecDeque;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/day23";

const MILLION: i64 = 1000000;

type Cups = HashMap<i64, i64>;

pub fn solve() -> Result<(), Box<dyn Error>> {
  let mut initial_state = parse_input()?;
  solve_part_one(&mut initial_state);
  let mut initial_state = parse_input()?;
  solve_part_two(&mut initial_state);
  return Ok(());
}

fn solve_part_one(cups: &mut VecDeque<i64>) {
  let mut cups2: Cups = HashMap::new();
  let first_val = cups[0];
  let mut prev_val = cups[0];
  for i in 1..cups.len() {
    cups2.insert(prev_val, cups[i]);
    prev_val = cups[i];
  }
  cups2.insert(prev_val, first_val);

  let mut val = first_val;

  for _ in 0..100 {
    move_cups2(&mut cups2, val);
    val = *cups2.get(&val).unwrap()
  }

  println!("Part 1: {}", get_order_after_val(&cups2, 1));
}

fn solve_part_two(cups: &mut VecDeque<i64>) {
  let mut cups2: Cups = HashMap::new();
  let first_val = cups[0];
  let mut prev_val = cups[0];

  for i in 1..MILLION {
    if (i as usize) < cups.len() {
      cups2.insert(prev_val, cups[i as usize]);
      prev_val = cups[i as usize];
    } else {
      cups2.insert(prev_val, i + 1);
      prev_val = i + 1;
    }
  }
  cups2.insert(prev_val, first_val);

  let mut val = first_val;
  for i in 0..10*MILLION {
    if i % 10000 == 0 {
      println!("Round {}", i);
    }
    move_cups2(&mut cups2, val);
    val = *cups2.get(&val).unwrap()
  }

  let &first = cups2.get(&1).unwrap();
  let &second = cups2.get(&first).unwrap();

  println!("Answer {}", first * second);
}

fn print(cups: &Cups) {
  for i in 1..=cups.len() {
    println!("{} -> {:?}", i, cups.get(&(i as i64)))
  }
}

fn get_order_after_val(cups: &Cups, v: i64) -> String {
  let mut val = v;
  let mut nums = vec![];
  loop {
    val = *cups.get(&val).unwrap();
    if val == v {
      break;
    } else {
      nums.push(val);
    }
  }
  nums.iter().map(|v| v.to_string()).collect::<String>()
}

fn get_labeling(cups: &VecDeque<i64>) -> String {
  cups.iter().map(|v| v.to_string()).collect::<String>()
}

fn get_order_after_1(cups: &VecDeque<i64>) -> String {
  let index_of_1 = cups.iter().position(|&c| c == 1).unwrap();
  let mut vals = vec![];
  for i in index_of_1+1..cups.len() {
    vals.push(cups[i].to_string());
  }
  for i in 0..index_of_1 {
    vals.push(cups[i].to_string());
  }

  vals.into_iter().collect::<String>()
}

fn move_cups(cups: &mut VecDeque<i64>, curr: usize, num: usize) {
  let num_cups = cups.len();
  let curr_val = cups[curr];
  let mut moved_cups = vec![];
  for _ in curr+1..=curr + num {
    let index_to_remove = if curr >= cups.len()-1 {0} else {curr+1};
    moved_cups.push(cups.remove(index_to_remove).unwrap());
  }
  let mut target_val = (curr_val - 2).rem_euclid(num_cups as i64) + 1;
  while moved_cups.contains(&target_val) {
    target_val = (target_val - 2).rem_euclid(num_cups as i64) + 1;
  }

  let target = cups.iter().position(|&v| v == target_val).unwrap();
  for c in moved_cups.iter().rev() {
    cups.insert((target + 1).rem_euclid(num_cups), *c);
  }
/*
  //println!("{}, {}, {}", curr, curr_val, cups[curr]);
  while cups[curr] != curr_val {
    //println!("rotating {}", get_labeling(cups));
    cups.rotate_right(1);
    //println!("rotated {}", get_labeling(cups));
  }
  */
}

fn move_cups2(cups: &mut Cups, curr_val: i64) {
  // Move forward 3, find value
  let mut moved_vals = vec![0,0,0];
  let mut last_moved = curr_val;
  let mut target = (curr_val - 2).rem_euclid(cups.len() as i64) + 1;
  for i in 0..3 {
    last_moved = *cups.get(&last_moved).unwrap();
    moved_vals[i] = last_moved;
  }

  while moved_vals.contains(&target) {
    target = (target - 2).rem_euclid(cups.len() as i64) + 1;
  }

  let &next_val = cups.get(&last_moved).unwrap();
  let &after_target = cups.get(&target).unwrap();
  cups.insert(curr_val, next_val);
  cups.insert(target, moved_vals[0]);
  cups.insert(moved_vals[2], after_target);
}


fn parse_input() -> Result<VecDeque<i64>, Box<dyn Error>> {
  let mut lines = helpers::read_lines(INPUT_FILE)?.map(|l| l.unwrap());
  let line = lines.next().unwrap();

  let numbers: VecDeque<i64> = line.chars().map(|c| c.to_digit(10).unwrap() as i64).collect();

  Ok(numbers)
}
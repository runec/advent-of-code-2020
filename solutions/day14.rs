use crate::helpers;
use std::error::Error;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/day14";

type Mask = Vec<char>;

type Mem = (u64, u64);

pub fn solve() -> Result<(), Box<dyn Error>> {
  let lines = parse_input()?;
  solve_part_one(lines);
  let lines = parse_input()?;
  solve_part_two(lines);
  return Ok(());
}

fn solve_part_one(lines: impl Iterator<Item=String> ) {
  let mut current_mask: Option<Mask> = None;
  let mut memory: HashMap<u64, u64> = HashMap::new();

  for line in lines {
    if line.starts_with("mask") {
      current_mask = Some(parse_mask(line));
    } else if line.starts_with("mem") {
      if let Some(m) = &current_mask {
        let (addr, val) = parse_mem(line);
        // apply mask
        let masked_val = apply_mask(m, val);
        // save
        memory.insert(addr, masked_val);
      } else {
        println!("Mem without any mask");
      }
    } else {
      println!("Unkonwn line: {}", line);
    }
  }

  let mut sum = 0;
  for addr in memory.keys() {
    sum += memory.get(addr).unwrap();
  }

  println!("Part 1: {}", sum);
}

fn solve_part_two(lines: impl Iterator<Item=String> ) {
  let mut current_mask: Option<Mask> = None;
  let mut memory: HashMap<u64, u64> = HashMap::new();

  for line in lines {
    if line.starts_with("mask") {
      current_mask = Some(parse_mask(line));
    } else if line.starts_with("mem") {
      if let Some(m) = &current_mask {
        let (addr, val) = parse_mem(line);
        // apply mask
        let masked_addrs = apply_mask_two(m, addr);

        for addr in masked_addrs {
          memory.insert(addr, val);
        }
      } else {
        println!("Mem without any mask");
      }
    } else {
      println!("Unkonwn line: {}", line);
    }
  }

  let mut sum = 0;
  for addr in memory.keys() {
    sum += memory.get(addr).unwrap();
  }

  println!("Part 2: {}", sum);
}

fn apply_mask(mask: &Mask, val: u64) -> u64 {
  let mut masked_val = 0;
  for i in 0..mask.len() {
    let c = mask[i];
    masked_val *= 2;
    masked_val += match c {
      '0' => 0,
      '1' => 1,
      _ => (val >> (mask.len() - i - 1)) & 1
    };

    println!("xx {}, {}, {}, {}", i, masked_val, val, mask.iter().collect::<String>());
  }
  
  masked_val
}

fn apply_mask_two(mask: &Mask, addr: u64) -> Vec<u64> {
  let mut masked_values: Vec<u64> = vec![0];
  for i in 0..mask.len() {
    let c = mask[i];

    let mut new_values: Vec<u64> = vec![];
    
    for v in &mut masked_values {
      *v *= 2;
      match c {
        '0' => *v += (addr >> (mask.len() - i - 1)) & 1,
        '1' => *v += 1,
        _ => {
          new_values.push(*v + 1);
        }
      }
    }

    masked_values.append(&mut new_values);
  }
  
  masked_values
}
      
fn parse_input() -> Result<impl Iterator<Item=String>, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?.map(|l| l.unwrap());

  Ok(lines)
}

fn parse_mask(line: String) -> Mask {
  let mask_string = line.split('=').skip(1).next().unwrap().trim();

  mask_string.chars().collect()
}

fn parse_mem(line: String) -> Mem {
  let parts = line.split('=').collect::<Vec<_>>();
  let mem_part = parts[0].chars().filter(|c| c.is_digit(10)).collect::<String>();
  let val_part = parts[1].trim();

  (mem_part.parse::<u64>().unwrap(), val_part.parse::<u64>().unwrap())
}
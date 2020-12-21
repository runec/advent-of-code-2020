use crate::helpers;
use std::error::Error;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/day04";

type Passport = HashMap<String, String>;

pub fn solve() -> Result<(), Box<dyn Error>> {
  let passports = parse_input()?;

  solve_part_one(&passports);
  solve_part_two(&passports);
  return Ok(());
}

fn solve_part_one(passports: &Vec<Passport>) {
  let mut count = 0;
  for passport in passports {
    if passport_is_valid(passport) {
      count += 1;
    }
  }
  println!("Part 1 answer: {}", count);
}

fn solve_part_two(passports: &Vec<Passport>) {
  let mut count = 0;
  for passport in passports {
    if passport_is_valid_part_two(passport) {
      count += 1;
    }
  }
  println!("Part 2 answer: {}", count);
}

fn passport_is_valid(passport: &Passport) -> bool {
  return passport.get(&String::from("byr")).is_some()
    && passport.get(&String::from("iyr")).is_some()
    && passport.get(&String::from("eyr")).is_some()
    && passport.get(&String::from("hgt")).is_some()
    && passport.get(&String::from("hcl")).is_some()
    && passport.get(&String::from("ecl")).is_some()
    && passport.get(&String::from("pid")).is_some()
}

fn passport_is_valid_part_two(passport: &Passport) -> bool {
  return byr_valid(passport)
    && iyr_valid(passport)
    && eyr_valid(passport)
    && hgt_valid(passport)
    && hcl_valid(passport)
    && ecl_valid(passport)
    && pid_valid(passport)
}

fn byr_valid(passport: &Passport) -> bool {
  let val = passport.get("byr");
  return match val {
    None => false,
    Some(b) => {
      let val = b.parse::<u32>();
      match val {
        Err(_) => false,
        Ok(v) => v >= 1920 && v <= 2002,
      }
    }
  }
}

fn iyr_valid(passport: &Passport) -> bool {
  let val = passport.get("iyr");
  return match val {
    None => false,
    Some(b) => {
      let val = b.parse::<u32>();
      match val {
        Err(_) => false,
        Ok(v) => v >= 2010 && v <= 2020,
      }
    }
  }
}

fn eyr_valid(passport: &Passport) -> bool {
  let val = passport.get("eyr");
  return match val {
    None => false,
    Some(b) => {
      let val = b.parse::<u32>();
      match val {
        Err(_) => false,
        Ok(v) => v >= 2020 && v <= 2030,
      }
    }
  }
}

fn hgt_valid(passport: &Passport) -> bool {
  let val = passport.get("hgt");
  return match val {
    None => false,
    Some(b) => {
      let (hgt, unit) = b.split_at(b.len() - 2);
      let val = hgt.parse::<u32>();
      match val {
        Err(_) => false,
        Ok(v) => match unit {
          "in" => v >= 59 && v <= 76,
          "cm" => v >= 150 && v <= 193,
          _ => false,
        }
      }
    }
  }
}

fn hcl_valid(passport: &Passport) -> bool {
  let val = passport.get("hcl");
  return match val {
    None => false,
    Some(b) => {
      let (start, val) = b.split_at(1);
      start == "#" && val.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
    }
  }
}

fn ecl_valid(passport: &Passport) -> bool {
  let val = passport.get("ecl");
  return match val {
    None => false,
    Some(b) => 
      match b.as_str() {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
      }
  }
}

fn pid_valid(passport: &Passport) -> bool {
  let val = passport.get("pid");
  return match val {
    None => false,
    Some(b) => b.len() == 9 && b.chars().all(|c| c.is_digit(10))
  }
}

fn parse_input() -> Result<Vec<Passport>, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?;

  let mut passports: Vec<Passport> = vec![]; 
  let mut current_entry: Passport = HashMap::new();
  for line in lines {
    let line = line?;
    if line.trim().len() == 0 {
      // new passport, save current and create new
      passports.push(current_entry);
      current_entry = HashMap::new();
    } else {
      // parse all line
      let attributes = line.split(' ');
      for att in attributes {
        let parts = att.splitn(2, ':').collect::<Vec<_>>();
        if parts.len() == 2 {
          current_entry.insert(String::from(parts[0]), String::from(parts[1]));
        }
      }
    }
  }

  if !current_entry.is_empty() {
    passports.push(current_entry);
  }

  return Ok(passports);
}
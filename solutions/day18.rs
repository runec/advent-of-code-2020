use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day18";

pub fn solve() -> Result<(), Box<dyn Error>> {
  let lines = parse_input()?;
  solve_part_one(lines);
  let lines = parse_input()?;
  solve_part_two(lines);
  return Ok(());
}

fn solve_part_one(lines: impl Iterator<Item=String>) {
  let sum = lines.fold(0, |sum, l| {
    let mut chars = l.chars();
    sum + evaluate_expression(&mut chars).unwrap()
  });

  println!("Part 1: {}", sum);
}

fn solve_part_two(lines: impl Iterator<Item=String>) {
  let sum = lines.fold(0, |sum, l| {
    let mut chars = l.chars();
    sum + evaluate_expression_two(&mut chars).unwrap()
  });

  println!("Part 2: {}", sum);
}


fn evaluate_expression (chars: &mut impl Iterator<Item=char>) -> Option<u64> {
  let mut result: Option<u64> = None;
  let mut current_operator: Option<char> = None;
  while let Some(c) = chars.next() {
    match c {
      c if c.is_digit(10) => {
        let val = c.to_digit(10).unwrap() as u64;
        match result {
          None => result = Some(val),
          Some(n) => {
            result = match current_operator {
              Some('+') => Some(n + val),
              Some('*') => Some( n * val),
              _ => {
                println!("new digit without operator");
                result
              }
            };
            current_operator = None;
          }
        }
      }
      '+' => current_operator = Some('+'),
      '*' => current_operator = Some('*'),
      '(' => {
        let val = evaluate_expression(chars).unwrap();
        match result {
          None => result = Some(val),
          Some(n) => {
            result = match current_operator {
              Some('+') => Some(n + val),
              Some('*') => Some(n * val),
              _ => {
                println!("new digit without operator");
                result
              }
            };
            current_operator = None;
          }
        }
      },
      ')' => return result,
      _ => ()
    };
  }

  result
}

fn evaluate_expression_two (chars: &mut impl Iterator<Item=char>) -> Option<u64> {
  let mut result: Option<u64> = None;
  let mut current_operator: Option<char> = None;
  let mut multi_part: Option<u64> = None;

  while let Some(c) = chars.next() {
    match c {
      c if c.is_digit(10) => {
        let val = c.to_digit(10).unwrap() as u64;
        match result {
          None => match current_operator {
            Some('+') => match multi_part {
              None => multi_part = Some(val),
              Some(m) => multi_part = Some(m + val),
            }
            Some('*') => {
              if let Some(m) = multi_part {
                result = Some(m);
              }
              multi_part = Some(val);
            }
            _ => {
              multi_part = Some(val);
            }
          }
          Some(n) => {
            match current_operator {
              Some('+') => match multi_part {
                None => multi_part = Some(val),
                Some(m) => multi_part = Some(m + val),
              }
              Some('*') => {
                if let Some(m) = multi_part {
                  result = Some(n * m);
                }
                multi_part = Some(val);
              }
              _ => {
                println!("new digit without operator")
              }
            };
            current_operator = None;
          }
        }
      }
      '+' => current_operator = Some('+'),
      '*' => current_operator = Some('*'),
      '(' => {
        let val = evaluate_expression_two(chars).unwrap();
        match result {
          None => match current_operator {
            Some('+') => match multi_part {
              None => multi_part = Some(val),
              Some(m) => multi_part = Some(m + val),
            }
            Some('*') => {
              if let Some(m) = multi_part {
                result = Some(m);
              }
              multi_part = Some(val);
            }
            _ => {
              multi_part = Some(val);
            }
          }
          Some(n) => {
            match current_operator {
              Some('+') => match multi_part {
                None => result = Some(n + val),
                Some(m) => multi_part = Some(m + val),
              }
              Some('*') => {
                if let Some(m) = multi_part {
                  result = Some(n * m);
                }
                multi_part = Some(val);
              }
              _ => {
                println!("new digit without operator")
              }
            };
            current_operator = None;
          }
        }
      },
      ')' => {
        if let Some(n) = multi_part {
          match result {
            None => result = Some(n),
            Some(m) => result = Some(n * m),
          }    
        }
        return result
      }
      _ => ()
    };
  }

  if let Some(n) = multi_part {
    match result {
      None => result = Some(n),
      Some(m) => result = Some(n * m),
    }
  }

  result
}
      
fn parse_input() -> Result<impl Iterator<Item=String>, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?.map(|l| l.unwrap());

  Ok(lines)
}
use crate::helpers;
use std::error::Error;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/day16";

type Rule = (u32, u32);

type Ticket = Vec<u32>;

pub fn solve() -> Result<(), Box<dyn Error>> {
  let (ticket_rules, my_ticket, tickets) = parse_input()?;
  solve_part_one(&ticket_rules, &tickets);

  solve_part_two(&ticket_rules, &my_ticket, &tickets);
  return Ok(());
}

fn solve_part_one(ticket_rules: &Vec<(Rule, Rule)>, tickets: &Vec<Ticket>) {
  let mut error_sum = 0;
  for ticket in tickets {
    for val in ticket {
      if !check_if_valid(ticket_rules, *val) {
        error_sum += *val;
      }
    }
  }

  println!("Part 1: {}", error_sum)
}

fn solve_part_two(ticket_rules: &Vec<(Rule, Rule)>, my_ticket: &Ticket, tickets: &Vec<Ticket>) {
  let valid_tickets: Vec<&Ticket> = tickets.iter().filter(|&t| {
    for val in t {
      if !check_if_valid(&ticket_rules, *val) {
        return false
      }
    }
    true
  }).chain(std::iter::once(my_ticket)).collect();

  let mut possible_rules: HashMap<usize, Vec<usize>> = HashMap::new();

  for i in 0..ticket_rules.len() {
    let (rule1, rule2) = ticket_rules[i];
    for j in 0..ticket_rules.len() {
      let mut rule_is_valid = true;
      for k in 0..valid_tickets.len() {
        let val = valid_tickets[k][j];
        if !check_rule(&rule1, val) && !check_rule(&rule2, val) {
          rule_is_valid = false;
          break;
        }
      }

      if rule_is_valid {
        let r = possible_rules.get_mut(&j);
        match r {
          None => { 
            let mut v: Vec<usize> = Vec::new();
            v.push(i);
            possible_rules.insert(j, v); 
          },
          Some(rules) => rules.push(i),
        };
      }
    }
  }

  let mut solution: HashMap<usize, usize> = HashMap::new();

  while solution.len() < ticket_rules.len() {
    for (&field, rules) in &possible_rules {
      let unused_rules = rules.iter().filter(|r| solution.get(r).is_none()).collect::<Vec<_>>();

      if unused_rules.len() == 1 {
        solution.insert(*unused_rules[0], field);
      }
    }
  }

  let mut departure_product: u64 = 1;
  for i in 0..6 {
    let field = solution.get(&i).unwrap();
    departure_product *= my_ticket[*field] as u64;
  }
  println!("Test : {:?}", departure_product);

}

fn check_if_valid(ticket_rules: &Vec<(Rule, Rule)>, val: u32) -> bool {
  for (rule1, rule2) in ticket_rules {
    if check_rule(rule1, val) || check_rule(rule2, val) {
      return true;
    }
  }

  false
}

fn check_rule(rule: &Rule, val: u32) -> bool {
  val >= rule.0 && val <= rule.1
}
      
fn parse_input() -> Result<(Vec<(Rule, Rule)>, Ticket, Vec<Ticket>), Box<dyn Error>> {
  let mut lines = helpers::read_lines(INPUT_FILE)?;

  let mut ticket_rules: Vec<(Rule, Rule)> = vec![];
  loop {
    let line = lines.next().unwrap()?;
    if line.trim().len() == 0 {
      break;
    }

    let rule_part = line.split(": ").skip(1).next().unwrap();
    let mut rules = rule_part.split(" or ");
    let rule1 = parse_rule(rules.next().unwrap())?;
    let rule2 = parse_rule(rules.next().unwrap())?;

    ticket_rules.push((rule1, rule2));
  }

  lines.next();

  let my_ticket = parse_ticket(lines.next().unwrap()?.as_str())?;

  lines.next();
  lines.next();

  let mut tickets = vec![];
  for line in lines {
    tickets.push(parse_ticket(line?.as_str())?);
  }

  Ok((ticket_rules, my_ticket, tickets))
}

fn parse_rule(rule_str: &str) -> Result<Rule, Box<dyn Error>> {
  let mut parts = rule_str.split('-');
  let lower = parts.next().unwrap().parse::<u32>()?;
  let upper = parts.next().unwrap().parse::<u32>()?;
  Ok((lower, upper))
}

fn parse_ticket(ticket_str: &str) -> Result<Ticket, Box<dyn Error>> {
  let numbers: Vec<u32> = ticket_str.split(',').map(|n| n.parse::<u32>().unwrap()).collect();

  Ok(numbers)
}
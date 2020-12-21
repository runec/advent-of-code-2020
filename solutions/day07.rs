use crate::helpers;
use std::error::Error;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/day07";

type BagRules = HashMap<String, Vec<(String, u32)>>;

pub fn solve() -> Result<(), Box<dyn Error>> {
  let rules = parse_input()?;
  let reversed_rules = get_reversed_rules(&rules);

  solve_part_one(&rules, &reversed_rules);
  solve_part_two(&rules);

  return Ok(());
}

fn solve_part_one(_rules: &BagRules, reversed_rules: &BagRules) {

  let base_rule = reversed_rules.get("shiny gold").unwrap();
  let mut valid_bags: Vec<String> = base_rule.iter().map(|(color, _count)| color.clone()).collect();
  let mut index = 0;
  loop {
    let mut new_bags: Vec<String> = vec![];
    for i in index..valid_bags.len() {
      let color = valid_bags.get(i).unwrap();
      let bags = reversed_rules.get(color);
      
      if let Some(bags) = bags {
        for (color, _count) in bags {
          if !valid_bags.contains(color) && !new_bags.contains(color) {
            new_bags.push(color.to_string());
          }
        }
      }
    }

    index = valid_bags.len();

    if new_bags.len() == 0 {
      break;
    } else {
      valid_bags.append(&mut new_bags);
    }
  }

  println!("Part 1 answer: {:?}", valid_bags.len());
}

fn solve_part_two(rules: &BagRules) {
  let result = get_number_of_bags_for_color(rules, &String::from("shiny gold"));
  println!("Part 2 answer: {}", result - 1);
}

fn get_number_of_bags_for_color(rules: &BagRules, color: &String) -> u32 {
  match rules.get(color) {
    None => 1,
    Some(v) => {
      let mut num = 1;
      for (color, count) in v {
        num += count * get_number_of_bags_for_color(rules, color)
      }
      num
    }
  }
}

fn get_reversed_rules (rules: &BagRules) -> BagRules {
  let mut reversed_rules: BagRules = HashMap::new();

  for container_color in rules.keys() {
    let containees = match rules.get(container_color) {
      None => vec![],
      Some(cs) => cs.clone()
    };

    for (containee_color, count) in containees.clone() {
      let current_rule = reversed_rules.get(&containee_color);
      
      if let Some(v) = current_rule {

        let mut new_rule = v.clone();
        new_rule.push((container_color.to_string(), count));
        reversed_rules.insert(containee_color, new_rule);
      } else {
        reversed_rules.insert(containee_color, vec![(container_color.to_string(), count)]);
      }
    }
  }

  return reversed_rules;
}

fn parse_input() -> Result<BagRules, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?;

  let mut bag_rules: BagRules = HashMap::new();

  for line in lines {
    let line = line?;
    let mut parts = line.split(" contain ");
    let container_part = parts.next().unwrap().to_string();
    let container_color = parse_color_of_bag(container_part);

    let mut containees: Vec<(String, u32)> = vec![];

    let contained_part = parts.next().unwrap().to_string();
    let contained_parts = contained_part.split(", ");

    for p in contained_parts {
      let mut parts = p.splitn(2, " ");
      let count = parts.next().unwrap().parse::<u32>();
      if let Ok(num) = count {
        let color = parse_color_of_bag(parts.next().unwrap().to_string());
        containees.push((color, num));
      }
    }

    if bag_rules.get(&container_color).is_some() {
      println!("Color already existed. Handle this? ({})", container_color);
    }

    bag_rules.insert(container_color, containees);
  }

  return Ok(bag_rules);
}

fn parse_color_of_bag(bag_string: String) -> String {
  let index = bag_string.rfind("bag").unwrap();
  bag_string[..index-1].to_string()
}
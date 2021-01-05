use crate::helpers;
use std::error::Error;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/day19";

enum Rule {
  Literal(String),
  Sequence(Vec<usize>),
  Sub(Vec<usize>, Vec<usize>),
}

type RuleMap = HashMap<usize, Rule>;

pub fn solve() -> Result<(), Box<dyn Error>> {
  let mut lines = parse_input()?;
  let mut rules = parse_rules(&mut lines)?;
  solve_part_one(lines, &rules);

  rules.insert(8, Rule::Sub(vec![42], vec![42, 8]));
  rules.insert(11, Rule::Sub(vec![42, 31], vec![42, 11, 31]));
  let lines = parse_input()?;
  solve_part_one(lines, &rules);

  return Ok(());
}

fn solve_part_one(lines: impl Iterator<Item=String>, rules: &RuleMap) {
  let mut matches = 0;
  for line in lines {
    // check if line fits
    let r = test_rule(rules, line.as_str(), 0);
    let line_len = line.len();
    if r.contains(&line_len) {
      matches += 1;
    }
  }

  println!("Part 1: {}", matches);
}

fn test_rule(rule_map: &RuleMap, txt: &str, rule_num: usize) -> Vec<usize> {
  let rule = rule_map.get(&rule_num).unwrap();

  match rule {
    Rule::Literal(s) => {
      if txt.starts_with(s) {
        vec![s.len()]
      } else {
        vec![]
      }
    },
    Rule::Sequence(ss) => test_sequence(rule_map, txt, ss),
    Rule::Sub(ss1, ss2) => {
      let mut res1 = test_sequence(rule_map, txt, ss1);
      let mut res2 = test_sequence(rule_map, txt, ss2);
      res1.append(&mut res2);
      res1
    }
  }
}

fn test_sequence(rule_map: &RuleMap, txt: &str, rule_seq: &Vec<usize>) -> Vec<usize> {
  rule_seq.iter().fold(vec![0], |res, &r| {
    let mut new_res = vec![];
    for n in res {
      let test_res = test_rule(rule_map, &txt[n..], r);
      for m in test_res {
        new_res.push(n + m);
      }
    };
    new_res
  })
}
      
fn parse_input() -> Result<impl Iterator<Item=String>, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?.map(|l| l.unwrap());

  Ok(lines)
}

fn parse_rules(lines: &mut impl Iterator<Item=String>) -> Result<RuleMap, Box<dyn Error>> {
  let mut rule_map: RuleMap = HashMap::new();

  while let Some(l) = lines.next() {
    if l.trim().len() == 0 {
      break;
    }
    let mut parts = l.split(": ");
    let rule_num = parts.next().unwrap().parse::<usize>()?;
    let parts = parts.next().unwrap().split(" | ").collect::<Vec<_>>();

    if parts.len() > 1 {
      let rule = Rule::Sub(parse_sequence(parts[0]), parse_sequence(parts[1]));
      rule_map.insert(rule_num, rule);
    } else if parts[0].contains('"') {
      let rule = Rule::Literal(parts[0].chars().filter(|&c| c != '"').collect());
      rule_map.insert(rule_num, rule);
    } else {
      let rule = Rule::Sequence(parse_sequence(parts[0]));
      rule_map.insert(rule_num, rule);
    }
  }

  Ok(rule_map)
}

fn parse_sequence(part: &str) -> Vec<usize> {
  part
    .split_whitespace()
    .map(|s| s.parse::<usize>().unwrap())
    .collect()
}
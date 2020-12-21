use crate::helpers;
use std::error::Error;
use std::collections::HashSet;

const INPUT_FILE: &str = "input/day06";

type Group = Vec<String>;

pub fn solve() -> Result<(), Box<dyn Error>> {
  let groups = parse_input()?;

  solve_part_one(&groups);
  solve_part_two(&groups);

  return Ok(());
}

fn solve_part_one(groups: &Vec<Group>) {

  let result: u32 = groups.iter().map(|g| any_in_group_answers_yes(g).len() as u32).sum();
  println!("Part 1 answer: {}", result);
}

fn solve_part_two(groups: &Vec<Group>) {

  let result: u32 = groups.iter().map(|g| all_in_group_answer_yes(g).len() as u32).sum();
  println!("Part 2 answer: {}", result);
}

fn any_in_group_answers_yes (group: &Group) -> Vec<char> {
  let mut all_answers = HashSet::new();
  for person_answers in group {
    for answer in person_answers.chars() {
      all_answers.insert(answer);
    }
  }
  all_answers.into_iter().collect()
}

fn all_in_group_answer_yes (group: &Group) -> Vec<char> {
  let mut all_answers: Option<Vec<char>> = None; // = group[0].chars().collect();

  for person_answers in group {
    all_answers = match all_answers {
      None => Some(person_answers.chars().collect()),
      Some(answers) => Some(Vec::into_iter(answers).filter(|&a| person_answers.chars().any(|pa| pa == a)).collect())
    }
  }

  match all_answers {
    None => vec![],
    Some(a) => a
  }
}

fn parse_input() -> Result<Vec<Group>, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?;

  let mut groups: Vec<Group> = vec![]; 
  let mut current_entry: Group = vec![];

  for line in lines {
    let line = line?;
    if line.trim().len() == 0 {
      groups.push(current_entry);
      current_entry = vec![];
    } else {
      current_entry.push(line);
    }
  }

  if !current_entry.is_empty() {
    groups.push(current_entry);
  }

  return Ok(groups);
}
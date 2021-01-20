use crate::helpers;
use std::error::Error;
use std::collections::VecDeque;
use std::collections::HashSet;

const INPUT_FILE: &str = "input/day22";

type GameState = (VecDeque<u32>, VecDeque<u32>);

pub fn solve() -> Result<(), Box<dyn Error>> {
  let mut initial_state = parse_input()?;
  solve_part_one(&mut initial_state);
  let mut initial_state = parse_input()?;
  solve_part_two(&mut initial_state);

  /*
  let test_border = border(&(&tiles[0], Direction::N, false), Direction::E);
  let s = test_border.iter().map(|&p| if p {'#'} else {'.'}).collect::<String>();
  println!("Test {}", s);
  */
  return Ok(());
}

fn solve_part_one(state: &mut GameState) {

  while state.0.len() > 0 && state.1.len() > 0 {
    run_one_round(state);
  }

  let score = if state.0.len() == 0 {
    calculate_score(&state.1)
  } else {
    calculate_score(&state.0)
  };

  println!("Part 1: {}", score);
}

fn solve_part_two(state: &mut GameState) {
  run_recursive_game(state);

  let score = if state.0.len() == 0 {
    calculate_score(&state.1)
  } else {
    calculate_score(&state.0)
  };

  println!("Part 2: {}", score);
}

fn calculate_score(final_deck: &VecDeque<u32>) -> u64 {
  final_deck.iter().rev().enumerate().fold(0, |score, (i, &val)| score + ((i + 1) as u64 * val as u64))
}

fn run_one_round(state: &mut GameState) {
  let (p1, p2) = state;
  let p1_card = p1.pop_front().unwrap();
  let p2_card = p2.pop_front().unwrap();

  if p1_card > p2_card {
    p1.push_back(p1_card);
    p1.push_back(p2_card);
  } else {
    p2.push_back(p2_card);
    p2.push_back(p1_card);
  }
}

fn run_recursive_game(state: &mut GameState) -> bool {
  let mut state_hashes = HashSet::new();

  loop {
    if state.0.len() == 0 {return false};
    if state.1.len() == 0 {return true};
    let state_hash = get_state_hash(state);
    if state_hashes.contains(&state_hash) {
      return true;
    } else {
      state_hashes.insert(state_hash);
    }
    run_recursive_round(state);
  }

}

fn run_recursive_round(state: &mut GameState) -> bool {
  let (p1, p2) = state;
  let p1_card = p1.pop_front().unwrap();
  let p2_card = p2.pop_front().unwrap();

  let mut p1_wins = false;

  if p1.len() as u32 >= p1_card && p2.len() as u32 >= p2_card {
    // recursive round decides winner
    let p1_sub = copy_front(p1, p1_card);
    let p2_sub = copy_front(p2, p2_card);
    let mut sub_state = (p1_sub, p2_sub);
    let p1_wins_sub = run_recursive_game(&mut sub_state);
    p1_wins = p1_wins_sub;
  } else if p1_card > p2_card {
    p1_wins = true;
  }

  if p1_wins {
    p1.push_back(p1_card);
    p1.push_back(p2_card);
  } else {
    p2.push_back(p2_card);
    p2.push_back(p1_card);
  }

  false
}

fn get_state_hash(state: &GameState) -> String {
  let (p1, p2) = state;
  let mut hash: String = String::new();

  for val in p1 {
    hash.push_str(format!("{},", val).as_str());
  }
  hash.push_str("-");
  for val in p2 {
    hash.push_str(format!("{},", val).as_str());
  }

  hash
}

fn copy_front(list: &VecDeque<u32>, n: u32) -> VecDeque<u32> {
  let mut result = VecDeque::new();
  let mut iter = list.iter();
  for _ in 0..n {
    result.push_back(*iter.next().unwrap());
  }
  result
}

fn parse_input() -> Result<GameState, Box<dyn Error>> {
  let mut lines = helpers::read_lines(INPUT_FILE)?.map(|l| l.unwrap());

  let mut player1 = VecDeque::new();
  let mut player2 = VecDeque::new();

  let mut player1_done = false;

  while let Some(l) = lines.next() {
    if l.trim().len() == 0 {
      player1_done = true;
    } else {
      match l.parse::<u32>() {
        Ok(val) => if !player1_done {
          player1.push_back(val)
        } else {
          player2.push_back(val)
        }
        Err(_) => ()
      }
    }
  }

  Ok((player1, player2))
}
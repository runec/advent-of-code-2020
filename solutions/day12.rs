use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day12";

enum Direction {
  N,
  E,
  S,
  W,
}

impl Direction {
  fn from_degrees(degrees: i32) -> Direction {
    match degrees.rem_euclid(360) {
      0 => Direction::N,
      90 => Direction::E,
      180 => Direction::S,
      270 => Direction::W,
      _ => {
        println!("Unknown direction! {}", degrees);
        Direction::N
      }
    }
  }

  fn in_degrees(&self) -> i32 {
    match self {
      Direction::N => 0,
      Direction::E => 90,
      Direction::S => 180,
      Direction::W => 270,
    }
  }

  fn as_vector(&self) -> (i32, i32) {
    match self {
      Direction::N => (0, 1),
      Direction::E => (1, 0),
      Direction::S => (0, -1),
      Direction::W => (-1, 0),
    }
  }

  fn rotate_left (&self, degrees: i32) -> Direction {
    Direction::from_degrees(self.in_degrees() - degrees)
  }

  fn rotate_right (&self, degrees: i32) -> Direction {
    Direction::from_degrees(self.in_degrees() + degrees)
  }
}

enum Action {
  N(i32),
  E(i32),
  S(i32),
  W(i32),
  L(i32),
  R(i32),
  F(i32),
}

type ShipState = (i32, i32, Direction);

type ShipWaypointState = (i32, i32, i32, i32);

pub fn solve() -> Result<(), Box<dyn Error>> {
  let actions = parse_input()?;
  solve_part_one(&actions);
  solve_part_two(&actions);
  return Ok(());
}

fn solve_part_one(actions: &Vec<Action>) {
  let mut ship = (0, 0, Direction::E);

  for action in actions {
    ship = perform_action(ship, action);
  }

  println!("Part 1: {}", ship.0.abs() + ship.1.abs());
}

fn solve_part_two(actions: &Vec<Action>) {
  let mut ship_waypoint = (10, 1, 0, 0);

  for action in actions {
    ship_waypoint = perform_waypoint_action(ship_waypoint, action);
    println!(":::: ({}, {}) -- ({}, {})", ship_waypoint.0, ship_waypoint.1, ship_waypoint.2, ship_waypoint.3);

  }

  println!("Part 2: {}", ship_waypoint.2.abs() + ship_waypoint.3.abs());
}

fn perform_action ((x, y, dir): ShipState, action: &Action) -> ShipState {
  match action {
    Action::N(val) => (x, y + *val, dir),
    Action::E(val) => (x + *val, y, dir),
    Action::S(val) => (x, y - *val, dir),
    Action::W(val) => (x - *val, y, dir),
    Action::L(val) => (x, y, dir.rotate_left(*val)),
    Action::R(val) => (x, y, dir.rotate_right(*val)),
    Action::F(val) => {
      let (dx, dy) = dir.as_vector();
      (x + dx * *val, y + dy * *val, dir)
    }
  }
}

fn perform_waypoint_action ((x_w, y_w, x_s, y_s): ShipWaypointState, action: &Action) -> ShipWaypointState {
  match action {
    Action::N(val) => (x_w, y_w + *val, x_s, y_s),
    Action::E(val) => (x_w + *val, y_w, x_s, y_s),
    Action::S(val) => (x_w, y_w - *val, x_s, y_s),
    Action::W(val) => (x_w - *val, y_w, x_s, y_s),
    Action::L(val) => rotate_waypoint_left((x_w, y_w, x_s, y_s), *val),
    Action::R(val) => rotate_waypoint_left((x_w, y_w, x_s, y_s), -*val),
    Action::F(val) => {
      let (dx, dy) = (x_w - x_s, y_w - y_s);
      (x_w + dx * *val, y_w + dy * *val, x_s + dx * *val, y_s + dy * *val)
    }
  }
}

fn rotate_waypoint_left ((x_w, y_w, x_s, y_s): ShipWaypointState, degrees: i32) -> ShipWaypointState {
  let mut new_pos = (x_w - x_s, y_w - y_s);
 
  new_pos = match degrees.rem_euclid(360) {
    0 => (new_pos.0, new_pos.1),
    90 => (-new_pos.1, new_pos.0),
    180 => (-new_pos.0, -new_pos.1),
    270 => (new_pos.1, -new_pos.0),
    _ => (new_pos.0, new_pos.1)
  };

  new_pos = (new_pos.0 + x_s, new_pos.1 + y_s);
  (new_pos.0, new_pos.1, x_s, y_s)

}

fn parse_input() -> Result<Vec<Action>, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?;
  let mut actions: Vec<Action> = vec![];
  
  for line in lines {
    let line = line?;
    let (command, val) = line.split_at(1);
    let val = val.parse::<i32>()?;

    match command {
      "N" => actions.push(Action::N(val)),
      "E" => actions.push(Action::E(val)),
      "S" => actions.push(Action::S(val)),
      "W" => actions.push(Action::W(val)),
      "L" => actions.push(Action::L(val)),
      "R" => actions.push(Action::R(val)),
      "F" => actions.push(Action::F(val)),
      _ => ()
    }
  }

  Ok(actions)
}

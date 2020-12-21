use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day11";

#[derive(Clone, PartialEq)]
enum SeatState {
  Floor,
  Empty,
  Occupied
}

type Seat = (usize, usize);

type Seating = Vec<Vec<SeatState>>;

pub fn solve() -> Result<(), Box<dyn Error>> {
  let initial_seating = parse_input()?;
  solve_part_one(&initial_seating);
  solve_part_two(&initial_seating);
  return Ok(());
}

fn solve_part_one(initial_seating: &Seating) {
  let mut seating = (*initial_seating).clone();

  let mut seating_changed = true;
  while seating_changed {
    seating_changed = run_one_round(&mut seating, 4, get_occupied_neighbours);
  }

  let mut occupied_seats = 0;
  for seat_row in seating {
    for seat in seat_row {
      if seat == SeatState::Occupied {
        occupied_seats += 1;
      }
    }
  }

  println!("Part 1: {}", occupied_seats);
}

fn solve_part_two(initial_seating: &Seating) {
  let mut seating = (*initial_seating).clone();

  let mut seating_changed = true;
  while seating_changed {
    seating_changed = run_one_round(&mut seating, 5, get_occupied_in_each_direction);
  }

  let mut occupied_seats = 0;
  for seat_row in seating {
    for seat in seat_row {
      if seat == SeatState::Occupied {
        occupied_seats += 1;
      }
    }
  }

  println!("Part 2: {}", occupied_seats);
}

fn run_one_round<F> (seating: &mut Seating, occupied_limit: u32, neighbour_func: F) -> bool 
where F: Fn(&Seating, Seat) -> u32 {
  let mut seating_changed = false;
  let mut seats_to_update: Vec<Seat> = vec![];
  for i in 0..seating.len() {
    for j in 0..seating.get(i).unwrap().len() {
      if seating[i][j] != SeatState::Floor {
        let occupied_neighbours = neighbour_func(seating, (i, j));

        if occupied_neighbours >= occupied_limit && seating[i][j] != SeatState::Empty {
          seats_to_update.push((i,j));
          seating_changed = true;
        } else if occupied_neighbours == 0 && seating[i][j] != SeatState::Occupied {
          seats_to_update.push((i,j));
          seating_changed = true;
        }
      }
    }
  }

  for (r, c) in seats_to_update {
    seating[r][c] = if seating[r][c] == SeatState::Occupied {
      SeatState::Empty
    } else {
      SeatState::Occupied
    }
  }

  seating_changed
}

fn get_occupied_neighbours (seating: &Seating, (row, col): Seat) -> u32 {
  let row_max = seating.len() - 1;
  let col_max = seating.get(0).unwrap().len() - 1;
  let rows = if row == 0 {
    0..=1
  } else if row == row_max {
    (row_max-1)..=row_max
  } else {
    (row-1)..=(row+1)
  };
  let cols = if col == 0 {
    0..=1
  } else if col == col_max {
    (col_max-1)..=col_max
  } else {
    (col-1)..=(col+1)
  };

  let mut occupied_seats = 0;
  for r in rows {
    for c in cols.clone() {
      if !(r == row && c == col) && seating[r][c] == SeatState::Occupied {
        occupied_seats += 1;
      }
    }
  }

  occupied_seats
} 

fn get_occupied_in_each_direction(seating: &Seating, (row, col): Seat) -> u32 {
  let directions: Vec<(i32, i32)> = vec![(-1, 1), (0, 1), (1,1), (-1, 0), (1,0), (-1,-1), (0,-1), (1,-1)];

  let num_rows = seating.len() as i32;
  let num_cols = seating.get(0).unwrap().len() as i32;

  let mut occupied_seats = 0;
  for dir in directions {
    let (mut r, mut c) = (row as i32 + dir.0, col as i32 + dir.1);
    while r >= 0 && r < num_rows && c >= 0 && c < num_cols {
      let seat = seating.get(r as usize).unwrap().get(c as usize).unwrap();
      match seat {
        SeatState::Floor => {
          r = r + dir.0;
          c = c + dir.1;
        }
        SeatState::Empty => {
          break;
        },
        SeatState::Occupied => {
          occupied_seats += 1;
          break;
        }


      }
    }   
  }
  
  occupied_seats
}


fn parse_input() -> Result<Seating, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?;
  let mut seating: Seating = vec![];
  
  for line in lines {
    let seating_row = line?.chars().map(|c| match c {
      'L' => SeatState::Empty,
      '#' => SeatState::Occupied,
      _ => SeatState::Floor
    }).collect::<Vec<_>>();

    seating.push(seating_row);
  }

  Ok(seating)
}

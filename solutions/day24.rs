use crate::helpers;
use std::error::Error;
use std::collections::HashSet;

const INPUT_FILE: &str = "input/day24";

enum Direction {
  E,
  NW,
  NE,
  W,
  SW,
  SE,
}

type HexCoord = (i64, i64);

fn hex_move(&(x, y): &HexCoord, dir: &Direction) -> HexCoord {
  match dir {
    Direction::E => (x + 1, y),
    Direction::NE => (x, y + 1),
    Direction::NW => (x - 1, y + 1),
    Direction::W => (x - 1, y),
    Direction::SW => (x, y - 1),
    Direction::SE => (x + 1, y - 1)
  }
}

pub fn solve() -> Result<(), Box<dyn Error>> {
  let tile_directions = parse_input()?;
  let mut flipped_tiles = solve_part_one(&tile_directions);
  solve_part_two(&mut flipped_tiles);
  return Ok(());
}

fn solve_part_one(tile_directions: &Vec<Vec<Direction>>) -> HashSet<(i64, i64)> {
  let mut flipped_tiles = HashSet::new();

  for directions in tile_directions {
    let mut tile: HexCoord = (0,0);
    for dir in directions {
      tile = hex_move(&tile, dir);
    }

    if flipped_tiles.get(&tile).is_none() {
      flipped_tiles.insert(tile);
    } else {
      flipped_tiles.remove(&tile);
    }
  }
  println!("Part 1: {}, {:?}", flipped_tiles.len(), flipped_tiles);

  flipped_tiles
}

fn solve_part_two(flipped_tiles: &mut HashSet<(i64, i64)>) {
  for _ in 0..100 {
    get_tiles_next_day(flipped_tiles);
  }

  println!("Part 2: {}", flipped_tiles.len());
}

fn get_tiles_next_day(flipped_tiles: &mut HashSet<(i64, i64)>) {
  let mut tiles_to_flip = HashSet::new();
  let dirs = [Direction::E, Direction::NE, Direction::NW, Direction::W, Direction::SW, Direction::SE];

  for flipped_tile in flipped_tiles.iter() {
    let blacks = get_black_neighbours(flipped_tiles, flipped_tile);
    if blacks == 0 || blacks > 2 {
      tiles_to_flip.insert(*flipped_tile);
    }
    for dir in dirs.iter() {
      let neighbour = hex_move(flipped_tile, dir);
      if flipped_tiles.get(&neighbour).is_none() {
        let blacks = get_black_neighbours(flipped_tiles, &neighbour);
        if blacks == 2 {
          tiles_to_flip.insert(neighbour);
        }
      }
    }
  }

  for tile in tiles_to_flip {
    if flipped_tiles.get(&tile).is_none() {
      flipped_tiles.insert(tile);
    } else {
      flipped_tiles.remove(&tile);
    }
  }
}

fn get_black_neighbours(flipped_tiles: &HashSet<(i64, i64)>, coord: &HexCoord) -> i32 {
  let dirs = [Direction::E, Direction::NE, Direction::NW, Direction::W, Direction::SW, Direction::SE];

  let mut count_black = 0;
  for dir in dirs.iter() {
    let neighbour = hex_move(coord, dir);
    if flipped_tiles.get(&neighbour).is_some() {
      count_black += 1;
    }
  }

  count_black
}

fn parse_input() -> Result<Vec<Vec<Direction>>, Box<dyn Error>> {
  let mut result = vec![];
  let lines = helpers::read_lines(INPUT_FILE)?.map(|l| l.unwrap());
  for line in lines {
    let mut directions = vec![];
    let mut prev_char = None;
    for c in line.chars() {
      match c {
        'n' => prev_char = Some('n'),
        's' => prev_char = Some('s'),
        'w' => {
          match prev_char {
            Some('n') => directions.push(Direction::NW),
            Some('s') => directions.push(Direction::SW),
            _ => directions.push(Direction::W),
          };
          prev_char = None;
        },
        'e' => {
          match prev_char {
            Some('n') => directions.push(Direction::NE),
            Some('s') => directions.push(Direction::SE),
            _ => directions.push(Direction::E),
          };
          prev_char = None;
        }
        _ => ()
      }
    }
    result.push(directions);
  }
  Ok(result)
}

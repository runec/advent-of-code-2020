use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day20";
const SEAMONSTER_FILE: &str = "input/day20_seamonster";


type Tile = (u32, Vec<Vec<bool>>);

#[derive(Clone, PartialEq, Copy, Debug)]
enum Direction {
  N,
  E,
  S,
  W
}

type OrientedTile<'a> = (&'a Tile, Direction, bool);

type Coord = (usize, usize);

type PartialImage<'a> = Vec<Vec<Option<OrientedTile<'a>>>>;

type CompletedImage = Vec<Vec<bool>>;

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

  fn rotate_right (&self, degrees: i32) -> Direction {
    Direction::from_degrees(self.in_degrees() + degrees)
  }

  fn mirror (&self) -> Direction {
    match self {
      Direction::N => Direction::N,
      Direction::E => Direction::W,
      Direction::S => Direction::S,
      Direction::W => Direction::E,
    }
  }
}

fn border((tile, orientation, mirrored): &OrientedTile, direction: Direction) -> Vec<bool> {
  let rotated_dir = direction.rotate_right(orientation.in_degrees());
  let rotated_dir = if *mirrored {rotated_dir.mirror()} else {rotated_dir};

  let x_max = tile.1.len();
  let y_max = tile.1[0].len();

  let (mut xs, mut ys) = match rotated_dir {
    Direction::N => (vec![0], (0..y_max).collect()),
    Direction::E => ((0..x_max).collect(), vec![y_max-1]),
    Direction::S => (vec![x_max-1], (0..y_max).collect()),
    Direction::W => ((0..x_max).collect(), vec![0]),
  };

  if rotated_dir == Direction::S || rotated_dir == Direction::W {
    xs.reverse();
    ys.reverse();
  }

  if *mirrored {
    xs.reverse();
    ys.reverse();
  }

  let mut border = vec![];

  for &x in &xs {
    for &y in &ys {
      border.push(tile.1[x][y]);
    }
  }

  border
}

pub fn solve() -> Result<(), Box<dyn Error>> {
  let tiles = parse_input()?;
  let partial_image = solve_part_one(&tiles);
  let completed = convert_partial_image(&partial_image);
  let seamonster = parse_seamonster()?;
  solve_part_two(&completed, &seamonster);
  /*
  let test_border = border(&(&tiles[0], Direction::N, false), Direction::E);
  let s = test_border.iter().map(|&p| if p {'#'} else {'.'}).collect::<String>();
  println!("Test {}", s);
  */
  return Ok(());
}

fn solve_part_one(tiles: &Vec<Tile>) -> PartialImage {
  let directions: Vec<Direction> = vec![
    Direction::N,
    Direction::E,
    Direction::S,
    Direction::W,
  ];
  let side_length = (tiles.len() as f64).sqrt().round() as usize;
  let mut image: PartialImage = vec![];
  for _ in 0..side_length {
    let mut row = vec![];
    for _ in 0..side_length {
      row.push(None);
    }
    image.push(row);
  }

  let mut used_tiles: Vec<usize> = vec![];

  let mut i = 0;
  let mut j = 0;
  let mut d = 0;

  let mut m = false;

  'outer: while i < tiles.len() {
    let (x, y) = (i / side_length, i.rem_euclid(side_length));

    while j < tiles.len() {
      if !used_tiles.contains(&j) {
        let tile = &tiles[j];
        while d < directions.len() {
          let dir = directions[d];
          let placed_tile = (tile, dir, m);
          let result = test_tile(&image, (tile, dir, m), (x, y));

          if result {
            image[x][y] = Some(placed_tile);
            used_tiles.push(j);
            j = 0;
            d = 0;
            m = false;
            i += 1;
            continue 'outer;
          }
          if m {
            m = false;
            d += 1;
          } else {
            m = true;
          }
        }
      }

      j += 1;
      d = 0;
    }



    if j == tiles.len() {
      i -= 1;
      let (old_x, old_y) = (i / side_length, i.rem_euclid(side_length));
      let failed_tile = image[old_x][old_y].unwrap();
      let old_index = used_tiles.pop().unwrap();

      j = old_index;
      
      d = match failed_tile.1 {
        Direction::N => 0,
        Direction::E => 1,
        Direction::S => 2,
        Direction::W => 3,
      };
      m = failed_tile.2;

      if !m {
        m = true
      } else if d < 3 {
        m = false;
        d += 1;
      } else {
        m = false;
        d = 0;
        j += 1;
      }

      image[old_x][old_y] = None;
      
    } else {
      println!("Should not happen");
    }
  }

  println!("Part 1: {}", 
    (image[0][0].unwrap().0.0 as u64) *
    (image[side_length - 1][0].unwrap().0.0 as u64) *
    (image[0][side_length-1].unwrap().0.0 as u64) *
    (image[side_length-1][side_length-1].unwrap().0.0 as u64)
  );

  image
}

fn solve_part_two(image: &CompletedImage, monster: &CompletedImage) {
  let directions: Vec<Direction> = vec![
    Direction::N,
    Direction::E,
    Direction::S,
    Direction::W,
  ];


  let mut seamonsters = 0;
  for d in directions {
    for m in vec![false, true] {
      let mut positions = vec![];
      for i in 0..image.len() {
        for j in 0..image[0].len() {
          let found = check_for_monster(image, monster, (i, j), d, m);
          if found {
            positions.push((i,j));
          }
        }
      }
      if positions.len() > 0 {
        println!("Found seamonsters at {:?} {}: {:?}", d, m, positions);
        seamonsters = positions.len();
        break;
      } else {
        println!("Found no seamonsters at {:?} {}", d, m);
      }
    }
  }

  let seamonster_size = monster.iter().fold(0, |res, r| res + r.iter().fold(0, |acc, &b| acc + if b {1} else {0}));
  let image_size = image.iter().fold(0, |res, r| res + r.iter().fold(0, |acc, &b| acc + if b {1} else {0}));

  println!("Part 2: {}", image_size - seamonsters * seamonster_size);

}

fn check_for_monster(image: &CompletedImage, monster: &CompletedImage, (x, y): Coord, dir: Direction, m: bool) -> bool {

  match dir {
    Direction::N | Direction::S => if x + monster.len() > image.len() || y + monster[0].len() > image[0].len() {return false}
    Direction::E | Direction::W => if x + monster[0].len() > image.len() || y + monster.len() > image[0].len() {return false}
  }

  for i in 0..monster.len() {
    // println!("");
    for j in 0..monster[i].len() {
      //print!("{}", if monster[i][j] {"#"} else {"."});

      let (mut image_x, mut image_y) = match dir {
        Direction::N => (i, j),
        Direction::E => (j, monster.len() - i - 1),
        Direction::S => (monster.len() - i - 1, monster[0].len() - j - 1),
        Direction::W => (monster[0].len() - j- 1, i)
      };
      if m {
        image_y = match dir {
          Direction::N | Direction::S => monster[0].len() - image_y - 1,
          Direction::E | Direction::W => monster.len() - image_y - 1,
        }
      }
      
      image_x = match dir {
        Direction::N | Direction::S => monster.len() - image_x - 1,
        Direction::E | Direction::W => monster[0].len() - image_x - 1,
      };

      image_x += x;
      image_y += y;

      if !image[image_x][image_y] && monster[i][j] {
        return false
      }
    }
  }

  true
}

fn test_tile(image: &PartialImage, tile: OrientedTile, (x, y): Coord) -> bool {
  let max = image.len() - 1;
  let directions: Vec<Direction> = vec![
    Direction::N,
    Direction::E,
    Direction::S,
    Direction::W,
  ];

  for dir in directions {
    let neighbour;
    match dir {
      Direction::N => if x == max {continue} else {neighbour = (x+1, y)},
      Direction::E => if y == max {continue} else {neighbour = (x, y+1)},
      Direction::S => if x == 0 {continue} else {neighbour = (x-1, y)},
      Direction::W => if y == 0 {continue} else {neighbour = (x, y-1)},
    }


    if let Some(neighbour_tile) = &image[neighbour.0][neighbour.1] {
      let neighbour_border = border(&neighbour_tile, dir.rotate_right(180));
      let this_border = border(&tile, dir);
      if !neighbour_border.iter().eq(this_border.iter().rev()) {
        return false;
      }
    }
  }

  true
} 

fn convert_partial_image (image: &PartialImage) -> CompletedImage {
  let mut completed = vec![];
  let tile_side = image[0][0].unwrap().0.1.len();
  let completed_tile_side = tile_side - 2;
  let completed_side = image.len() * completed_tile_side;

  for _ in 0..completed_side {
    completed.push(vec![false; completed_side]);
  }

  for x in 0..image.len() {
    for y in 0..image[x].len() {
      let (tile, dir, m) = image[x][y].unwrap();
      for i in 1..tile_side-1 {
        for j in 1..tile_side-1 {

          let mut tile_coords = match dir {
            Direction::N => (i, j),
            Direction::E => (tile_side - j - 1, i),
            Direction::S => (tile_side - i - 1, tile_side - j - 1),
            Direction::W => (j, tile_side - i - 1),
          };
          if m {
            tile_coords = (tile_coords.0, tile_side - tile_coords.1 - 1);
          }
          tile_coords = (tile_side - tile_coords.0 - 1, tile_coords.1);
          
          let completed_coords = (completed_tile_side * x + i - 1, completed_tile_side * y + j - 1);
          completed[completed_coords.0][completed_coords.1] = tile.1[tile_coords.0][tile_coords.1];
        }
      }
    }
  }

  for r in &completed {
    for c in r {
      print!("{}", if *c {"#"} else {"."});
    }
    println!("");
  }
  completed
}


fn print_solution_1 (image: &PartialImage) {
  let mut completed = vec![];
  let tile_side = image[0][0].unwrap().0.1.len();
  let completed_tile_side = tile_side;
  let completed_side = image.len() * completed_tile_side;

  for _ in 0..completed_side {
    completed.push(vec![false; completed_side]);
  }

  for x in 0..image.len() {
    for y in 0..image[x].len() {
      let (tile, dir, m) = image[x][y].unwrap();
      if x == 2 && y == 2 {
        println!("Test {} {:?} {}", m, dir, tile.0);
      }
      for i in 0..tile_side {
        for j in 0..tile_side {

          let mut tile_coords = match dir {
            Direction::N => (i, j),
            Direction::E => (tile_side - j - 1, i),
            Direction::S => (tile_side - i - 1, tile_side - j - 1),
            Direction::W => (j, tile_side - i - 1),
          };
          if m {
            tile_coords = (tile_coords.0, tile_side - tile_coords.1 - 1);
          }
          tile_coords = (tile_side - tile_coords.0 - 1, tile_coords.1);
          
          let completed_coords = (completed_tile_side * x + i, completed_tile_side * y + j);
          completed[completed_coords.0][completed_coords.1] = tile.1[tile_coords.0][tile_coords.1];
        }
      }
    }
  }

  for r in &completed {
    for c in r {
      print!("{}", if *c {"#"} else {"."});
    }
    println!("");
  }
}
      
fn parse_input() -> Result<Vec<Tile>, Box<dyn Error>> {
  let mut lines = helpers::read_lines(INPUT_FILE)?.map(|l| l.unwrap()).peekable();

  let mut tiles = vec![];
  while let Some(_) = lines.peek() {
    tiles.push(parse_tile(&mut lines)?)
  }

  Ok(tiles)
}

fn parse_seamonster() -> Result<CompletedImage, Box<dyn Error>> {
  let mut lines = helpers::read_lines(SEAMONSTER_FILE)?.map(|l| l.unwrap());

  let mut monster = vec![];
  for line in lines {
    monster.push(line.chars().map(|c| c == '#').collect());
  }

  Ok(monster)
}

fn parse_tile(lines: &mut impl Iterator<Item=String>) -> Result<Tile, Box<dyn Error>> {
  // First: ID
  // Then: tile rows
  let id_line = lines.next().unwrap();
  let id = id_line.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u32>()?;

  let mut tile_rows: Vec<Vec<bool>> = vec![];

  while let Some(l) = lines.next() {
    if l.trim().len() == 0 {
      break;
    }

    let tile_points: Vec<bool> = l.chars().map(|c| match c {
      '#' => true,
      _ => false
    }).collect();

    tile_rows.push(tile_points);
  }

  Ok((id, tile_rows))
}
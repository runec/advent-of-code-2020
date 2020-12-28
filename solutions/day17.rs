use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day17";

const CYCLES: usize = 6;

type Source = Vec<Vec<Vec<bool>>>;
type Source4D = Vec<Source>;

type Point = (usize, usize, usize);

type Point4D = (usize, usize, usize, usize);

pub fn solve() -> Result<(), Box<dyn Error>> {
  let source = parse_input()?;
  solve_part_one(&source);

  let source_4d: Source4D = init_source_4d(&source);
  solve_part_two(&source_4d);
  return Ok(());
}

fn solve_part_one(init_source: &Source) {

  let mut source = init_source.clone();

  for _ in 0..CYCLES {
    run_cycle(&mut source);
  }

  let mut active_sum = 0;
  for i in 0..source.len() {
    for j in 0..source[i].len() {
      for k in 0..source[i][j].len() {
        if source[i][j][k] {
          active_sum += 1;
        }
      }
    }
  }

  println!("Part 1: {}", active_sum);
}

fn solve_part_two(init_source: &Source4D) {

  let mut source = init_source.clone();

  for _ in 0..CYCLES {
    run_cycle_4d(&mut source);
  }

  let mut active_sum = 0;
  for i in 0..source.len() {
    for j in 0..source[i].len() {
      for k in 0..source[i][j].len() {
        for l in 0..source[i][j][k].len() {
          if source[i][j][k][l] {
            active_sum += 1;
          }
        }
      }
    }
  }

  println!("Part 2: {}", active_sum);
}

fn print_source(source: &Source) {
  for k in 0..source[0][0].len() {
    println!("z={}", k);
    for j in 0..source[0].len() {
      for i in 0..source.len() {
        if source[i][j][k] {
          print!("#")
        } else {
          print!(".")
        }
      }
      println!("");
    }
    println!("");
  }
}

fn run_cycle(source: &mut Source) {
  let mut points_to_update: Vec<Point> = vec![];
  for i in 0..source.len() {
    for j in 0..source[i].len() {
      for k in 0..source[i][j].len() {
        let neighbours = get_neighbours(source, (i, j, k));
        let active_neighbours = neighbours.iter().fold(0, |sum, neigh| sum + *neigh as usize);
        if source[i][j][k] && (active_neighbours < 2 || active_neighbours > 3) {
          points_to_update.push((i,j,k));
        } else if !source[i][j][k] && active_neighbours == 3 {
          points_to_update.push((i,j,k)) 
        }
      }
    }
  }

  for (i,j,k) in points_to_update {
    source[i][j][k] = !source[i][j][k];
  }
}

fn run_cycle_4d(source: &mut Source4D) {
  let mut points_to_update: Vec<Point4D> = vec![];
  for i in 0..source.len() {
    for j in 0..source[i].len() {
      for k in 0..source[i][j].len() {
        for l in 0..source[i][j][k].len() {
          let neighbours = get_neighbours_4d(source, (i, j, k, l));
          let active_neighbours = neighbours.iter().fold(0, |sum, neigh| sum + *neigh as usize);
          if source[i][j][k][l] && (active_neighbours < 2 || active_neighbours > 3) {
            points_to_update.push((i,j,k, l));
          } else if !source[i][j][k][l] && active_neighbours == 3 {
            points_to_update.push((i,j,k, l)) 
          }
        }
      }
    }
  }

  for (i,j,k, l) in points_to_update {
    source[i][j][k][l] = !source[i][j][k][l];
  }
}

fn get_neighbours(source: &Source, (x,y,z): Point) -> Vec<bool> {
  let x_max = source.len() - 1;
  let y_max = source[0].len() - 1;
  let z_max = source[0][0].len() - 1;

  let xs = if x == 0 {
    0..=1
  } else if x == x_max {
    (x_max-1)..=x_max
  } else {
    (x-1)..=(x+1)
  };
  let ys = if y == 0 {
    0..=1
  } else if y == y_max {
    (y_max-1)..=y_max
  } else {
    (y-1)..=(y+1)
  };
  let zs = if z == 0 {
    0..=1
  } else if z == z_max {
    (z_max-1)..=z_max
  } else {
    (z-1)..=(z+1)
  };

  let mut result = vec![];
  for i in xs {
    for j in ys.clone() {
      for k in zs.clone() {
        if !(i == x && j == y && k == z) {
          result.push(source[i][j][k]);
        }
      }
    }
  }
  result

}


fn get_neighbours_4d(source: &Source4D, (x,y,z, w): Point4D) -> Vec<bool> {
  let x_max = source.len() - 1;
  let y_max = source[0].len() - 1;
  let z_max = source[0][0].len() - 1;
  let w_max = source[0][0][0].len() - 1;
  let xs = if x == 0 {
    0..=1
  } else if x == x_max {
    (x_max-1)..=x_max
  } else {
    (x-1)..=(x+1)
  };
  let ys = if y == 0 {
    0..=1
  } else if y == y_max {
    (y_max-1)..=y_max
  } else {
    (y-1)..=(y+1)
  };
  let zs = if z == 0 {
    0..=1
  } else if z == z_max {
    (z_max-1)..=z_max
  } else {
    (z-1)..=(z+1)
  };
  let ws = if w == 0 {
    0..=1
  } else if w == w_max {
    (w_max-1)..=w_max
  } else {
    (w-1)..=(w+1)
  };

  let mut result = vec![];
  for i in xs {
    for j in ys.clone() {
      for k in zs.clone() {
        for l in ws.clone() {
          if !(i == x && j == y && k == z && l == w) {
            result.push(source[i][j][k][l]);
          }
        }
      }
    }
  }
  result

}
      
fn parse_input() -> Result<Source, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?;

  let mut input_plane: Vec<Vec<bool>> = vec![];

  for line in lines {
    let mut row = vec![];
    for c in line?.chars() {
      if c == '#' {
        row.push(true);
      } else {
        row.push(false);
      }
    }
    input_plane.push(row);
  }


  let mut source: Source = vec![];
  let input_dimensions = (input_plane.len(), input_plane[0].len());

  let source_dimensions = (input_dimensions.0 + 2 * CYCLES, input_dimensions.1 + 2 * CYCLES, 1 + 2* CYCLES);

  for _ in 0..source_dimensions.0 {
    let mut a = vec![];
    for _ in 0..source_dimensions.1 {
      let  mut b = vec![];
      for _ in 0..source_dimensions.2 {
        b.push(false);
      }
      a.push(b);
    }
    source.push(a);
  }

  for i in 0..input_dimensions.0 {
    for j in 0..input_dimensions.1 {
      source[CYCLES+i][CYCLES+j][CYCLES] = input_plane[i][j];
    }
  }

  Ok(source)
}

fn init_source_4d(source: &Source) -> Source4D {
  let mut source_4d: Source4D = vec![];
  for i in 0..source.len() {
    let mut a = vec![];
    for j in 0..source[i].len() {
      let  mut b = vec![];
      for _ in 0..source[i][j].len() {
        let  mut c = vec![];
        for _ in 0..(CYCLES * 2 + 1) {
          c.push(false);
        }
        b.push(c);
      }
      a.push(b);
    }
    source_4d.push(a);
  }
  

  for i in 0..source.len() {
    for j in 0..source[i].len() {
      source_4d[i][j][CYCLES][CYCLES] = source[i][j][CYCLES];
    }
  }

  source_4d
}
use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day03";

struct Map {
  height: usize,
  width: usize,
  trees: Vec<Vec<bool>>,
}

type Point = (usize, usize);
type Route = (usize, usize);

pub fn solve() -> Result<(), Box<dyn Error>> {
  let map = parse_input()?;

  solve_part_one(&map);
  solve_part_two(&map);

  return Ok(());
}

fn solve_part_one(map: &Map) {
  let answer = number_of_trees_on_route(map, (3, 1));
  println!("Part 1 answer: {}", answer);
}

fn solve_part_two(map: &Map) {
  let slopes: Vec<Route> = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

  let mut product: u64 = 1;
  for slope in slopes {
    product *= number_of_trees_on_route(map, slope) as u64;
  }
  println!("Part 2 answer: {}", product);
}

fn check_point_for_tree(map: &Map, point: Point) -> bool {
  let (x, y) = point;
  if y < 0 || y >= map.height {
    return false;
  } else {
    return map.trees[y][x % map.width];
  }
}

fn get_points_on_route(map: &Map, route: Route) -> Vec<Point> {
  let (x,y) = route;
  let num_steps = map.height / y + 1;

  let mut points: Vec<Point> = vec![];
  for i in 0..num_steps {
    points.push((i * x, i * y));
  }
  return points;
}

fn number_of_trees_on_route(map: &Map, route: Route) -> u32 {
  let points = get_points_on_route(map, route);
  let mut count = 0;
  for point in points {
    if check_point_for_tree(map, point) {
      count += 1;
    }
  }

  return count;
}

fn parse_input() -> Result<Map, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?;

  let trees = lines.map(|line| {
    match line {
      Ok(l) => l.chars().map(|c| c == '#').collect::<Vec<_>>(),
      Err(_) => vec![]
    }
  }).collect::<Vec<Vec<bool>>>();

  let map = Map {
    width: trees[0].len(),
    height: trees.len(),
    trees,
  };

  return Ok(map);
}
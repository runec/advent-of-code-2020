use crate::helpers;
use std::error::Error;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/day21";

type ParsedMenu = Vec<(Vec<String>, Vec<String>)>;

pub fn solve() -> Result<(), Box<dyn Error>> {
  let menu_items = parse_input()?;
  solve_part_one(&menu_items);
  /*
  let test_border = border(&(&tiles[0], Direction::N, false), Direction::E);
  let s = test_border.iter().map(|&p| if p {'#'} else {'.'}).collect::<String>();
  println!("Test {}", s);
  */
  return Ok(());
}

fn solve_part_one(menu: &ParsedMenu) {
  let mut ingredients_by_allergen: HashMap<String, Vec<String>> = HashMap::new();

  for (ingredients, allergens) in menu {
    /*
    for ingredient in ingredients {
      let existing_allergens = allergens_by_ingredient.get_mut(ingredient);
      
      if let Some(algs) = existing_allergens {
        algs.retain(|a| allergens.contains(a));
      } else {
        allergens_by_ingredient.insert(ingredient, allergens.iter().map(|a| a).collect());
      }
    }
    */

    for allergen in allergens {
      let existing_ingredients = ingredients_by_allergen.get_mut(allergen);
      
      if let Some(ings) = existing_ingredients {
        ings.retain(|ing| ingredients.contains(ing));
      } else {
        ingredients_by_allergen.insert(allergen.clone(), ingredients.iter().map(|ing| ing.clone()).collect());
      }
    }
  }

  let mut solved_ingredients: HashMap<String, String> = HashMap::new();
  let mut changed = true;
  while changed {
    changed = false;

    for (alg, ings) in ingredients_by_allergen.iter_mut() {
      if ings.len() == 1 {
        if solved_ingredients.get(ings[0].as_str()).is_none() {
          solved_ingredients.insert(ings[0].clone(), alg.clone());
          changed = true;
        }
      } else {
        let prev_len = ings.len();
        ings.retain(|ing| solved_ingredients.get(ing).is_none());
        changed = changed || prev_len != ings.len();
      }
    }
  }

  let mut count = 0;
  for (ingredients, _) in menu {
    for ingredient in ingredients {
      if solved_ingredients.get(ingredient).is_none() {
        count += 1;
      }
    }
  }

  println!("Part 1: {}", count);

  let mut dangerous_ingredients: Vec<(String, String)> = solved_ingredients
    .iter()
    .map(|(ing, alg)| (ing.clone(), alg.clone()))
    .collect();
  dangerous_ingredients.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
  let list = dangerous_ingredients.iter().map(|(ing, _)| ing.clone()).collect::<Vec<_>>().join(",");
  println!("Part 2: {}", list);
}

fn parse_input() -> Result<Vec<(Vec<String>, Vec<String>)>, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?.map(|l| l.unwrap());

  let mut result = vec![];
  for line in lines {
    let mut parts = line.split(" (contains ");
    let ingredients_part = parts.next().unwrap();
    let allergens_part = parts.next().unwrap();

    let ingredients: Vec<String> = ingredients_part.split_whitespace().map(|s| String::from(s)).collect();
    let allergens: Vec<String> = allergens_part[..allergens_part.len()-1].split(", ").map(|s| String::from(s)).collect();

    result.push((ingredients, allergens));
  }

  Ok(result)
}
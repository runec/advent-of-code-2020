use crate::helpers;
use std::error::Error;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/day08";

#[derive(Clone, Debug)]
enum Instruction {
  Nop(i32),
  Acc(i32),
  Jmp(i32),
}

type Program = Vec<Instruction>;

type Execution<'a> = (&'a Program, i32, usize);

pub fn solve() -> Result<(), Box<dyn Error>> {
  let program = parse_input()?;

  solve_part_one(&program);
  solve_part_two(&program);

  return Ok(());
}

fn solve_part_one(program: &Program) {
  let (_terminated, acc) = run_program(program);

  println!("Part 1: {}", acc);
}

fn solve_part_two(program: &Program) {
  for i in 0..program.len() {
    let instruction = program.get(i).unwrap();
    match *instruction {
      Instruction::Nop(val) => {
        let mut modded_program: Program = (*program).clone();
        modded_program[i] = Instruction::Jmp(val);
        let (terminated, acc) = run_program(&modded_program);
        if terminated {
          println!("Part 2: {}", acc);
        }
      },
      Instruction::Jmp(val) => {
        let mut modded_program: Program = (*program).clone();
        modded_program[i] = Instruction::Nop(val);
        let (terminated, acc) = run_program(&modded_program);
        if terminated {
          println!("Part 2: {}", acc);
        }
      },
      _ => (),
    }
  }
}

fn run_program(program: &Program) -> (bool, i32) {
  let mut execution: Execution = (program, 0, 0);
  let mut inst_count: HashMap<usize, u32> = HashMap::new();

  loop {
    let inst = execution.2;
    if inst >= program.len() {
      return (true, execution.1);
    } else if let Some(_x) = inst_count.get(&inst) {
      return (false, execution.1)
    } else {
      inst_count.insert(inst, 1);
    }
    execution = execute_next(execution);
  }
}

fn execute_next(execution: Execution) -> Execution {
  let (program, acc, inst) = execution;
  let current_instruction = program.get(inst).unwrap();

  match current_instruction {
    Instruction::Nop(_val) => (program, acc, inst + 1),
    Instruction::Acc(val) => (program, acc + val, inst + 1),
    Instruction::Jmp(val) => (program, acc, (inst as i32 + val) as usize),
  }
}

fn parse_input() -> Result<Program, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?;

  let mut program: Program = vec![];

  for line in lines {
    let line = line?;
    let mut parts = line.split_whitespace();

    let operation = parts.next().unwrap().to_string();
    let arg = parts.next().unwrap().parse::<i32>()?;

    match operation.as_str() {
      "acc" => program.push(Instruction::Acc(arg)),
      "jmp" => program.push(Instruction::Jmp(arg)),
      "nop" => program.push(Instruction::Nop(arg)),
      _ => (),
    }

  }

  return Ok(program);
}
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::str::FromStr;
use std::collections::HashSet;

use crate::aoc_test;

#[derive(Debug, Clone, Copy)]
enum Operation {
  Acc,
  Jmp,
  Nop
}

impl FromStr for Operation {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "acc" => Ok(Operation::Acc),
      "jmp" => Ok(Operation::Jmp),
      "nop" => Ok(Operation::Nop),
      _ => Err("Invalid instruction"),
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
  op: Operation,
  arg: i32,
}

impl FromStr for Instruction {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split_whitespace().collect();

    if parts.len() != 2 {
      return Err("Invalid number of arguments");
    }

    let op = Operation::from_str(parts.get(0).ok_or("Missing op")?)?;
    let arg = parts.get(1).ok_or("Missing arg")?.parse::<i32>().map_err(|_| "Arg is not an i32")?;

    Ok(
      Instruction {
        op,
        arg,
      }
    )
  }
}

#[derive(Debug, Clone)]
struct CPU {
  instructions: Vec<Instruction>,
  acc: i32,
  pc: i32,
}

impl CPU {
  fn new(instructions: Vec<Instruction>) -> CPU {
    CPU {
      instructions,
      acc: 0,
      pc: 0,
    }
  }

  fn next(&mut self) -> bool {
    // If the program counter is greater than the length of our instructions list, it can terminate
    if self.pc >= self.instructions.len() as i32 {
      return false;
    }

    let instruction = self.instructions.get(self.pc as usize).expect("Program counter out of bounds");

    match instruction.op {
      Operation::Acc => self.acc += instruction.arg,
      Operation::Jmp => self.pc += instruction.arg,
      Operation::Nop => (),
    };

    // We don't want to increment program counter for jmp instructions
    match instruction.op {
      Operation::Jmp => (),
      _ => self.pc += 1,
    };

    true
  }
}

#[aoc_generator(day8)]
fn generator(input: &str) -> CPU {
  let input = input.lines();

  let instructions: Vec<Instruction> = input.map(|line| {
    Instruction::from_str(line).expect("Unable to parse instruction")
  }).collect();

  CPU::new(instructions)
}

#[aoc(day8, part1)]
fn part1(input: &CPU) -> i32 {
  let mut input = (*input).clone();
  let mut run = HashSet::new();

  while !run.contains(&input.pc) {
    run.insert(input.pc);
    input.next();
  }

  input.acc
}

#[aoc(day8, part2)]
fn part2(input: &CPU) -> i32 {
  let input = (*input).clone();

  // Loop over each instruction
  for (i, instruction) in input.instructions.iter().enumerate() {
    let arg = instruction.arg;

    match instruction.op {
      // If the instruction we are checking is a jmp or a nop
      Operation::Jmp | Operation::Nop => {
        let mut new_instructions = input.instructions.clone();

        // Switch the operation of the instruction
        new_instructions[i] = Instruction {
          op: match instruction.op {
            Operation::Jmp => Operation::Nop,
            Operation::Nop => Operation::Jmp,
            _ => unreachable!(),
          },
          arg,
        };

        let mut new_cpu = CPU::new(new_instructions);

        // If there's not a loop in these instructions, we can return the value of acc
        if !check_loop(&new_cpu) {
          while new_cpu.next() {}

          return new_cpu.acc;
        }
      },
      // Otherwise we can ignore this instruction
      _ => (),
    }
  }

  input.acc
}

// Simple functiont to check if there's a loop in the program or not
fn check_loop(input: &CPU) -> bool {
  let mut input = input.clone();
  let mut run = HashSet::new();

  while input.next() {
    if run.contains(&input.pc) {
      return true;
    }

    run.insert(input.pc);
  }

  false
}

aoc_test! {
  input = "
    nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6
  ";

  part1 = "5";
  part2 = "8";
}

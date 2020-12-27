use anyhow::{Result, anyhow};
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword { Acc, Jmp, Nop, }

#[derive(Debug, Clone)]
pub struct Instruction {
    keyword: Keyword,
    arg: i32,
}

peg::parser!{
    grammar instruction_parser() for str {
        rule keyword() -> Keyword
            = acc() / jmp() / nop()

        rule acc() -> Keyword
             = "acc" { Keyword::Acc }

        rule jmp() -> Keyword
             = "jmp" { Keyword::Jmp }

        rule nop() -> Keyword
             = "nop" { Keyword::Nop }

        rule number() -> i32
            = n:$( ['+'|'-'] ['0'..='9']+) { n.parse().unwrap() }

        pub rule instruction() -> Instruction
            = k:keyword() " " n:number() { 
                Instruction {
                    keyword: k, 
                    arg: n,
                }
            }
    }
}

fn run_program(instructions: &[Instruction]) -> Result<(bool, usize, i32)> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut accumulator: i32 = 0;
    let mut pc: usize = 0;
    let mut success = false;

    loop {
        if visited.contains(&pc) { break; }
        if pc == instructions.len() { success = true; break; }
        visited.insert(pc);
        let instruction = instructions.get(pc).ok_or(anyhow!("pc out of bounds {}, {}", pc, accumulator))?;
        match &instruction.keyword {
            Keyword::Nop => { pc += 1; },
            Keyword::Acc => {
                accumulator += instruction.arg;
                pc += 1; 
            },
            Keyword::Jmp => { pc = ((pc as i32) + instruction.arg).try_into().unwrap() },
        }
    }
    Ok((success, pc, accumulator))
}

pub fn eight() -> Result<()> {
    let content = include_str!("input/eight.txt");
    let instructions: Vec<Instruction> = content
        .lines()
        .map(|line| instruction_parser::instruction(line))
        .filter_map(Result::ok)
        .collect();

    let variants: Vec<Vec<Instruction>> = instructions
        .iter()
        .enumerate()
        .filter(|(_, instr)| instr.keyword == Keyword::Jmp || instr.keyword == Keyword::Nop)
        .map(|(index, instr)| {
            let mut variant = instructions.clone();
            variant[index].keyword = match instr.keyword {
                Keyword::Jmp => Keyword::Nop,
                Keyword::Nop => Keyword::Jmp,
                Keyword::Acc => Keyword::Acc,
            };
            variant
        })
    .collect();


    variants
        .iter()
        .map(|list| run_program(list))
        .filter_map(|res| {
            match res {
                Ok(x) => Some(x),
                Err(message) => {
                    dbg!(message);
                    None 
                }
            }
        })
        .filter(|(success, _, _)| *success)
        .for_each(|(success, pc, accumulator)| {
            dbg!(success, pc, accumulator);
        });
    
    Ok(())
}


// plan:
// instructions form a graph with one cycle
// each instruction has a 'next' instruction
// each nop and jmp as an alternate next instruction
// how can we find the one alternate instruction to use that avoids a cycle?
// ideas:
//  - sections of code that can or can't be escaped from
//  - some kind of graph traversal algorithm
//  - weird stateful graph, where we keep track of whether we've taken a alt or not, and backtrack
//  if we hit a cycle then?
//  - check them all

use anyhow::Result;
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug)]
pub enum Keyword { Acc, Jmp, Nop, }

#[derive(Debug)]
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

pub fn eight() -> Result<()> {
    let content = include_str!("input/eight.txt");
    let instructions: Vec<Instruction> = content
        .lines()
        .map(|line| instruction_parser::instruction(line))
        .filter_map(Result::ok)
        .collect();

    let mut visited: HashSet<usize> = HashSet::new();
    let mut accumulator: i32 = 0;
    let mut pc: usize = 0;
    
    loop {
        if visited.contains(&pc) { break; }
        visited.insert(pc);
        let instruction = instructions.get(pc).expect("pc not in bounds of program");
        dbg!(pc, instruction);
        match &instruction.keyword {
            Keyword::Nop => { pc += 1; },
            Keyword::Acc => {
                accumulator += instruction.arg;
                pc += 1; 
            },
            Keyword::Jmp => { pc = ((pc as i32) + instruction.arg).try_into().unwrap() },
        }
    }
    dbg!(pc, accumulator, visited.len());

    Ok(())
}

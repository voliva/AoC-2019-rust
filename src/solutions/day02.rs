use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver<Vec<usize>, usize, usize> for Problem {
    fn read_input(&self, file_reader: BufReader<&File>) -> Vec<usize> {
        file_reader
            .lines()
            .filter_map(|x| x.ok())
            .flat_map(|line| {
                line.split(',')
                    .map(|x| x.parse())
                    .filter_map(|x| x.ok())
                    .collect::<Vec<usize>>()
            })
            .collect()
    }
    fn solve_first(&self, input: &Vec<usize>) -> Result<usize, String> {
        let mut program = input.clone();
        program[1] = 12;
        program[2] = 2;
        return intcode(program);
    }
    fn solve_second(&self, input: &Vec<usize>) -> Result<usize, String> {
        let mut program = input.clone();
        for i in 0..99 {
            program[1] = i;
            for j in 0..99 {
                program[2] = j;
                let result = intcode(program.clone());
                match result {
                    Ok(19690720) => return Ok(100 * i + j),
                    _ => {}
                }
            }
        }
        return Err(format!("Couldn't find the specific value"));
    }
}

fn intcode(mut program: Vec<usize>) -> Result<usize, String> {
    let mut ip = 0;
    while program[ip] != 99 {
        match program[ip] {
            1 => {
                let a = program[ip + 1];
                let b = program[ip + 2];
                let out = program[ip + 3];
                program[out] = program[a] + program[b];
                ip += 4;
            }
            2 => {
                let a = program[ip + 1];
                let b = program[ip + 2];
                let out = program[ip + 3];
                program[out] = program[a] * program[b];
                ip += 4;
            }
            x => return Err(format!("Unknown opcode {}", x)),
        }
    }

    Ok(program[0])
}

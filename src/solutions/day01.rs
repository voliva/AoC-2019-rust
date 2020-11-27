use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver<Vec<isize>, isize, isize> for Problem {
    fn read_input(&self, file_reader: BufReader<&File>) -> Vec<isize> {
        file_reader
            .lines()
            .filter_map(|x| x.ok())
            .map(|line| line.parse())
            .filter_map(|x| x.ok())
            .collect()
    }
    fn solve_first(&self, input: &Vec<isize>) -> Result<isize, String> {
        Ok(input.into_iter().map(|x| get_fuel(*x)).sum())
    }
    fn solve_second(&self, input: &Vec<isize>) -> Result<isize, String> {
        Ok(input.into_iter().map(|x| get_fuel_rec(*x)).sum())
    }
}

fn get_fuel(mass: isize) -> isize {
    return (mass / 3) - 2;
}

fn get_fuel_rec(mass: isize) -> isize {
    let fuel = get_fuel(mass);
    if fuel < 0 {
        return 0;
    }
    return fuel + get_fuel_rec(fuel);
}

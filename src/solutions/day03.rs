use super::Solver;
use std::cmp;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver<Vec<Vec<String>>, usize, usize> for Problem {
    fn read_input(&self, file_reader: BufReader<&File>) -> Vec<Vec<String>> {
        file_reader
            .lines()
            .filter_map(|x| x.ok())
            .map(|line| line.split(",").map(|x| String::from(x)).collect())
            .collect()
    }
    fn solve_first(&self, input: &Vec<Vec<String>>) -> Result<usize, String> {
        let first_path = get_wire_positions(&input[0]);
        let second_path = get_wire_positions(&input[1]);
        // print_wires(&first_path, &second_path);
        let intersections = get_intersections(&first_path, &second_path);
        let result = intersections
            .into_iter()
            .map(Reverse)
            .collect::<BinaryHeap<_>>();
        let Reverse(closest) = result.peek().unwrap();
        Ok((closest.x.abs() + closest.y.abs()).try_into().unwrap())
    }
    fn solve_second(&self, _input: &Vec<Vec<String>>) -> Result<usize, String> {
        todo!()
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Ord)]
struct Position {
    x: isize,
    y: isize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let this_distance = self.x.abs() + self.y.abs();
        let other_distance = other.x.abs() + other.y.abs();
        this_distance.partial_cmp(&other_distance)
    }
}

fn get_wire_positions(wire: &Vec<String>) -> HashSet<Position> {
    let mut result = HashSet::new();
    let mut position = Position { x: 0, y: 0 };
    for (dir, quantity) in wire.into_iter().map(parse_wire_command) {
        for _ in 1..=quantity {
            match dir {
                'R' => position.x += 1,
                'L' => position.x -= 1,
                'U' => position.y -= 1,
                'D' => position.y += 1,
                _ => {}
            }
            result.insert(position.clone());
        }
    }

    result
}
fn get_intersections(wire_a: &HashSet<Position>, wire_b: &HashSet<Position>) -> HashSet<Position> {
    let mut result = HashSet::new();
    for pos in wire_a {
        if wire_b.contains(pos) {
            result.insert(pos.clone());
        }
    }

    result
}

fn parse_wire_command(command: &String) -> (char, usize) {
    let (dir, value) = command.split_at(1);

    (dir.chars().nth(0).unwrap(), value.parse::<usize>().unwrap())
}

#[allow(dead_code)]
fn print_wires(wire_a: &HashSet<Position>, wire_b: &HashSet<Position>) {
    let min_position =
        wire_a
            .into_iter()
            .chain(wire_b.iter())
            .fold(Position { x: 0, y: 0 }, |acc, val| Position {
                x: cmp::min(acc.x, val.x),
                y: cmp::min(acc.y, val.y),
            });
    let max_position =
        wire_a
            .into_iter()
            .chain(wire_b.iter())
            .fold(Position { x: 0, y: 0 }, |acc, val| Position {
                x: cmp::max(acc.x, val.x),
                y: cmp::max(acc.y, val.y),
            });
    for y in (min_position.y)..=(max_position.y) {
        let mut line = String::new();
        for x in (min_position.x)..=(max_position.x) {
            if y == 0 && x == 0 {
                line.push_str("#");
                continue;
            }
            let mut v = 0;
            if wire_a.contains(&Position { x, y }) {
                v += 1
            }
            if wire_b.contains(&Position { x, y }) {
                v += 1
            }
            line.push_str(&format!("{}", v))
        }
        println!("{}", line);
    }
}

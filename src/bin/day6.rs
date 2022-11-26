use aoc2018::util;
use std::collections::HashSet;

fn main(){
    let path = "inputs/day6.txt";
    let lines = util::read_lines(path).unwrap();
    let mut input: HashSet<(u16,u16)> =  HashSet::new();
    for line in lines{
        let in_str = line.unwrap();
        let coord: Vec<&str> = in_str.split(", ").collect();
        input.insert((coord[0].parse::<u16>().unwrap(),coord[1].parse::<u16>().unwrap()));
    }
    
}
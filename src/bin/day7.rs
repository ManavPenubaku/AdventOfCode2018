use std::collections::HashMap;

use aoc2018::util;
use regex::Regex;

fn p1(input: HashMap<char,Vec<char>>) -> String{
    let mut feasible_steps: Vec<char> = Vec::new();
    let mut out: Vec<char> = Vec::new();
    let mut count;
    let mut iter_steps = input.clone();
    while out.len() != input.len(){
        for (key,values) in &iter_steps{
            count = 0;
            for value in values{
                if out.contains(value){
                    count+=1;
                }
            }
            if count == values.len(){
                feasible_steps.push(*key);
            }
        }
        feasible_steps.sort();
        out.push(feasible_steps[0]);
        iter_steps.remove(&feasible_steps[0]);
        feasible_steps = Vec::new();
    }
    let out_string = out.iter().collect();
    return out_string;
}

fn main(){
    let path = "inputs/day7.txt";
    let lines = util::read_lines(path).unwrap();
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin").unwrap();
    let mut steps = HashMap::<char, Vec<char>>::new();
    for line in lines{
        let text = line.unwrap();
        let cap = re.captures(&text).unwrap();
        let parent = cap[1].chars().next().unwrap();
        let child = cap[2].chars().next().unwrap();
        steps.entry(child).or_default().push(parent);
        steps.entry(parent).or_default();
    }
    println!("Part 1 : {}",p1(steps));
}
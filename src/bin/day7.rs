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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Worker{
    instruction: char,
    start_time: u32
}

impl Worker {
    pub fn new(instruction: char, start_time: u32) -> Self {
        Worker {
            instruction,
            start_time
        }
    }
}

fn p2(input: HashMap<char,Vec<char>>) ->u32{
    let mut feasible_steps: Vec<char> = Vec::new();
    let mut time_elapsed: u32 = 0;
    let mut out: Vec<char> = Vec::new();
    let mut iter_steps= input.clone();
    let mut count;
    let mut prev_out_len = 1;

    let worker_count = 5;
    let mut workers: Vec<Worker> = Vec::with_capacity(worker_count);
    for _i in 0..worker_count{
        let worker = Worker::new('0',0);
        workers.push(worker);
    }
    while out.len() != input.len(){
        if prev_out_len != out.len(){
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
        }
        prev_out_len = out.len();
        feasible_steps.sort();
        for i in 0..worker_count{
            if workers[i].instruction == '0' && !feasible_steps.is_empty(){
                workers[i].instruction = feasible_steps[0];
                workers[i].start_time = time_elapsed;
                feasible_steps.remove(0);
                iter_steps.remove(&workers[i].instruction);
            }
            if workers[i].instruction != '0'{
                if (time_elapsed - workers[i].start_time) == (workers[i].instruction as u32 - 4){
                    out.push(workers[i].instruction);
                    workers[i].instruction = '0';
                    workers[i].start_time = 0;
                    time_elapsed -= 1;
                }
            }
        }
        feasible_steps = Vec::new();
        time_elapsed+=1;
    }
    for character in out{
        print!("{}",character);
    }
    return time_elapsed;
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
    println!("Part 1 : {}",p1(steps.clone()));
    println!("Part 2 : {}", p2(steps));
}
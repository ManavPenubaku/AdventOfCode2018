use aoc2018::util;
use std::collections::HashSet;

fn read_ints(path: &str) -> Vec<i32>{
    let mut out: Vec<i32> = Vec::new();
    let lines = util::read_lines(path).unwrap();

    for line in lines{
        let num = line.unwrap().parse::<i32>().unwrap();
        out.push(num);
    }
    out
}
fn main() {
    let nums = read_ints("inputs/day1.txt");
    let mut frequency_p1 = 0;
    for num in nums.iter(){
        frequency_p1 += num;
    }
    println!("Solution to Part 1 is : {}",frequency_p1);

    let mut frequency_dict: HashSet<i32> = HashSet::new();
    let mut frequency_p2 = 0;
    let mut index = 0;
    while !frequency_dict.contains(&frequency_p2){
        frequency_dict.insert(frequency_p2);
        frequency_p2 += nums[index];
        if index < nums.len() - 1{
            index += 1;
        }else {
            index = 0;
        }
    }
    println!("Solution to Part 2 is : {}",frequency_p2)
}

use aoc2018::util;

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
    let mut frequency = 0;
    for num in nums.iter(){
        frequency += num;
    }
    println!("Solution to Part 1 is : {}",frequency);
}

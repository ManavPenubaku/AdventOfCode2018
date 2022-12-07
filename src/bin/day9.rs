use aoc2018::util;
use std::collections::VecDeque;
use std::collections::HashMap;

fn main(){
    let path = "inputs/day9.txt";
    let input = util::read_string(path).unwrap();
    let input_split = input.split(" ");

    let vec: Vec<&str> = input_split.collect();
    let player_count = vec[0].parse::<usize>().unwrap();
    let last_marble = vec[6].parse::<i32>().unwrap();

    let mut marble_list = VecDeque::from([0,1]);
    let mut score: HashMap<i32,u32> = HashMap::with_capacity(player_count);
    for n in 2..(last_marble*100 + 1){
        if n % 23 == 0{
            marble_list.rotate_right(7);
            let marble = marble_list.pop_back().unwrap() + n;
            let player_number = n % player_count as i32;
            *score.entry(player_number).or_insert(0) += marble as u32;
            marble_list.rotate_left(1);
        }else{
            marble_list.rotate_left(1);
            marble_list.push_back(n);
        }
        if n == last_marble{
            println!("Part 1 : {}",score.values().max().unwrap());
        }
    }
    println!("Part 2 : {}",score.values().max().unwrap());
}
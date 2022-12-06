use aoc2018::util;
use std::collections::LinkedList;
use std::collections::HashMap;

fn main(){
    let path = "inputs/day9.txt";
    let input = util::read_string(path).unwrap();
    let input_split = input.split(" ");

    let vec: Vec<&str> = input_split.collect();
    let player_count = vec[0].parse::<usize>().unwrap();
    let last_marble = vec[6].parse::<i32>().unwrap();

    let mut marble_list = LinkedList::from([0,1]);
    let mut score: HashMap<i32,i32> = HashMap::with_capacity(player_count);
    let mut index: i32 = 1;
    for n in 2..(last_marble * 100 + 1){
        if n%23 == 0{
            let mut remove_index = (index - 7) % (marble_list.len() as i32);
            if remove_index <= 0{
                remove_index += marble_list.len() as i32;
            }
            let mut split = marble_list.split_off(remove_index as usize + 1);
            let player_number = n % player_count as i32;
            let marble = marble_list.pop_back().unwrap() + n;
            *score.entry(player_number).or_insert(0) += marble;
            marble_list.append(&mut split);
            index = remove_index;
        }else{
            let mut insert_index = (index+2) % (marble_list.len() as i32);
            if insert_index == 0{
                insert_index += marble_list.len() as i32;
            }
            let mut split = marble_list.split_off(insert_index as usize);
            marble_list.push_back(n);
            marble_list.append(&mut split);
            index = insert_index;
        }
        if n == last_marble{
            println!("Part 1 : {}",score.values().max().unwrap());
        }
    }
    println!("Part 2 : {}",score.values().max().unwrap());
}
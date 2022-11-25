use aoc2018::util;

fn execute_reaction(polymer: Vec<char>) -> Vec<char>{
    let mut out: Vec<char> = polymer.clone();
    let mut i = 0;
    while i < out.len()-1{
        let diff = out[i] as i32 - out[i+1] as i32;
        if diff.abs() == 32{
            out.drain(i..(i+2));
        }else{
            i+=1;
        }
    }
    if out.len() != polymer.len(){
        return execute_reaction(out);
    }else{
        return out;
    }
}

fn main(){
    let path = "inputs/day5.txt";
    let lines = util::read_lines(path).unwrap();
    for line in lines{
        let input: Vec<char> = line.unwrap().chars().collect();
        let output = execute_reaction(input.clone());
        println!("Part 1 : {}",output.len());
        let mut min_polymer = input.len();
        for i in 97u8..=122 {
            let char_rem = i as char;
            let mut temp_input = input.clone();
            temp_input.retain(|&x| x!=char_rem && x!=char_rem.to_ascii_uppercase());
            let output_2 = execute_reaction(temp_input.clone());
            if output_2.len() < min_polymer{
                min_polymer = output_2.len();
            }
        }
        println!("Part 2 : {}",min_polymer);
    }
}
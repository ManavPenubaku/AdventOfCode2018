use aoc2018::util;

fn read_boxids(path: &str) -> Vec<String>{
    let mut out: Vec<String> = Vec::new();
    let lines = util::read_lines(path).unwrap();

    for line in lines{
        let boxid = line.unwrap();
        out.push(boxid);
    }
    out
}

fn main(){
    let boxids = read_boxids("inputs/day2.txt");
    let mut id_twos = 0;
    let mut id_threes = 0;
    for id in &boxids{
        let mut two_flag = 0;
        let mut three_flag = 0;
        for char in id.chars(){
            let c = id.matches(char).count();
            if c == 2 {two_flag = 1};
            if c == 3 {three_flag = 1};
        }
        id_twos += two_flag;
        id_threes += three_flag;
    }
    let checksum = id_twos * id_threes;
    println!("Solution to Part 1 is : {}",checksum);

    let string_lengths = boxids[0].chars().count();
    println!("{}",string_lengths as i32);
    for id_1 in &boxids{
        let mut match_chars = string_lengths as i32;
        let comp_length = string_lengths as i32;
        let mut diff_char = 'a';
        for id_2 in &boxids{
            for n in 0..string_lengths{
                if id_1.chars().nth(n).unwrap() != id_2.chars().nth(n).unwrap(){
                    match_chars -= 1;
                    diff_char = id_1.chars().nth(n).unwrap();
                }
                if match_chars < comp_length-1 {continue};
            }
            if match_chars == comp_length-1{
                println!("Solution to Part 2 is : {}",id_1.replace(diff_char, ""));
                break;
            }
            match_chars = comp_length;
        }
        if match_chars == comp_length-1{break};
    }
}
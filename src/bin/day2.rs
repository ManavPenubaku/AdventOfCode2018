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
    for id in boxids{
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
}
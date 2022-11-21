use aoc2018::util;
use regex::Regex;
use std::collections::HashSet;

fn get_cutregions(path: &str) -> Vec<HashSet<(u16,u16)>>{
    let mut out: Vec<HashSet<(u16,u16)>> = Vec::new();
    let lines = util::read_lines(path).unwrap();

    for line in lines{
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let text = line.unwrap();
        for cap in re.captures_iter(&text){
            let x = cap[2].parse::<u16>().unwrap();
            let y = cap[3].parse::<u16>().unwrap();
            let w = cap[4].parse::<u16>().unwrap();
            let h = cap[5].parse::<u16>().unwrap();
            let mut coord_set: HashSet<(u16,u16)> = HashSet::new();
            for i in x..(x+w){
                for j in y..(y+h){
                    coord_set.insert((i,j));
                }
            }
            out.push(coord_set);
        }
    }
    return out;
}

fn main(){
    let cut_regions = get_cutregions("inputs/day3.txt");
    let mut intersect_fabric: HashSet<(u16,u16)> = HashSet::new();
    for i in 0..cut_regions.len(){
        let claim_a = &cut_regions[i];
        for j in (i+1)..cut_regions.len(){
            let claim_b = &cut_regions[j];
            let temp_intersect = claim_a.intersection(&claim_b);
            intersect_fabric.extend(temp_intersect);
        }
        if claim_a.is_disjoint(&intersect_fabric){
            println!("Intact claim is : {}",i+1);
        }
    }
    println!("Fabric with two or more claims : {}",intersect_fabric.len());
}
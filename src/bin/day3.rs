use aoc2018::util;
use regex::Regex;
use std::collections::HashSet;

fn get_cutregions(path: &str) -> Vec<[u16; 5]>{
    let mut out: Vec<[u16; 5]> = Vec::new();
    let lines = util::read_lines(path).unwrap();

    for line in lines{
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let text = line.unwrap();
        for cap in re.captures_iter(&text){
            let x = cap[2].parse::<u16>().unwrap();
            let y = cap[3].parse::<u16>().unwrap();
            let w = cap[4].parse::<u16>().unwrap();
            let h = cap[5].parse::<u16>().unwrap();
            let claim_no = cap[1].parse::<u16>().unwrap();
            let coord: [u16; 5] = [x, x+w, y,y+h,claim_no];
            out.push(coord);
        }
    }
    return out
}

fn get_coord_set(coord: &[u16]) -> HashSet<(u16,u16)>{
    let mut coord_set: HashSet<(u16,u16)> = HashSet::new();
    for i in coord[0]..coord[1]{
        for j in coord[2]..coord[3]{
            coord_set.insert((i,j));
        }
    }
    return coord_set;
}   

fn main(){
    let cut_regions = get_cutregions("inputs/day3.txt");
    let mut intersect_fabric: HashSet<(u16,u16)> = HashSet::new();
    let claim_counts  = cut_regions.len();
    let mut has_intersection: Vec<u8> = Vec::with_capacity(claim_counts);
    for n in 0..cut_regions.len() {has_intersection.push(0);    }
    for i in 0..cut_regions.len(){
        let claim_a = get_coord_set(&cut_regions[i]);
        for j in (i+1)..cut_regions.len(){
            let claim_b = get_coord_set(&cut_regions[j]);
            if !claim_a.is_disjoint(&claim_b){
                let temp_intersect = claim_a.intersection(&claim_b);
                intersect_fabric.extend(temp_intersect);
                has_intersection[i] = 1;
                has_intersection[j] = 1;
            }
        }
        if has_intersection[i] == 0{
            println!("{}",i);
            println!("Intact claim is : {}",cut_regions[i][4])
        }
    }
    println!("Fabric with two or more claims : {}",intersect_fabric.len());

}
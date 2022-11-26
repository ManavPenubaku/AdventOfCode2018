use aoc2018::util;
use std::collections::HashMap;
use std::cmp;

fn manhattan_distance(p1: Vec<i32>,p2: Vec<i32>) -> i32{
    let dist = (p1[0]-p2[0]).abs() + (p1[1]-p2[1]).abs();
    return dist;
}

fn main(){
    let path = "inputs/day6.txt";
    let lines = util::read_lines(path).unwrap();
    let mut coord_closest: HashMap<Vec<i32>,u16> =  HashMap::new();
    let (mut max_x, mut max_y) = (0,0);
    for line in lines{
        let in_str = line.unwrap();
        let coord: Vec<&str> = in_str.split(", ").collect();
        let x = coord[0].parse::<i32>().unwrap();
        let y = coord[1].parse::<i32>().unwrap();
        max_x = cmp::max(max_x, x);
        max_y = cmp::max(max_y, y);
        coord_closest.insert(vec![x,y],0);
    }
    let grid_max = cmp::max(max_x,max_y) + 1;
    let mut min_dist;
    let mut min_count;
    let mut coord_update = vec![0,0];
    let mut infinite_coord: Vec<Vec<i32>> = Vec::new();
    let mut total_dist;
    let mut safe_region_area = 0;
    for i in 0..grid_max{
        for j in 0..grid_max{
            min_dist = 10000;
            min_count = 0;
            total_dist = 0;
            for (coord,_count) in &coord_closest{
                let dist = manhattan_distance(coord.to_vec(), vec![i,j]);
                total_dist += dist;
                if dist < min_dist{
                    min_dist = dist;
                    min_count = 1;
                    coord_update = coord.to_vec();
                }else if dist == min_dist{
                    min_count += 1;
                }
            }
            if min_count==1{
                *coord_closest.entry(coord_update.clone()).or_insert(0) += 1;
                if i == 0 || j ==0 || i == grid_max-1 || j == grid_max-1{
                    infinite_coord.push(coord_update.clone());
                } 
            }
            if total_dist < 10000{
                safe_region_area+=1;
            }
        }
    }
    for c in infinite_coord{
        coord_closest.remove(&c);
    }
    let largest_area:Vec<u16> = coord_closest.values().cloned().collect();
    println!("Part 1 : {}",largest_area.iter().max().unwrap());
    println!("Part 2 : {}",safe_region_area);
}
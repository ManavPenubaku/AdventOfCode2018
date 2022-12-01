use aoc2018::util;

fn p1(license_file : Vec<u16>, mut index : usize) -> (u32,usize){
    let mut metadata_sum: u32 = 0;
    let node_count = license_file[index];
    index+=1;
    let metadata_count = license_file[index];
    index+=1;
    let mut processed_nodes = 0;
    if node_count != 0{
        while processed_nodes != node_count{
            let(metadata_sub_sum, index_temp) = p1(license_file.clone(), index);
            index = index_temp;
            metadata_sum += metadata_sub_sum;
            processed_nodes += 1;
        }
    }
        
    for _n in 0..metadata_count{
        metadata_sum += license_file[index] as u32;
        index+=1;
    }
    return (metadata_sum,index);
}

fn p2(license_file : Vec<u16>, mut index : usize) -> (u32,usize){
    let mut metadata_sum: u32 = 0;
    let node_count = license_file[index];
    index+=1;
    let metadata_count = license_file[index];
    index+=1;
    let mut processed_nodes = 0;
    let mut node_sums = vec![0;node_count as usize];
    if node_count != 0{
        while processed_nodes != node_count{
            let(metadata_sub_sum, index_temp) = p2(license_file.clone(), index);
            index = index_temp;
            node_sums[processed_nodes as usize] = metadata_sub_sum;
            processed_nodes += 1;
        }
    }else{
        for _n in 0..metadata_count{
            metadata_sum += license_file[index] as u32;
            index+=1;
        }
        return (metadata_sum,index);
    }
    for _n in 0..metadata_count{
        if license_file[index] != 0 && license_file[index] <= node_count{
            metadata_sum += node_sums[(license_file[index]-1) as usize];
        }
        index+=1;
    }
    return (metadata_sum,index);
}

fn main(){
    let path = "inputs/day8.txt";
    let mut license_file: Vec<u16> = Vec::new();

    let lines = util::read_lines(path).unwrap();
    for line in lines{
        let text = line.unwrap();
        let license_file_str: Vec<&str> = text.split(' ').collect();
        for num_str in license_file_str{
            let num = num_str.parse().unwrap();
            license_file.push(num);
        }
    }
    let (p1_sol,_index) = p1(license_file.clone(),0);
    let (p2_sol,_index) = p2(license_file,0);
    println!("Part 1 : {}",p1_sol);
    println!("Part 1 : {}",p2_sol);
}
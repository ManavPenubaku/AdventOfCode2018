use aoc2018::util;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Record{
    timestamp: String,
    entry: String
}

impl Record {
    pub fn new(timestamp: String, entry: String) -> Self {
        Record {
            timestamp,
            entry
        }
    }
}

fn get_guardpost_records(path: &str) -> Vec<Record>{
    let mut out: Vec<Record> = Vec::new();
    let lines = util::read_lines(path).unwrap();    
    for line in lines{
        let text = line.unwrap();
        let split: Vec<&str> = text.split("]").collect();
        out.push(Record::new(split[0].to_string(), split[1].to_string()));
    }
    return out;
}

fn main(){
    let mut guardpost_records = get_guardpost_records("inputs/day4.txt");
    guardpost_records.sort();

    let mut guard_sleep_record: HashMap<u16,Vec<u16>> = HashMap::new();

    let mut start_time = 0;
    let mut end_time;

    let re = Regex::new(r"\[\d+-\d+-\d+ 00:(\d+)").unwrap();

    let re_guard = Regex::new(r" Guard #(\d+) begins shift").unwrap();
    let mut guard_number = 0;

    for record in guardpost_records{
        if record.entry == " falls asleep"{
            for cap in re.captures_iter(&record.timestamp){
                start_time = cap[1].parse::<u16>().unwrap();
            }
        }else if record.entry == " wakes up" {
            for cap in re.captures_iter(&record.timestamp){
                end_time = cap[1].parse::<u16>().unwrap();
                for n in start_time..end_time{
                    guard_sleep_record.entry(guard_number).or_insert(vec![0;60])[n as usize] += 1;
                }
            }
        }else{
            for cap in re_guard.captures_iter(&record.entry){
                guard_number = cap[1].parse::<u16>().unwrap();
                if !guard_sleep_record.contains_key(&guard_number){
                    guard_sleep_record.insert(guard_number,vec![0;60]);
                }
            }
        }
    }

    let mut max_val;
    let mut max_index = 0;
    let mut max_val_2 = 0;
    let mut max_index_2 = 0;
    let mut guard_index = 0;
    let mut guard_index_2 = 0;
    let mut max_sum = 0;
    for (k,v) in guard_sleep_record.iter(){
        let sum : u16 = v.iter().sum();
        if sum > max_sum{
            max_val = 0;
            max_index = 0;
            guard_index = *k as u32;
            max_sum = sum;
            for n in 0..60{
                if v[n] > max_val{
                    max_val = v[n];
                    max_index = n as u32;
                }
            }
        }
        for n in 0..60{
            if v[n] > max_val_2{
                max_val_2 = v[n];
                max_index_2 = n as u32;
                guard_index_2 = *k as u32;
            }
        }
    }
    println!("Part 1 : {}",max_index*guard_index);
    println!("Part 2 : {}",max_index_2*guard_index_2);
}
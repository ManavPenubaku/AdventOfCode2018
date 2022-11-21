use aoc2018::util;

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
    for record in guardpost_records{
        println!("{}",record.entry);
    }

}
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Create file
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut max = 0;
    let mut cur_value = 0;
    for line in lines {

        let cur_line:String = line.unwrap();
        
        if !cur_line.is_empty(){
            let converted: i32 = cur_line.parse().unwrap();
            cur_value += converted;
        } else{
            if cur_value > max {
                max = cur_value
            }
            cur_value = 0;
        }
    }
    print!("{}",max)
}

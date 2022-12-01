use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BinaryHeap;
use std::cmp::Reverse;


fn main() {
    // Create file
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut heap = BinaryHeap::new();
    let mut cur_value = 0;
    for line in lines {
        let cur_line:String = line.unwrap();
        if !cur_line.is_empty(){
            let converted: i32 = cur_line.parse().unwrap();
            cur_value += converted;
        } else{
            heap.push(Reverse(cur_value));
            if heap.len() == 4{
                heap.pop();
            }
            cur_value = 0;
        }
    }
    let mut total = 0;
    for x in &heap {
        total += x.0;
        
    }
    println!("{}",total);
}

// Chapter 4 Code

use std::io;

fn read_next_line() -> String {
    let mut line = String::new();
    
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    
    line.trim().to_string()
}

fn parse_line(line: &str) -> String {
    let mut response = String::new();
    
    match line {
        "CPU" => response.push_str("Great choice with the CPU"),
        "Memory" => response.push_str("Can't go wrong with Memory"),
        _ => response.push_str("Whatever you choose can't be that bad!")
    }
    
    response
}

fn print_string_slice(line: &str, start: usize, end: usize) {
    if start <= end && end <= line.len() {
        println!("{}", &line[start..end]);
    } else {
        println!("Invalid bounds!");
    }
}

fn main() {
    let mut count = 2;
    
    while count > 0 {
        let input = read_next_line();
        let output = parse_line(&input);
        
        print_string_slice(output.as_str(), 0, output.len());
        
        // Use saturating_sub to avoid underflow
        print_string_slice(output.as_str(), 10, output.len().saturating_sub(10));
        
        count -= 1;
    }
}

// Chapter 3 Code

use std::io;

fn parse_input(input: &str) -> (bool, String) {
    // Trim the input
    let input = input.trim().to_string();

    if input == "CPU" {
        (true, String::from("Data"))
    } else if input == "Memory" {
        (true, String::from("I/O"))
    } else {
        (false, String::from("Input not handled here!"))
    }
}

fn main() {
    println!("Get system information here");

    let mut count = 0;
    let mut request_tracker: Vec<String> = Vec::new();

    'counting_pulls: loop {
        println!("Which type of information do you want?");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let (worked, output) = parse_input(&line);

        println!("{}", output);
        
        if !worked {
            break 'counting_pulls;
        } else {
            request_tracker.push(output);
        };

        count += 1;
    }

    println!("You pulled system data {} times!\nHere are the requests", count);
    
    for request in &request_tracker {
        println!("{}", request);
    }
    /* Able to do this
        for (index, request) in request_tracker.iter().enumerate() {
            println!("Request {}: {}", index + 1, request);
        }
    */
}

/* Using string literals instead
    fn parse_input(input: &str) -> (bool, &'static str) {
        // Trim the input
        let input = input.trim();

        if input == "CPU" {
            (true, "Data")
        } else if input == "Memory" {
            (true, "I/O")
        } else {
            (false, "Input not handled here!")
        }
    }
*/

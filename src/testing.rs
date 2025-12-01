use std::fs::File;
use std::io;
use std::path::Path;
mod turing_machine;
use crate::turing_machine::{tape::Tape, machine::CTuringMachine};

fn main() {
	//get file input (either .c or .txt file)
	println!("Please enter a file name (either .c or .txt):"); // Prompt the user for input
    let mut user_input = String::new(); 
    io::stdin()
        .read_line(&mut user_input) 
        .expect("Failed to read line");	
	let path = Path::new(user_input);
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    // Read the file contents into a string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
			let my_tape = Tape::new(); //create tape by sending to linked list
			for c in s.chars(){
				my_tape.push_back(c);
			}	
			let tm = CTuringMachine::new(my_tape);  //send tape to TM function
			let result = tm.run();
			if result{
				println!("accepted");
			} else {
				println!("rejected");
			}
		}
    }
}


mod align;

use std::borrow::Borrow;
use std::env::args;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use align::{global_alignment};


fn read_fasta(path: &str) -> String{
    /*
        sequence: variable used to store the entire sequence
        line: variable used to store the current unwrapped line in the for loop
        line_counter: used to signal some error while parsing the file

        this function uses BufReader for efficiency
    */
    let mut sequence = String::new();
    let mut line:String;
    let mut line_counter = 1;

    let file = File::open(path).expect(
        "Something went wrong while reading the file, check the path."
    );
    let reader = BufReader::new(file);


    for l in reader.lines(){
        line = l.expect(&*format!("Something went wrong at line {}", line_counter));

        //If the current line is not a FASTA header then we can add it
        if line.chars().nth(0).unwrap() != '>'{
            //remove carriage return and new line
            if line.ends_with('\n'){
                line.pop();
                if line.ends_with('\r'){
                    line.pop();
                }
            }

            sequence.push_str(&*line);
        }


        line_counter += 1;
    }

    return sequence;
}

fn main() {
    let args: Vec<String> = args().collect();
    let mut first_seq = String::new();
    let mut second_seq = String::new();
    let mut first_align = String::from("");
    let mut second_align = String::from("");

    match args.len(){
        1 => println!("Usage: align path/to/FASTA path/to/FASTA"),
        2 => println!("Missing one argument."),
        3 => {
            first_seq = read_fasta(&args[1]);
            second_seq = read_fasta(&args[2]);


            (first_align, second_align) = global_alignment(first_seq.as_bytes(), second_seq.as_bytes());
            println!("{}\n{}", first_align, second_align)
        }
        _ => println!("Too many arguments."),
    }
}
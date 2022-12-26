use std::env;
use crate::search;



struct Arguments {
    input_file: String
}

pub fn run() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        print_helper();
        std::process::exit(1);
    }
    let args = Arguments{input_file: args[0].clone()};
    execute_search(args);
}

fn execute_search(args: Arguments) {
    let file = match search::generate_file_tree(args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("unable to generate tree due to err: {}", e);
            std::process::exit(1);
        }
    };
}

fn print_helper() {
    println!("shelf shows the child files");
    eprintln!("Usage shelf search <ABSOLUTE INPUT PATH>");
}
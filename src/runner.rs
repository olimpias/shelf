use std::{env, rc::Rc, cell::RefCell};
use colored::Colorize;
use crate::search::{self, TreeNode, File};

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
    let tree_root = match search::generate_file_tree(args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("unable to generate tree due to err: {}", e);
            std::process::exit(1);
        }
    };
    print_nodes(tree_root, String::from(""));
}

fn print_nodes(current: Rc<RefCell<TreeNode<File>>>, path: String) {
    let borrow_current = current.borrow();
    print_file(&borrow_current.val, &path);
    for child in borrow_current.children.iter() {
        let new_path = if path == "" {
            format!("{}", borrow_current.val.file_name)
        } else{
            format!("{}/{}", path, borrow_current.val.file_name)
        };
        print_nodes(Rc::clone(child),new_path );
    }
}

fn print_file(file: &File, path: &String) {
    if file.is_directory {
        if path == "" {
            println!("{}", file.file_name);
        } else {
            println!("{}/{}",path, file.file_name);
        }
    } else {
        if path == "" {
            println!("{}", file.file_name);
        } else {
            println!("{}/{}",path, file.file_name.green());
        }
        
    }
}

fn print_helper() {
    println!("shelf shows the child files");
    eprintln!("Usage shelf search <ABSOLUTE INPUT PATH>");
}
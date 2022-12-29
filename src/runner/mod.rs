use colored::Colorize;
use search::{get_current_working_dir, File, TreeNode};
use std::{cell::RefCell, env, rc::Rc};

mod search;

struct Arguments {
    input_file: String,
}

pub fn run() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        print_helper();
        std::process::exit(1);
    }
    let args = Arguments {
        input_file: args[0].clone(),
    };
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
    print_nodes(tree_root, &get_current_working_dir());
}

fn print_nodes(current: Rc<RefCell<TreeNode<File>>>, path: &String) {
    let borrow_current = current.borrow();
    print_file(&borrow_current.val, &path);
    for child in borrow_current.children.iter() {
        let new_path = if path.is_empty() {
            format!("{}", borrow_current.val.file_name)
        } else {
            let mut file_path = format!("{}/{}", path, borrow_current.val.file_name);
            if borrow_current.val.file_name.is_empty() {
                file_path = path.clone();
            }
            file_path
        };
        print_nodes(Rc::clone(child), &new_path);
    }
}

fn print_file(file: &File, path: &String) {
    if file.is_directory {
        if path == "" {
            println!("{}", file.file_name);
        } else {
            println!("{}/{}", path, file.file_name);
        }
    } else {
        if path == "" {
            println!("{}", file.file_name);
        } else {
            println!("{}/{}", path, file.file_name.green());
        }
    }
}

fn print_helper() {
    println!("shelf shows the child files");
    eprintln!("Usage shelf search <ABSOLUTE INPUT PATH>");
}

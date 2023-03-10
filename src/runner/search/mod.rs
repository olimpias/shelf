use std::cell::RefCell;
use std::env;
use std::fs;
use std::io::Error;
use std::rc::Rc;
pub struct TreeNode<T> {
    pub val: T,
    pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
    pub children: Vec<Rc<RefCell<TreeNode<T>>>>,
}

pub struct File {
    pub is_directory: bool,
    pub file_name: String,
}

pub fn generate_file_tree(input_path: String) -> Result<Rc<RefCell<TreeNode<File>>>, Error> {
    let md = fs::metadata(&input_path)?;
    if !md.is_dir() {
        let root = TreeNode {
            val: File {
                is_directory: md.is_dir(),
                file_name: input_path.clone(),
            },
            parent: None,
            children: Vec::new(),
        };
        return Ok(Rc::new(RefCell::new(root)));
    }


    let initial_file_path: String;
    if input_path.starts_with("./") {
        initial_file_path = input_path.strip_prefix("./").unwrap().to_string();
    } else if input_path.starts_with(".") {
        initial_file_path = input_path.strip_prefix(".").unwrap().to_string();
    } else {
        initial_file_path = input_path.clone();
    }
    let root = Rc::new(RefCell::new(TreeNode {
        val: File {
            is_directory: md.is_dir(),
            file_name: initial_file_path.to_string(),
        },
        parent: None,
        children: Vec::new(),
    }));
    match look_up_children(Rc::clone(&root), input_path) {
        Ok(_) => {}
        Err(err) => {
            return Err(err);
        }
    }
    Ok(root)
}

fn look_up_children(
    current_node: Rc<RefCell<TreeNode<File>>>,
    current_path: String,
) -> Result<(), Error> {
    let paths = fs::read_dir(&current_path)?;
    for path in paths {
        let entry = match path {
            Ok(entry) => entry,
            Err(err) => {
                return Err(err);
            }
        };
        let file_path = entry.file_name().into_string().unwrap();
        let file = File {
            file_name: file_path.clone(),
            is_directory: entry.metadata().unwrap().is_dir(),
        };
        let child_node = TreeNode {
            val: file,
            parent: Some(Rc::clone(&current_node)),
            children: Vec::new(),
        };
        let child_rc_node = Rc::new(RefCell::new(child_node));

        if child_rc_node.clone().borrow().val.is_directory {
            look_up_children(
                Rc::clone(&child_rc_node),
                format!("{}/{}", &current_path, &file_path),
            )?;
        }

        let current_clone = Rc::clone(&current_node);
        current_clone
            .borrow_mut()
            .children
            .push(Rc::clone(&child_rc_node));
    }
    Ok(())
}

pub fn get_current_working_dir() -> String {
    env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_generate_file_tree() {
        let result = generate_file_tree(String::from("test_data"));
        match result {
            Ok(result) => verify_tree_structure(result),
            Err(err) => {
                panic!("unable to generate file tree err: {}", err);
            }
        }
    }

    #[test]
    fn test_generate_file_tree_with_prefix_dot() {
        let result = generate_file_tree(String::from("./test_data"));
        match result {
            Ok(result) => verify_tree_structure(result),
            Err(err) => {
                panic!("unable to generate file tree err: {}", err);
            }
        }
    }

    fn verify_tree_structure(result: Rc<RefCell<TreeNode<File>>>) {
        let mut expected_result_map = HashMap::from([
            (String::from("b-test.txt"), false),
            (String::from("b"), false),
            (String::from("a-test.txt"), false),
            (String::from("a"), false),
            (String::from("test.txt"), false),
            (String::from("test_data"), false),
        ]);

        tree_traverse(&result, &mut expected_result_map);
        for (k, v) in expected_result_map {
            assert!(v, "{} unable to find file", k)
        }
    }

    fn tree_traverse(
        current: &Rc<RefCell<TreeNode<File>>>,
        expected_result_map: &mut HashMap<String, bool>,
    ) {
        let borrow_current_val = current.borrow();
        let value = expected_result_map.get(&borrow_current_val.val.file_name);
        match value {
            Some(f) => {
                assert!(
                    !f,
                    "{} duplicated file detected",
                    &borrow_current_val.val.file_name
                );
                expected_result_map.insert(borrow_current_val.val.file_name.clone(), true);
            }
            None => {
                panic!(
                    "{} unable to find the file",
                    &borrow_current_val.val.file_name
                );
            }
        }

        for child in borrow_current_val.children.iter() {
            tree_traverse(child, expected_result_map);
        }
    }
}

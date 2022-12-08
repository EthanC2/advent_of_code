use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::cell::RefCell;
use std::str::FromStr;
use std::fs;

struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,  
    files: HashMap<String, u64>,
    children: HashMap<String, Rc<RefCell<Directory>>>
}

#[allow(dead_code)]
pub fn part1() -> u64 {
    let file = fs::File::open("input/day7.txt").unwrap();
    let file = BufReader::new(file); 
    let mut root = Rc::new(RefCell::new(
                                            Directory { name: String::from("/"), 
                                            parent: None,                           
                                            files: HashMap::new(), 
                                            children: HashMap::new() 
                                    }));
    let mut working_directory = root;

    for line in file.lines() {
        let line = line.unwrap();

        if line.starts_with("$ cd") {
            let directory_name = line.split_ascii_whitespace().last().unwrap();

            if directory_name == ".." {
                if let Some(parent_dir) = &working_directory.borrow().parent {
                    working_directory = parent_dir.clone();
                }
            } else {
                if working_directory.borrow_mut() {
                    
                }

                // working_directory = *working_directory.borrow_mut().children
                // .entry(String::from(directory_name))
                // .or_insert(Rc::new(RefCell::new(Directory {
                //     name: String::from(directory_name), 
                //     parent: Some(working_directory.clone()),                           
                //     files: HashMap::new(), 
                //     children: HashMap::new() 
                // }))).c;
            }
        
        }
        // } else {
        //     let (size, filename) = line.split_once(' ').unwrap();
        //     let size = u64::from_str(size).unwrap();

        //     working_directory.files.insert(String::from(filename), size);
        // }
    }

    1000
}
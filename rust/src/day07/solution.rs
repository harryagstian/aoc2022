const DAY: &str = "07";

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use itertools::Itertools;

use crate::utils;

#[derive(Debug)]
struct Node {
    name: String,
    size: i32,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(name: String, size: i32) -> Node {
        return Node {
            name,
            size,
            children: vec![],
            parent: None,
        };
    }
}

fn dfs(
    mut data: &mut HashMap<String, i32>,
    node: &Rc<RefCell<Node>>,
    current_path: &String,
) -> i32 {
    // didn't notice the folder name could overlap. we should store full path as data.key
    // thanks reddit!
    // https://www.reddit.com/r/adventofcode/comments/zeuexo/comment/iz8l2jw

    let current = node.borrow();
    let mut sum: i32 = 0;

    // if current node doesn't have child, return current node size
    if current.children.len() == 0 {
        return current.size;
    }

    // if current node has child, call dfs to each children
    for child in current.children.iter() {
        let new_path = format!("{}/{}", current_path, &child.borrow().name);
        sum += dfs(&mut data, &child, &new_path);
    }

    // track sum of each folder in data
    data.insert(current_path.clone(), sum);

    // traverse up
    return sum;
}

pub fn test_results() -> (String, String) {
    let part1 = String::from("95437");
    let part2 = String::from("24933642");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut part1 = 0;
    let mut part2 = 2147483646; // set to max i32 since we want to find lowest value

    let mut mode_add_file = false;
    let root_node: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::new("/".to_string(), 0)));
    let mut current_node: Rc<RefCell<Node>> = Rc::clone(&root_node);

    for line in contents.lines() {
        if mode_add_file && line.starts_with("$") { // if previous line is a dir / file and current line has $, means we are done listing file
            mode_add_file = false;
        }

        if mode_add_file { // dir / file to be created
            let mut new_node: Node;

            if line.starts_with("dir") { // dir has size = 0
                let path = line.replace("dir ", "");

                new_node = Node::new(path.clone(), 0);
                new_node.parent = Some(Rc::clone(&current_node));
            } else { // file has actual size value
                let (size_str, path) = line.split_whitespace().collect_tuple().unwrap();

                new_node = Node::new(path.to_string(), size_str.parse().unwrap());
                new_node.parent = Some(Rc::clone(&current_node));
            }

            // push new_node as child of current_node
            current_node
                    .borrow_mut()
                    .children
                    .push(Rc::new(RefCell::new(new_node)));
        } else if line == "$ cd /" {
            // we already initialized root node
            // do nothing
        } else if line == "$ ls" {
            mode_add_file = true;
        } else if line.contains("$ cd") {
            // traverse to directory
            // means that we need to change current_node pointer
            let path = line.replace("$ cd ", "");
            let new_node: Rc<RefCell<Node>>;

            if path == ".." { // travel to parent node
                new_node = current_node.borrow_mut().parent.as_ref().unwrap().clone();
            } else { // travel to child node
                new_node = current_node
                    .borrow_mut()
                    .children
                    .iter()
                    .find(|&f| f.borrow().name == path)
                    .unwrap()
                    .clone();
            }

            current_node = Rc::clone(&new_node);
        }
    }

    let mut data: HashMap<String, i32> = HashMap::new();
    dfs(&mut data, &root_node, &"/".to_string()); // call to depth-first search

    let needed_storage = 30000000 - (70000000 - data.get("/").unwrap());

    for (_, value) in data {
        if value <= 100000 {
            part1 += value;
        }

        if value >= needed_storage && value < part2 {
            part2 = value
        }
    }
    return (part1.to_string(), part2.to_string());
}

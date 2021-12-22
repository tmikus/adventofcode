use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::{Error, Lines, StdinLock};
use std::ops::Deref;
use std::rc::{Rc, Weak};

struct Node {
    connections: Vec<Weak<RefCell<Node>>>,
    id: u8,
    is_end: bool,
    is_large: bool,
    is_start: bool,
    name: String,
}

impl Node {
    fn parse(id: u8, name: &str) -> Node {
        Node {
            connections: vec![],
            id,
            is_end: name == "end",
            is_large: name.chars().next().unwrap().is_uppercase(),
            is_start: name == "start",
            name: name.to_owned(),
        }
    }
}

struct Graph {
    end_node: Weak<RefCell<Node>>,
    nodes: HashMap<String, Rc<RefCell<Node>>>,
    start_node: Weak<RefCell<Node>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            end_node: Weak::new(),
            nodes: HashMap::new(),
            start_node: Weak::new(),
        }
    }

    fn get_or_create_node(&mut self, name: &str) -> Weak<RefCell<Node>> {
        if let Some(existing_node) = self.nodes.get(name) {
            return Rc::downgrade(existing_node);
        }
        let node = Rc::new(RefCell::new(
            Node::parse(self.nodes.len() as u8, name)
        ));
        let weak_node = Rc::downgrade(&node);
        self.nodes.insert(name.to_owned(), node);
        if name == "start" {
            self.start_node = weak_node.clone();
        } else if name == "end" {
            self.end_node = weak_node.clone();
        }
        weak_node
    }

    fn set_connection(&mut self, line: String) {
        let mut nodes = line.split('-');
        let left_name = nodes.next().unwrap();
        let left = self.get_or_create_node(left_name);
        let right_name = nodes.next().unwrap();
        let right = self.get_or_create_node(right_name);
        {
            let left = left.upgrade().unwrap();
            (*left).borrow_mut().connections.push(right.clone());
        }
        {
            let right = right.upgrade().unwrap();
            (*right).borrow_mut().connections.push(left);
        }
    }
}

fn find_unique_paths_in_graph(graph: &Graph) -> Vec<Vec<u8>> {
    let node = Weak::upgrade(&graph.start_node).unwrap();
    find_path(node, vec![], false)
}

fn find_path(
    node: Rc<RefCell<Node>>,
    mut visited_nodes: Vec<u8>,
    mut has_visited_small_twice: bool,
) -> Vec<Vec<u8>> {
    let node_ref = (*node).borrow();
    if !node_ref.is_large && !node_ref.is_start && visited_nodes.contains(&node_ref.id) {
        has_visited_small_twice = true;
    }
    visited_nodes.push(node_ref.id);
    if node_ref.is_end {
        return vec![visited_nodes];
    }
    let mut results = vec![];
    for child_weak in &node_ref.connections {
        let child = Weak::upgrade(child_weak).unwrap();
        {
            let child_ref = (*child).borrow();
            if child_ref.is_start {
                continue;
            }
            if !child_ref.is_large
            && has_visited_small_twice
            && visited_nodes.contains(&child_ref.id) {
                continue;
            }
        }
        let mut child_results = find_path(
            child,
            visited_nodes.clone(),
            has_visited_small_twice,
        );
        results.append(&mut child_results);
    }
    results
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut graph = Graph::new();
    for line in lines {
        let line = line.unwrap();
        graph.set_connection(line);
    }
    let paths = find_unique_paths_in_graph(&graph);
    println!("Count: {}", paths.len());
}

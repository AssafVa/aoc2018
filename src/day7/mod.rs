use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap, BTreeSet, BTreeMap};
use std::rc::Rc;

use super::utils::IterableInput;

impl<'a> IterableInput<'a> {
    fn get_coordinates(&self) -> impl Iterator<Item=(char, char)> {
        let file = File::open(&self.path()).unwrap();
        let mut reader = BufReader::new(file);

        let test_lines = vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ];

        let it = reader.lines().map(|s| s.unwrap());
//        let it = test_lines.into_iter();
        it
            .map(|s| {
                let mut splitted = s.split(" ");
                splitted.next();
                let parent = splitted.next().unwrap().chars().next().unwrap();
                splitted.next();
                splitted.next();
                splitted.next();
                splitted.next();
                splitted.next();
                let child = splitted.next().unwrap().chars().next().unwrap();
                (parent, child)
            })
    }
}

struct Node {
    parents: Vec<Rc<Node>>,
    node: char,
    children: Vec<Rc<Node>>
}
//
//fn build_graph(relationships: Vec<(char, char)>) -> Vec<Rc<Node>> {
//    let node_map = HashMap::new();
//
//    for (parent, child) in relationships.iter() {
//
//    }
//}

fn day7a(relationships: Vec<(char, char)>) {
    let mut roots = BTreeSet::new();
    let mut not_parents = HashSet::new();
//    let mut node_map = HashMap::new();

//    let mut parent_to_children = BTreeMap::new();

    for (parent, child) in relationships.iter() {
        if !not_parents.contains(&parent) {
            roots.insert(parent);
        }
        not_parents.insert(child);
        roots.remove(&child);

//        let mut children = parent_to_children.entry(parent).or_insert(BTreeSet::new());
//        children.insert(child);
        println!("{} -> {}", parent, child);
    }

    println!("{:?}", roots);




//    for (parent, child) in relationships {
//        println!("{} -> {}", parent, child);
//    }
}

fn day7b() {

}

pub fn day7() {
    let mut relationships = IterableInput::new("/home/assaf/dev/code/tmp/aocDay7.txt").get_coordinates();
    let relationships: Vec<(char, char)> = relationships.collect();

    day7a(relationships);
    day7b();
}
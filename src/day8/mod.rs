use std::fs::File;
use std::io::{BufReader, BufRead, Read};

use super::utils::IterableInput;

impl<'a> IterableInput<'a> {
    fn get_data(&self) -> Vec<u8> {
        let file = File::open(&self.path()).unwrap();
        let mut reader = BufReader::new(file);

//        let s = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string();
//
        let mut s = String::new();
        reader.read_to_string(&mut s);

        s.split(" ").map(|n| n.parse::<u8>().unwrap()).collect()
    }
}

#[derive(Debug)]
struct Node {
    metadata: Vec<u8>,
    children: Vec<Node>
}

fn build_tree(slice: &[u8]) -> (Node, usize) {
    let num_children = slice[0] as usize;
    let len_metadata = slice[1] as usize;
    let mut offset = 2;

    let mut children = Vec::new();

    for i in 0..num_children {
        let (child, additional_offset) = build_tree(&slice[offset..]);
        offset += additional_offset;
        children.push(child);
    }
    let metadata: Vec<u8> = slice[offset..(offset+len_metadata)].to_vec();
    offset += len_metadata;
    (Node { children, metadata }, offset)
}

fn print_tree(node: &Node, indent: usize) {
    println!("{:indent$}{:?}", "", node.metadata, indent=indent);
    for child in node.children.iter() {
        print_tree(child, indent + 2);
    }
}

fn sum_metadata(node: &Node) -> u32 {
    let mut sum = node.metadata.iter().map(|i| *i as u32).sum();
    for child in node.children.iter() {
        sum += sum_metadata(child);
    }
    sum
}

fn sum_metadata2(node: &Node) -> u32 {
    println!("{:?}", node);
    if node.children.is_empty() {
        node.metadata.iter().map(|i| *i as u32).sum()
    } else {
        let mut sum = 0;
        for num in node.metadata.iter() {
            let num = (*num as usize) - 1;
            if num < node.children.len() {
                sum += sum_metadata2(&node.children[num]);
            }
        }
        sum
    }
}

fn day8a(root: &Node) {
//    let (root, _) = build_tree(&nums);
    println!("day8a metadata sum: {}", sum_metadata(&root));
//    println!("{:?}", root);
}

fn day8b(root: &Node) {
    println!("day8b metadata sum: {}", sum_metadata2(&root));
}

pub fn day8() {
    let mut nums = IterableInput::new("/home/assaf/dev/code/tmp/aocDay8.txt").get_data();
    let (root, _) = build_tree(&nums);
    print_tree(&root, 0);
    day8a(&root);
    day8b(&root);
}

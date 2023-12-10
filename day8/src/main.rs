use std::{cell::RefCell, rc::Rc};

#[derive(Debug,Clone)]
enum Dirs {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    id: String,
    left: usize,
    right: usize,
}

type NodeRef = Rc<RefCell<Node>>;

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let move_chars = contents.lines().collect::<Vec<&str>>()[0].chars();

    let mut dir_seq: Vec<Dirs> = Vec::new();
    for c in move_chars {
        let dir = match c {
            'L' => Dirs::Left,
            'R' => Dirs::Right,
            _ => panic!("found unexpectd move char"),
        };
        dir_seq.push(dir);
    }

    let lines: Vec<&str> = contents.lines().collect();

    let mut node_strings: Vec<(String, String, String)> = Vec::new();

    let mut nodes: Vec<Node> = Vec::new();

    for line in &lines[2..] {
        let (id, children) = line.split_once("=").unwrap();
        let id_str = id.replace(" ", "");

        let children = children.replace(" ", "");
        let children = children.replace("(", "");
        let children = children.replace(")", "");
        let (left, right) = children.split_once(",").unwrap();

        node_strings.push((id_str, left.to_owned(), right.to_owned()));
    }

    // make nodes
    for node in &node_strings {
        nodes.push(Node {
            id: node.0.clone(),
            left: 0,
            right: 0,
        });
    }


    //build connections between nodes
    for i in 0..nodes.len() {
        for j in 0..node_strings.len() {
            if node_strings[j].0 == nodes[i].id {
                for k in 0..nodes.len() {
                    //left
                    if node_strings[j].1 == nodes[k].id {
                        nodes[i].left = k;
                    }
                    //right
                    if node_strings[j].2 == nodes[k].id {
                        nodes[i].right = k;
                    }
                }
            }
        }
    }

    let mut pos = 0;
    for i in 0..nodes.len() {
        if nodes[i].id == String::from("AAA") {
            pos = i;
        }
    }



    //walk the graph
    let mut steps = 0;

    for step in dir_seq.into_iter().cycle() {
        pos = match step {
            Dirs::Left => nodes[pos].left,
            Dirs::Right => nodes[pos].right,
        };
        steps += 1;
        if nodes[pos].id == String::from("ZZZ") {
            break;
        }
    }




    println!("{:?}", nodes);
    println!("{:?}", nodes.len());
    println!("{:?}", node_strings.len());

    println!("Steps taken: {}", steps);
}

#[derive(Debug, Clone)]
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

    let mut count_of_starts = 0;

    let mut pos_vec: Vec<usize> = Vec::new();
    let mut steps_vec: Vec<u32> = Vec::new();

    for i in 0..nodes.len() {
        if nodes[i].id.chars().last().unwrap() == 'A' {
            pos_vec.push(i);
            steps_vec.push(0);
            count_of_starts += 1;
        }
    }

    for (p, mut pos) in pos_vec.into_iter().enumerate() {
        //walk the graph
        let mut steps = 0;

        for step in dir_seq.clone().into_iter().cycle() {
            pos = match step {
                Dirs::Left => nodes[pos].left,
                Dirs::Right => nodes[pos].right,
            };
            steps += 1;
            if nodes[pos].id.chars().last().unwrap() == 'Z' {
                steps_vec[p] = steps;
                break;
            }
        }
    }

    println!("{:?}", nodes);
    println!("{:?}", nodes.len());
    println!("{:?}", node_strings.len());

    println!("Steps taken: {:?}", steps_vec);
    println!("starts: {}", count_of_starts);
    //TODO(alb): Get LCM of steps_vec. LCM in rust takes 2 inputs. Rather than recursing or doing
    //something clever and functional, just print the vec and and get lcm from python. I'm tired.
}

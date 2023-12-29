fn main() {
    let input = include_str!("input.txt");
    let output = solution(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Node {
    node: String,
    left: String,
    right: String,
}

fn solution(input: &str) -> u32 {
    let lines: Box<[&str]> = input.lines().collect::<Vec<_>>().into_boxed_slice();

    let instructions: Box<[char]> = get_instructions(&lines);
    // println!("instructions:\n{:?}", instructions);
    let network: Box<[Node]> = get_network(lines);
    // println!("network:\n{:?}", network);

    let mut amount_steps = 0;

    let mut current_nodes = network
        .iter()
        .filter(|node| node.node.ends_with("A"))
        .map(|node| node.clone())
        .collect::<Box<[Node]>>();

    let mut instruction_index = 0;

    // let mut current_node = network[*current_node_index].clone();
    // println!("first one:\n{:?}", current_node);

    let instructions_len = &instructions.len().clone();

    // let max_steps = get_next_node(network, *first_node_index, 0, 0, instructions);

    while !is_all_final_nodes(&current_nodes) {
        // println!("instruction index:\n{:?}", next_instruction_index);

        current_nodes = get_next_nodes(
            network.clone(),
            current_nodes,
            instruction_index,
            instructions.clone(),
        );

        instruction_index = get_next_instruction_index(*instructions_len, instruction_index);
        amount_steps += 1;
    }

    amount_steps
}

fn get_instructions<'a>(lines: &Box<[&str]>) -> Box<[char]> {
    lines[0]
        .trim()
        .chars()
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn get_network(lines: Box<[&str]>) -> Box<[Node]> {
    let mut network: Vec<Node> = Vec::new();

    lines.iter().skip(2).for_each(|line| {
        let splitted: Vec<_> = line.split("=").collect();

        let directions: Vec<_> = splitted[1].split(",").collect();
        network.push(Node {
            node: String::from(splitted[0].trim()),
            left: String::from(directions[0].replace("(", "").trim()),
            right: String::from(directions[1].replace(")", "").trim()),
        })
    });

    network.into_boxed_slice()
}

fn get_next_nodes(
    network: Box<[Node]>,
    mut current_nodes: Box<[Node]>,
    instruction_index: u32,
    instructions: Box<[char]>,
) -> Box<[Node]> {
    // let mut current_node = network[next_node_index].clone();
    // println!("current:\n{:?}", current_node);
    // println!(
    //     "instruction:\n{:?}",
    //     instructions[instruction_index as usize]
    // );

    let mut next_nodes = Vec::new();

    current_nodes.iter().for_each(|c_node| {
        if instructions[instruction_index as usize] == 'L' {
            let next_node = network
                .iter()
                .find(|node| node.node == c_node.left)
                .unwrap()
                .clone();

            next_nodes.push(next_node);
        } else {
            let next_node = network
                .iter()
                .find(|node| node.node == c_node.right)
                .unwrap()
                .clone();
            next_nodes.push(next_node);
        }
    });

    // println!("next:\n{:?}", current_node);

    next_nodes.into_boxed_slice()
}

fn is_all_final_nodes(nodes: &Box<[Node]>) -> bool {
    nodes.iter().all(|node| node.node.ends_with("Z"))
}

fn get_next_instruction_index(instructions_len: usize, current_index: u32) -> u32 {
    let max = instructions_len - 1;
    // println!("max:\n{:?}", max);

    if current_index >= max.try_into().unwrap() {
        // println!("reset to 0:\n{:?}", current_index);
        0
    } else {
        // println!("received:\n{:?}", current_index);
        // println!("increase:\n{:?}", current_index + 1);
        current_index + 1
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution(
            "LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)",
        );

        assert_eq!(result, 6);
    }
}

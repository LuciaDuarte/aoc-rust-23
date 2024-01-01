use num::integer::lcm;

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

fn solution(input: &str) -> i64 {
    let lines: Box<[&str]> = input.lines().collect::<Vec<_>>().into_boxed_slice();

    let instructions: Box<[char]> = get_instructions(&lines);
    // println!("instructions:\n{:?}", instructions);
    let network: Box<[Node]> = get_network(lines);
    // println!("network:\n{:?}", network);

    let mut amount_steps: Vec<i64> = Vec::new();

    let current_nodes = network
        .iter()
        .filter(|node| node.node.ends_with("A"))
        .map(|node| node.clone())
        .collect::<Box<[Node]>>();

    current_nodes.iter().for_each(|node| {
        let steps = solution_part1(&network, &instructions, node.clone());
        amount_steps.push(steps.into());
    });

    amount_steps.sort_by(|a, b| b.cmp(&a));
    println!("amount:\n{:?}", amount_steps);

    let mut final_value = amount_steps[0];

    for (index, _steps) in amount_steps.iter().enumerate() {
        if amount_steps.len() - 1 > index {
            final_value = lcm(final_value, amount_steps[index + 1]);
        }
    }
    final_value

    // 12833235391111
}

fn solution_part1(network: &Box<[Node]>, instructions: &Box<[char]>, current_node: Node) -> u32 {
    let mut current_node = current_node;
    let mut amount_steps = 0;

    let mut instruction_index = 0;

    let instructions_len = &instructions.len().clone();

    while !is_final_node(&current_node) {
        current_node = get_next_node(
            network.clone(),
            current_node,
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

fn get_next_node(
    network: Box<[Node]>,
    mut current_node: Node,
    instruction_index: u32,
    instructions: Box<[char]>,
) -> Node {
    let next_node_index;

    if instructions[instruction_index as usize] == 'L' {
        next_node_index = network
            .iter()
            .position(|node| node.node == current_node.left)
            .unwrap();
    } else {
        next_node_index = network
            .iter()
            .position(|node| node.node == current_node.right)
            .unwrap();
    }

    current_node = network[next_node_index].clone();

    current_node
}

fn is_final_node(node: &Node) -> bool {
    node.node.ends_with("Z")
}

fn get_next_instruction_index(instructions_len: usize, current_index: u32) -> u32 {
    let max = instructions_len - 1;

    if current_index >= max.try_into().unwrap() {
        0
    } else {
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

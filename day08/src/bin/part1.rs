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
    let network: Box<[Node]> = get_network(lines);

    let mut amount_steps = 0;

    let current_node_index = &network
        .iter()
        .position(|node| node.node == String::from("AAA"))
        .unwrap();

    let mut instruction_index = 0;

    let mut current_node = network[*current_node_index].clone();

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
    if node.node == String::from("ZZZ") {
        true
    } else {
        false
    }
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
        let result1 = solution(
            "RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)",
        );
        let result2 = solution(
            "LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result1, 2);
        assert_eq!(result2, 6);
    }
}

// Problem #79: Passcode Derivation
// https://projecteuler.net/problem=79

use std::collections::{HashMap, HashSet, VecDeque};

fn build_graph(sequences: Vec<&str>) -> HashMap<char, HashSet<char>> {
    let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
    let mut in_degree: HashMap<char, usize> = HashMap::new();

    for seq in &sequences {
        for ch in seq.chars() {
            graph.entry(ch).or_insert(HashSet::new());
            in_degree.entry(ch).or_insert(0);
        }
    }

    for seq in sequences {
        let chars: Vec<char> = seq.chars().collect();
        for i in 0..chars.len() - 1 {
            let from = chars[i];
            let to = chars[i + 1];
            if graph.get(&from).unwrap().insert(to) {
                *in_degree.get_mut(&to).unwrap() += 1;
            }
        }
    }

    (graph, in_degree)
}

fn topological_sort(graph: HashMap<char, HashSet<char>>, in_degree: HashMap<char, usize>) -> String {
    let mut queue = VecDeque::new();
    let mut result = String::new();
    
    for (&node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    while let Some(node) = queue.pop_front() {
        result.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                let count = in_degree[&neighbor] - 1;
                in_degree.insert(neighbor, count);
                if count == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    result
}

fn main() {
    let sequences = vec![
        "319", "680", "180", "690", "129", "620", "762", "689", "762", 
        "318", "368", "710", "720", "710"
    ];

    let (graph, in_degree) = build_graph(sequences);

    let passcode = topological_sort(graph, in_degree);

    println!("The shortest passcode is {}", passcode);
}

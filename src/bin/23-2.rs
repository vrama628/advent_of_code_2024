use std::io::stdin;

use im::{HashMap, HashSet};

struct Graph(HashMap<String, HashSet<String>>);

impl Graph {
    fn parse() -> Self {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        for line in stdin().lines().map(Result::unwrap) {
            let (a, b) = line.split_once("-").unwrap();
            debug_assert_ne!(a, b);
            graph.entry(a.to_owned()).or_default().insert(b.to_owned());
            graph.entry(b.to_owned()).or_default().insert(a.to_owned());
        }
        Self(graph)
    }

    fn clique(&self, node: &String, candidates: &HashSet<String>) -> Vec<String> {
        if candidates.is_empty() {
            vec![node.clone()]
        } else {
            let mut remaining_candidates = candidates.clone();
            let mut best = vec![];
            for candidate in candidates {
                remaining_candidates.remove(candidate).unwrap();
                let clique = self.clique(candidate, &(&remaining_candidates * &self.0[candidate]));
                if clique.len() > best.len() {
                    best = clique;
                }
            }
            best.push(node.clone());
            best
        }
    }
}

fn main() {
    let graph = Graph::parse();
    let mut visited = HashSet::new();
    let mut result = graph
        .0
        .iter()
        .map(|(node, edges)| {
            visited.insert(node.clone());
            graph.clique(node, &edges.clone().relative_complement(visited.clone()))
        })
        .max_by_key(|c| c.len())
        .unwrap();
    result.sort();
    println!("{}", result.join(","));
}

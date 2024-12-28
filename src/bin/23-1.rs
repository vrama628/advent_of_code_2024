use std::io::stdin;

use im::{HashMap, HashSet};

fn main() {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for line in stdin().lines().map(Result::unwrap) {
        let (a, b) = line.split_once("-").unwrap();
        debug_assert_ne!(a, b);
        graph.entry(a.to_owned()).or_default().insert(b.to_owned());
        graph.entry(b.to_owned()).or_default().insert(a.to_owned());
    }
    let result = graph
        .iter()
        .filter(|(node, _)| node.starts_with("t"))
        .flat_map(|(node, edges)| {
            edges.iter().flat_map(|neighbor| {
                graph[neighbor]
                    .clone()
                    .intersection(edges.clone())
                    .into_iter()
                    .map(|third| {
                        let mut v = vec![node.clone(), neighbor.clone(), third];
                        v.sort();
                        v
                    })
            })
        })
        .collect::<HashSet<Vec<String>>>()
        .len();
    println!("{result}");
}

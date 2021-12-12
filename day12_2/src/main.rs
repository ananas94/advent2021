use std::io;
use std::error::Error;
use std::collections::HashMap;


type Graph = HashMap<String, Vec<String>>;

fn graph_add(graph: &mut Graph, a:String, b:String)   {
    if ! graph.contains_key(&a) { graph.insert( a.clone(), Vec::new() ); }
    if ! graph.contains_key(&b) { graph.insert( b.clone(), Vec::new() ); }

    if let Some(v) = graph.get_mut(&a) { v.push(b.clone()); }
    if let Some(v) = graph.get_mut(&b) { v.push(a.clone()); }
}

fn read_next_graph_edge() -> Option<(String,String)> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok()?;
    let mut line_iter = inp.split('-');
    let a = line_iter.next()?.trim();
    let b = line_iter.next()?.trim();
    Some((a.to_string(),b.to_string()))
}

fn ways_to_end(graph: &Graph, current: &String, history: & mut Vec<String>, there_was_special_small: bool) -> u64 {
    if *current == "start" && history.len() != 0 {
        return 0;
    }
    if *current == "end" {
        return 1;
    }
    let first_time = history.iter().find(|&x| x==current ) == None ;
    let big_cave =  current.find(char::is_lowercase) == None ;
    if first_time || big_cave || !there_was_special_small {
        let edges = graph.get(current).unwrap();
        let mut sum = 0;
        history.push(current.to_string());
        for edge in edges {
            if (!first_time && !big_cave)    {
                sum += ways_to_end(graph, edge, history, true);
            }
            else {
                sum += ways_to_end(graph, edge, history, there_was_special_small);
            }
        }
        history.pop();
    
       return sum;    
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut graph: Graph = HashMap::new();
    while let Some((a,b)) = read_next_graph_edge() {
        graph_add(&mut graph,a,b);
    }

    let mut history = Vec::new();

    let n = ways_to_end(&graph, &("start".to_string()), & mut history, false);
    println!("{}", n);



    Ok(())
}

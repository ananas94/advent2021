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

fn ways_to_end(graph: &Graph, current: &String, history: & mut Vec<String>) -> u64 {
    println!("ways called from {}", current);
    if *current == "end" {
        return 1;
    }
    if history.iter().find(|&x| x==current ) == None || current.find(char::is_lowercase) == None {
        println!("current node: {}, path history is:", current);
        history.iter().for_each(|x| { println!("{} ", x); });
        let edges = graph.get(current).unwrap();
        let mut sum = 0;
        history.push(current.to_string());
        for edge in edges {
            sum += ways_to_end(graph, edge, history);
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

    for (i,v) in &graph {
        println!("node {} has following connections:", i);
        for j in v {
            print!("{} ",j);
        }
        println!("");
    }

    let mut history = Vec::new();

    let n = ways_to_end(&graph, &("start".to_string()), & mut history);
    println!("{}", n);



    Ok(())
}

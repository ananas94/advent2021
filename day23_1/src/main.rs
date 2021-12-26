
use std::collections::HashMap;
use std::collections::HashSet;

use std::collections::BinaryHeap;
use std::cmp::Ordering;

type State = [[char;11];3];
type Graph = HashMap<State,HashSet<(State,u64)>>;

fn get_room(c: char) -> usize {
    match c 
    {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
         _ => unreachable!()
    }
}

fn get_move_price(c: char) -> u64 {
    match c 
    {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
         _ => unreachable!()
    }
}

fn check_room_valid(state: &State, i:usize) -> bool {
    let index = get_room(state[0][i]);
    return state[1][index] == '.' && (state[2][index] == '.' || state[2][index] == state[0][i]) 
}

fn check_room_reachable(state: &State, i:usize) -> bool {
    let index = get_room(state[0][i]);
    let a = i.min(index);
    let b = i.max(index);
    for it in a+1..b {
        if state[0][it] != '.' { return false; }
    }
    true
}

fn move_to_room(state: &mut State, i: usize) -> u64 {
    let letter = state[0][i];
    let move_price = get_move_price(state[0][i]);
    let index = get_room(state[0][i]);
    state[0][i] = '.';
    let a = i.min(index) as u64;
    let b = i.max(index) as u64;
    let mut room_price = 0;
    if state[2][index] == '.' { 
        room_price = 2;
        state[2][index] = letter;
    }
    else {
        room_price = 1;
        state[1][index] = letter;
    } 
    move_price*(b-a + room_price)
}

fn move_from_coridor(state: &State) -> Vec<(State, u64)> {
    let mut ret = Vec::new();
    for i in 0..state[0].len() {
      if state[0][i] != '.' && check_room_reachable(state,i) && check_room_valid(state,i) {
            let mut new_state = *state;
            let price = move_to_room(&mut new_state, i);
            ret.push((new_state, price));
       }
    }
    ret
}

fn has_no_others_behind(state: &State, i:usize, j:usize) -> bool {
    if j == 1 && state[2][i] != state[1][i]  {return false;}
    return true;
}

fn final_dest(state: &State, i:usize, j:usize) -> bool {
    let room_index =  get_room(state[j][i]);
    if room_index == i && has_no_others_behind(state,i,j) { return true; }
    else { return false; }
}

fn could_go_out(state: &State, i: usize, j:usize) -> bool {
    if j == 1 { return true;} 
    if j ==2 && state[1][i] == '.' { return true; }
    false 
}

fn move_from_room(state: &mut State, i:usize, j:usize, target: usize) -> u64 {

    let letter = state[j][i];
    let move_price = get_move_price(state[j][i]);
    state[j][i]='.';

    let a = i.min(target) as u64;
    let b = i.max(target) as u64;

    let mut room_price = 0;

    if j==2 { 
        room_price = 2;
    }
    else {
        room_price = 1;
    } 
    state[0][target] = letter;

    move_price*(b-a + room_price)
}

fn is_reachable(state: &State, k: usize, i: usize) -> bool {
    let a = k.min(i);
    let b = k.max(i);
    for it in a..=b {
       if state[0][it]!='.' { return false; }
    }
    true
}


fn move_to_coridor(state: &State) -> Vec<(State, u64)> {
    let mut ret = Vec::new();
    for i in [2,4,6,8] {
        for j in [1,2] {
            if state[j][i] != '.' && !final_dest(state,i,j) && could_go_out(state,i,j) {
                for k in [0,1,3,5,7,9,10] {
                    if is_reachable(state, k, i) {
                        let mut new_state = *state;
                        let price = move_from_room(&mut new_state,i,j,k);
                        ret.push((new_state,price));
                    }
                }
            }
        }
    }
    ret
}

fn generate_graph(state: &State, graph: &mut Graph) {

    graph.entry(*state).or_insert(HashSet::new());
    if let Some(hs)=graph.get(state) {
        if hs.len() != 0 { return; } 
    }

    let move_from_room_states = move_to_coridor(state);
    for edge in move_from_room_states {
        let (s,p) =edge;
        if let Some(hs)=graph.get_mut(state) {
            hs.insert(edge);
        }

        generate_graph(&s,graph);
    }

    let move_from_coridor_states = move_from_coridor(state);
    for edge in move_from_coridor_states {
        let (s,p) = edge;
        if let Some(hs)=graph.get_mut(state) {
            hs.insert(edge);
        }

        generate_graph(&s,graph);
    }
}


fn print_state(state:& State) {
    for i in 0..state.len() {
        for j in 0..state[0].len() {
            print!("{}",state[i][j]);
        }
        println!("");
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct QState{
    st: State,
    price: u64
}

impl Ord for QState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.price.cmp(&self.price)    
    }
}

impl PartialOrd for QState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}



fn main() {
    let inital_state: State = [
        ['.';11],
        ['#','#','B','#','C','#','B','#','D','#','#'],
        ['#','#','A','#','D','#','C','#','A','#','#']

        /*['#','#','A','#','D','#','A','#','B','#','#'],
        ['#','#','B','#','C','#','D','#','C','#','#']*/
//###A#D#A#B###
//  #B#C#D#C#
//
    ];
    let final_state: State = [
        ['.';11],
        ['#','#','A','#','B','#','C','#','D','#','#'],
        ['#','#','A','#','B','#','C','#','D','#','#']
    ];
    let mut graph = HashMap::new();
    generate_graph(&inital_state, &mut graph);

    /*
    for (s,hs) in &graph {
        print_state(s);
        println!("");
    }*/

    println!("graph build finished");
    println!("graph node num {}", graph.len());

    let f = graph.get(&final_state).unwrap();
    let mut queue = BinaryHeap::new();
    queue.push( QState{st:inital_state,price:0} );

    println!("look for path");

    let mut distances = HashMap::new();
    distances.insert(inital_state, 0);

    while let Some(h) = queue.pop() {
        let state = h.st;
        let price = h.price;
        let hs = graph.get(&state).unwrap();

        for (s,p) in hs {
            let d = distances.get(s);
            if d == None {
                distances.insert(*s, p+price);
                queue.push(QState{st:*s,price: p+price}); 
            } else {
                let prev_price = d.unwrap();
                if *prev_price > (p+price) {
                    distances.insert(*s, p+price);
                    queue.push(QState{st:*s,price: p+price}); 
                }
            }
        }
    }
   let f = distances.get(&final_state).unwrap();
    println!("final state price: {}", f);

}

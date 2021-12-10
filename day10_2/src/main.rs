use std::io;
use std::error::Error;

fn is_symbol_illegal(open_opt: Option<char>, close: char) ->bool {
    if open_opt == None { return true; }
    let open = open_opt.unwrap();
    if  open == '(' && close == ')' ||
        open == '[' && close == ']' ||
        open == '{' && close == '}' ||
        open == '<' && close == '>' {
            return false;
    }
    true
}

fn symbol_score( c: char) -> u64 {
    match c{
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut inp = String::new();
    let mut points_vec = Vec::new();
    while let Ok(len) = io::stdin().read_line(&mut inp) {
        if len == 0 { break; }
        let mut symbol_stack = Vec::new();
        let mut legal = true;
        for c in inp.chars() {
            match c {
                '('|'['|'{'|'<' => { symbol_stack.push(c); },
                ')'|']'|'}'|'>' => { 
                    let open_symbol = symbol_stack.pop(); 
                    if is_symbol_illegal(open_symbol, c) {
                        legal = false;
                        break;
                    }
                },
                _ => {}
            }
        }   
        if legal {
            let mut points = 0;
            while let Some(c) = symbol_stack.pop() {
                points *= 5;
                points += symbol_score(c)
            }
            points_vec.push(points);
        }
        inp = String::new();
    }
    points_vec.sort();

    let mid = points_vec.len()/2;
    println!("mid {}", points_vec[mid] );
    

    Ok(())
}

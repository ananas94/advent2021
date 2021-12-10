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

fn symbol_score(c :char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut inp = String::new();
    let mut points = 0;
    while let Ok(len) = io::stdin().read_line(&mut inp) {
        if len == 0 { break; }
        let mut symbol_stack = Vec::new();
        for c in inp.chars() {
            match c {
                '('|'['|'{'|'<' => { symbol_stack.push(c); },
                ')'|']'|'}'|'>' => { 
                    let open_symbol = symbol_stack.pop(); 
                    if is_symbol_illegal(open_symbol, c) {
                        points += symbol_score(c);
                        break;
                    }
                },
                _ => {}
            }
        }   
        inp = String::new();
    }
    println!("{}", points);
    

    Ok(())
}

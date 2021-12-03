use std::io;
use std::error::Error;

fn GetOxMostPopular(strings: &Vec<&String>,pos: usize) -> char {
    let mut counter = 0;
    for s in strings {
        counter += match s.chars().nth(pos).unwrap() {
            '1' => 1,
            '0' => 0,
            _ => panic!("waaat")
        }
    }
    if counter >= (strings.len() - counter) { '1'}
    else { '0' }
}

fn GetNextOxygenApproximation<'a>(strings: &Vec<&'a String>, pos: usize) -> Vec<&'a String> {
    let f = GetOxMostPopular(strings, pos);
    let answer : Vec<&String> = strings.iter().filter(|s| s.chars().nth(pos).unwrap() == f).map(|s| *s).collect();
    answer
}


fn GetOxygen<'a>(strings: &Vec<&'a String>) -> &'a String {
    let len = strings[0].len();
    let mut aprox: Vec<&'a String> = strings.to_vec();
    for i in 0..len {
        aprox = GetNextOxygenApproximation(&aprox, i);
        if aprox.len()==1 { break; }
    }
    aprox[0]
}


fn GetCo2MostPopular(strings: &Vec<&String>,pos: usize) -> char {
    let mut counter = 0;
    for s in strings {
        counter += match s.chars().nth(pos).unwrap() {
            '1' => 1,
            '0' => 0,
            _ => panic!("waaat")
        }
    }
    if counter < (strings.len() - counter) { '1'}
    else { '0' }
}

fn GetNextCo2Approximation<'a>(strings: &Vec<&'a String>, pos: usize) -> Vec<&'a String> {
    let f = GetCo2MostPopular(strings, pos);
    let answer : Vec<&String> = strings.iter().filter(|s| s.chars().nth(pos).unwrap() == f).map(|s| *s).collect();
    answer
}


fn GetCo2<'a>(strings: &Vec<&'a String>) -> &'a String {
    let len = strings[0].len();
    let mut aprox: Vec<&'a String> = strings.to_vec();
    for i in 0..len {
        aprox = GetNextCo2Approximation(&aprox, i);
        if aprox.len()==1 { break; }
    }
    aprox[0]
}




fn main() ->Result<(),Box<dyn Error> >{
    let mut strings: Vec<String> = vec![];
    loop {
        let mut inp = String::new();
        let str = match io::stdin().read_line(&mut inp) {
            Ok(_) => inp.trim().to_string(),
            Err(_) => break,
        };
        if str.len() == 0  { break; }
        strings.push(str);
    }

    let inputs: Vec<&String> = strings.iter().collect();
    let ox = GetOxygen(&inputs);
    let co2 = GetCo2(&inputs);

    let mut oxInt = 0;
    let mut co2Int = 0;
    for i in 0..ox.len() {
        oxInt *=2;
        co2Int *=2;
        if ox.chars().nth(i).unwrap() == '1' { oxInt += 1; }
        if co2.chars().nth(i).unwrap() == '1' { co2Int += 1; }
    }
    println!("ox {} co2 {}", oxInt, co2Int);
    println!("answ {}", oxInt * co2Int);
    Ok(())
}

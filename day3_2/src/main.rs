use std::error::Error;
use std::io;

fn get_ox_most_popular(strings: &Vec<&String>, pos: usize) -> char {
    let mut counter = 0;
    for s in strings {
        counter += match s.chars().nth(pos).unwrap() {
            '1' => 1,
            '0' => 0,
            _ => panic!("waaat"),
        }
    }
    if counter >= (strings.len() - counter) {
        '1'
    } else {
        '0'
    }
}

fn get_next_oxygen_approximation<'a>(strings: &Vec<&'a String>, pos: usize) -> Vec<&'a String> {
    let f = get_ox_most_popular(strings, pos);
    let answer: Vec<&String> = strings
        .iter()
        .filter(|s| s.chars().nth(pos).unwrap() == f)
        .map(|s| *s)
        .collect();
    answer
}

fn get_oxygen<'a>(strings: &Vec<&'a String>) -> &'a String {
    let len = strings[0].len();
    let mut aprox: Vec<&'a String> = strings.to_vec();
    for i in 0..len {
        aprox = get_next_oxygen_approximation(&aprox, i);
        if aprox.len() == 1 {
            break;
        }
    }
    aprox[0]
}

fn get_co2_most_popular(strings: &Vec<&String>, pos: usize) -> char {
    let mut counter = 0;
    for s in strings {
        counter += match s.chars().nth(pos).unwrap() {
            '1' => 1,
            '0' => 0,
            _ => panic!("waaat"),
        }
    }
    if counter < (strings.len() - counter) {
        '1'
    } else {
        '0'
    }
}

fn get_next_co2_approximation<'a>(strings: &Vec<&'a String>, pos: usize) -> Vec<&'a String> {
    let f = get_co2_most_popular(strings, pos);
    let answer: Vec<&String> = strings
        .iter()
        .filter(|s| s.chars().nth(pos).unwrap() == f)
        .map(|s| *s)
        .collect();
    answer
}

fn get_co2<'a>(strings: &Vec<&'a String>) -> &'a String {
    let len = strings[0].len();
    let mut aprox: Vec<&'a String> = strings.to_vec();
    for i in 0..len {
        aprox = get_next_co2_approximation(&aprox, i);
        if aprox.len() == 1 {
            break;
        }
    }
    aprox[0]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut strings: Vec<String> = vec![];
    loop {
        let mut inp = String::new();
        let str = match io::stdin().read_line(&mut inp) {
            Ok(_) => inp.trim().to_string(),
            Err(_) => break,
        };
        if str.len() == 0 {
            break;
        }
        strings.push(str);
    }

    let inputs: Vec<&String> = strings.iter().collect();
    let ox = get_oxygen(&inputs);
    let co2 = get_co2(&inputs);

    let mut ox_int = 0;
    let mut co2_int = 0;
    for i in 0..ox.len() {
        ox_int *= 2;
        co2_int *= 2;
        if ox.chars().nth(i).unwrap() == '1' {
            ox_int += 1;
        }
        if co2.chars().nth(i).unwrap() == '1' {
            co2_int += 1;
        }
    }
    println!("ox {} co2 {}", ox_int, co2_int);
    println!("answ {}", ox_int * co2_int);
    Ok(())
}

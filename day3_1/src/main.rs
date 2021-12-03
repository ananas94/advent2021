use std::io;
use std::error::Error;
use std::cmp::Ordering;


fn main() ->Result<(),Box<dyn Error> >{
    let mut counters: Vec<u32> = vec![0;15];
    let mut len = 0;
    let mut total:u32 = 0;
    let mut gamma:u32 = 0;
    let mut epsilon:u32 = 0;
    loop {
        let mut inp = String::new();
        let str = match io::stdin().read_line(&mut inp) {
            Ok(_) => inp,
            Err(_) => break,
        };
        if str.len() == 0 { break; }
        len = str.len()-1;
        total+=1;

        for i in 0..str.len() {
            counters[i] += match str.chars().nth(i)   {
                Some('1') => 1,
                Some('0')=> 0,
                _ => 0
            }
        }
    }
    for i in 0..len {
        println!("{} {}", counters[i], total/2);
        gamma *= 2;
        epsilon *=2;
        gamma += match (counters[i]).partial_cmp(&(total/2)).unwrap() {
            Ordering::Less => 0,
            Ordering::Greater => 1,
            _ => 0
        };
        epsilon += match (counters[i]).partial_cmp(&(total/2)).unwrap() {
            Ordering::Greater => 0,
            Ordering::Less => 1,
            _ => 1
        };
    }
    println!("{} {}", gamma, epsilon);
    println!("{}", gamma*epsilon);

    Ok(())
}


use std::error::Error;
use std::io;

fn get_input() -> Option<Vec<u32> > {
    let mut inp = String::new();
    let parsed: Vec<u32> = match io::stdin().read_line(&mut inp) {
        Ok(_) => inp.trim().split(",").map(|str|->Result<u32,_> { str.parse() }).take_while(|x| x.is_ok()).map(|x| x.unwrap()).collect(),
        Err(_) => return None,
    };
    

    return Some(parsed);
}




fn main() -> Result<(), Box<dyn Error>> {
    let depths = get_input().ok_or("aaa")?;
    let len = depths.len();
    let sum = depths.iter().fold(0, |acc,x| acc+x );
    let mid = (sum as f64/len as f64).round()as u32;


    println!("btw max is {}", depths.iter().max().unwrap());
    println!("btw mid is {}", mid);
    println!("btw sum {} and len  {}", sum,len);
    println!("btw mid as float  is {}",  (sum as f64/len as f64));

    let mut save_index = -1;
    let mut min = 99540659;
    for mid in 0..2000 {
        let mut result:u64 = 0;
        for i in &depths {
            let n = (*i as i32 - mid as i32).abs() as u64;
            result += n*(n+1)/2;
        }
        if result < min {
            save_index = mid;
            min = result;
        }
    }
    println!("depth {} fuel {}",save_index,min );
   
    Ok(())
}

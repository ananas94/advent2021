
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
    let mut depths = get_input().ok_or("aaa")?;
    let len = depths.len();
    depths.select_nth_unstable(len/2);
    let med = depths[depths.len()/2];
    let mut sum = 0;

    for it in depths {
        sum += (it as i32 - med as i32 ).abs();
    }
    println!("{}", sum);
   
    Ok(())
}

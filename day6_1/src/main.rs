
use std::error::Error;
use std::io;

fn get_days() -> Option<[u32;9] > {
    let mut inp = String::new();
    let mut days: Vec<&str> = match io::stdin().read_line(&mut inp) {
        Ok(_) => inp.trim().split(",").collect(),
        Err(_) => return None,
    };
    
    let mut days_counter: [u32;9] = [0;9];
    let mut total = 0;
    days.iter().map(|str|->Result<u32,_> { str.parse() }).take_while(|x| x.is_ok()).map(|x| x.unwrap()).for_each(|x| { if x<9 && x>=0 {days_counter[x as usize]+=1; total+=1;} } );
    if total != days.len()  { return None; }

    return Some(days_counter);
}




fn main() -> Result<(), Box<dyn Error>> {
    let mut days = get_days().ok_or("aaa")?;
    for i in 0..9 {
        println!("{}",days[i]);
    }
    for i in 0..80 {
        let z = days[0];
        for i in (1..9) {
            days[i-1 as usize] = days[i];
            days[i]=0;
        }
        days[8]+=z;
        days[6]+=z;


    }
    let sum = days.iter().fold(0, |acc, x| acc+x );
    println!("{}", sum);

   
    Ok(())
}

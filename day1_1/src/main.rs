use std::io;
use std::error::Error;

fn getNumber() -> Result<u32,Box<dyn Error> > {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp)?;
    let num: u32 = inp.trim().parse()?;
    Ok(num)
}

fn main() ->Result<(),Box<dyn Error> >{
    let mut prev = getNumber()?;
    let mut counter = 0;
    loop{
        let current = match getNumber() {
            Ok(n) => n,
            Err(_) => { println!("result is {}", counter); break; }
        };
        if ( current > prev ) {
            counter = counter + 1;
        };
        prev = current;
    }
    Ok(())
}

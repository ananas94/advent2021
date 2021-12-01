use std::io;
use std::error::Error;

fn getNumber() -> Result<u32,Box<dyn Error> > {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp)?;
    let num: u32 = inp.trim().parse()?;
    Ok(num)
}


fn main() ->Result<(),Box<dyn Error> >{
    let mut a= [0; 3];
    for i in 0..3 {
        a[i] = getNumber()?
    }
    let mut index = 0;
    let mut counter = 0;
    loop{
        let current = match getNumber() {
            Ok(n) => n,
            Err(_) => { println!("result is {}", counter); break; }
        };
        if current > a[index] {
            counter = counter + 1;
        };
        a[index] = current;
        index = (index + 1) % 3;
    }
    Ok(())
}

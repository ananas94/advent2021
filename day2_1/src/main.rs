use std::io;
use std::error::Error;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up
}

fn getDirection() -> Option< (Direction, u32) > {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok()?;
    let mut it  = inp.split(' ');
    let direction = match it.next()? {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        _ => return None,
    };

    let num = it.next()?.trim().parse().ok()?;

    Some((direction, num))
}

fn main() ->Result<(),Box<dyn Error> >{
    let mut depth: u32 = 0;
    let mut distance: u32 = 0;
    loop {
        let (d,n) = match getDirection() {
            Some(value) => value,
            None => break,
        };
        match d {
            Direction::Forward => { distance += n; },
            Direction::Down => { depth +=n; },
            Direction::Up => {depth -= n; },
        }
    }

    println!("depth {} distance {}",depth,distance);
    println!("answer {}",depth*distance);
    Ok(())
}

use std::io;
use std::error::Error;
use std::collections::HashSet;

#[derive(Hash,PartialEq,Eq)]
struct Point {
    x: u32,
    y: u32,
}


fn read_point() -> Option<Point> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok()?;

    let mut i = inp.split(",").map(|x| { x.trim().parse::<u32>() } );
    let x=i.next()?.ok()?;
    let y=i.next()?.ok()?;

    Some(Point{x,y})
}

#[derive(PartialEq,Debug)]
enum Direction
{
    X,
    Y
}

struct Fold {
    direction: Direction,
    r : u32
}

fn read_fold()  -> Option<Fold> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok()?;
    let mut i = inp.split("=");
    let prefix = i.next()?.trim();
    let direction =  if prefix.ends_with('x') { Direction::X }    else { if prefix.ends_with('y') { Direction::Y } else { return None;} };
    let r = i.next()?.trim().parse::<u32>().ok()?;

    Some(Fold{direction, r})
}

fn fold(r: u32, c :u32 ) -> u32 {
    if c > r { 2*r -c } else {c}
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut set = HashSet::new();
    while let Some(p) = read_point() {
        set.insert(p);
    }
    while let Some(f) = read_fold() {
        println!("fold {:?} {}", f.direction, f.r);
        let mut next_set = HashSet::new();
        for p in set  {
            let new_point = match f.direction {
                Direction::X => { let x = fold(f.r, p.x); Point{ x: x, y: p.y} } 
                Direction::Y => { let y = fold(f.r, p.y); Point{ x: p.x, y: y} }
            };
            next_set.insert(new_point);
        }

        println!("new set has {} elements", next_set.len());
        set = next_set;
    }



    Ok(())
}

use std::io;
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

//from day 9 begin 
fn get_input() -> Option<Vec<u8>> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok()?;
    let numbers: Vec<u8> = inp.trim().split("").skip(1).
                            map(|x| x.trim().parse()).
                            take_while(|x| x.is_ok()).
                            map(|x| x.unwrap()).collect();
    if numbers.len() == 0  {return None; }
    Some(numbers)
}

fn get_matrix() -> Option<Vec<Vec<u8>>> {
    let mut ret = Vec::new();

    let line = get_input()?;
    let init_len = line.len();
    ret.push(line);
    while let Some(line) = get_input() {
        let len = line.len();
        if len != init_len { return None; }
        ret.push(line);
    }
    Some(ret)
}
// from day9 end

fn step(x: usize, y:usize, dx: usize, dy:usize, cave: &Vec<Vec<u8>>, distances: &mut Vec<Vec<u64>>,  queue: &mut BinaryHeap<DCpair>) {
    if (distances[x][y] + (cave[dx][dy] as u64)) < distances[dx][dy] {
        distances[dx][dy] = distances[x][y] + (cave[dx][dy] as u64);
        let dist = distances[dx][dy];

        let DC = DCpair{ 
            distance: distances[dx][dy],
            x : dx,
            y: dy
        };
        queue.push( DC);
    }

}


#[derive(Eq,PartialEq)]
struct DCpair{
    distance: u64,
    x: usize,
    y: usize
}

impl Ord for DCpair {
        fn cmp(&self, other: &Self) -> Ordering {
            other.distance.cmp(&self.distance)
        }
}

impl PartialOrd for DCpair {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}




fn main() ->Result<(), Box<dyn Error>> {
    let cave = get_matrix().ok_or("bad_inp")?;

    println!("EHHEHEHEHEHEHHE");

    let mut distances: Vec<Vec<u64>>= vec!( vec![u64::MAX; cave.len()]; cave[0].len() );

    let mut queue: BinaryHeap<DCpair> = BinaryHeap::new();

    distances[0][0] = 0;
    queue.push(DCpair{distance:0,x:0,y:0});

    while let Some(h) = queue.pop() {
        let x = h.x;
        let y = h.y;
        if x>0 {
            step(x,y,x-1,y,&cave,&mut distances, &mut queue);
        }
        if y>0
        {
            step(x,y,x,y-1,&cave,&mut distances, &mut queue);
        }
        if x<cave.len()-1 {
            step(x,y,x+1,y,&cave,&mut distances, &mut queue);
        }
        if y<cave[0].len()-1 {
            step(x,y,x,y+1,&cave,&mut distances, &mut queue);
        }
        if  distances[distances.len() -1 ][distances[0].len()-1] != u64::MAX { break;}
    }
    println!("{}", distances[distances.len() -1 ][distances[0].len()-1]);


    Ok(())
}

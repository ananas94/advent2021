
use std::error::Error;
use std::io;
use std::cmp::min;
use std::cmp::max;

struct Line{
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize
}

fn get_line() -> Option<Line > {
    let mut inp = String::new();
    let mut line_iterator = match io::stdin().read_line(&mut inp) {
        Ok(_) => inp.trim().split("->"),
        Err(_) => return None,
    };

    let x1y1 = line_iterator.next()?.trim();
    let x2y2 = line_iterator.next()?.trim();
    
    let mut start_it = x1y1.split(',');
    let x1: usize = start_it.next()?.parse().ok()?;
    let y1: usize = start_it.next()?.parse().ok()?;

    let mut end_it = x2y2.split(',');
    let x2: usize = end_it.next()?.parse().ok()?;
    let y2: usize = end_it.next()?.parse().ok()?;


    Some(Line{x1:x1,y1:y1,x2:x2,y2:y2})
}




fn main() -> Result<(), Box<dyn Error>> {
    let mut sum: [[u32; 1024];1024] = [[0;1024];1024];
    loop {
        let l = match get_line() {
            Some(v) => v,
            None => break,
        };
        if  l.x1 == l.x2  {
            for i in min(l.y1,l.y2) .. max(l.y1,l.y2)+1 {
               sum[l.x1][i] += 1; 
            }
        } else if l.y1 == l.y2  {
            for i in min(l.x1,l.x2) .. max(l.x1,l.x2)+1 {
               sum[i][l.y1] += 1; 
            }
        }
    }
    let mut two_or_more_number = 0;

    for i in 0..1024 {
        for j in 0..1024 {
            if sum[i][j] >=2 { two_or_more_number+=1; }
        }
    }

    println!("{}", two_or_more_number);
    Ok(())
}

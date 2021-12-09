use std::io;
use std::error::Error;

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

fn is_first_lower(data: &Vec<Vec<u8>> , x1: i32, y1: i32 , x2: i32 , y2: i32 ) -> bool {
    if x2 >= 0 && y2 >= 0 && (x2 as usize) < data.len()  && (y2 as usize) < data[0].len() {
        if data[x1 as usize][y1 as usize] < data[x2 as usize][y2 as usize] { return true; }
        return false;
    }
    true
}


fn is_first_lower_or_eq(data: &Vec<Vec<u8>> , x1: i32, y1: i32 , x2: i32 , y2: i32 ) -> bool {
    if x2 >= 0 && y2 >= 0 && (x2 as usize) < data.len()  && (y2 as usize) < data[0].len() {
        if data[x1 as usize][y1 as usize] <= data[x2 as usize][y2 as usize] { return true; }
        return false;
    }
    true
}



fn is_low_point(data: &Vec<Vec<u8>> , x: i32, y: i32 ) -> bool {
    if is_first_lower(data, x,y, x-1,y) &&
       is_first_lower(data, x,y, x,y-1) &&
       is_first_lower(data, x,y, x+1,y) &&
       is_first_lower(data, x,y, x,y+1) {
           return true;
    }
    false
}


fn is_low_point2(data: &Vec<Vec<u8>> , x: i32, y: i32, cmp: fn(&Vec<Vec<u8>>, i32,i32,i32,i32 ) -> bool )->bool{
    if cmp(data, x,y, x-1,y) &&
       cmp(data, x,y, x,y-1) &&
       cmp(data, x,y, x+1,y) &&
       cmp(data, x,y, x,y+1) {
           return true;
    }
    false
}



fn fill_basin_and_count(data: &mut Vec<Vec<u8>>, x:i32, y:i32) -> u64 {
    if x >= 0 && y >= 0 && (x as usize) < data.len()  && (y as usize) < data[0].len() && is_low_point2(data,x,y,is_first_lower_or_eq) && data[x as usize][y as usize] != 9 {
       data[x as usize][y as usize] = 9;
       return 1 + fill_basin_and_count(data,x,y-1) + fill_basin_and_count(data,x-1,y) + fill_basin_and_count(data, x+1,y) + fill_basin_and_count(data, x,y+1);       
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = get_matrix().ok_or("... bad input")?;
    let mut ret: Vec<u64> = Vec::new();
    for i in 0..data.len() as i32 {
        for j in 0..data[0].len() as i32 {
            if is_low_point(&data,i,j) {
                let s = fill_basin_and_count(&mut data, i, j);
                ret.push(s);
            }
        }
    }
    ret.sort();
    let size = ret.len();
    println!("{}", ret[size-1] * ret[size-2] * ret[size-3]);
    
    Ok(())
}

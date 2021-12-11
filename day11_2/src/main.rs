use std::io;
use std::error::Error;

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


fn inc_energy_lead_to_flashes(energy: &mut Vec<Vec<u8>>, x: i32, y: i32) -> u32 {
    if x>=0 && (x as usize)< energy.len() && y>=0 && (y as usize)<energy[0].len() {
        let ux = x as usize;
        let uy = y as usize;
        energy[ux][uy] +=1;
        if energy[ux][uy] == 10 {
            let mut ret = 1;
            for i in -1..2 {
                for j in -1..2 {
                    ret += inc_energy_lead_to_flashes(energy, x+i, y+j);
                }
            }
            return ret;
        }
    }
    0
}

fn main() ->Result<(), Box<dyn Error>> {
    let mut octopuses_energy = get_matrix().ok_or("bad_inp")?;
    for iteration in 0..1000 {
        let mut iteration_flashes = 0;
        for i in 0..octopuses_energy.len() {
            for j in 0..octopuses_energy[0].len() {
                iteration_flashes += inc_energy_lead_to_flashes(&mut octopuses_energy, i as i32, j as i32);
            }
        }
        if iteration_flashes == ((octopuses_energy.len() * octopuses_energy[0].len()) as u32) {
            println!("iteration: {}, flashes number {}",  iteration+1, iteration_flashes);
            break;
        }
        for i in 0..octopuses_energy.len() {
            for j in 0..octopuses_energy[0].len() {
                if octopuses_energy[i][j]>9 {
                    octopuses_energy[i][j]=0;
                }
            }
        }
    }

    Ok(())
}

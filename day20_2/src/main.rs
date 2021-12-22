use std::io;
use std::error::Error;


fn read_enh_alg_line() -> Option<Vec<bool>> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok()?;
    let m_line: Vec<bool> = inp.trim().chars().map(|c| match c { 
                '.' => { false },
                '#' => { true },
                _   => { unreachable!() },
    }).collect();
    Some(m_line)
}

fn skip_inp_line() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok();
}


fn read_matrix() -> Option<Vec<Vec<bool>>> {
    let mut v = Vec::new();
    loop {
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).ok()?;
        let s = inp.trim();
        if s.len() == 0 { break; }
        let m_line: Vec<bool> = s.chars().map(|c| match c { 
                '.' => { false },
                '#' => { true },
                _   => { unreachable!() },
        }).collect();
        v.push(m_line);
    }
    Some(v)
}

#[derive(Clone,Copy)]
enum InfColor{
    Dark,
    Light
}

fn get_color(v:& Vec<Vec<bool>>, i: isize, j: isize, default:InfColor) -> bool {
    let vh = v.len() as isize;
    let vl = v[0].len() as isize;

    if 0<=i && i<vh && 0<=j && j<vl {
        match v[i as usize][j as usize] {
            false => false,
            true  => true,
        }
    }
    else {
        match default {
            InfColor::Dark => false,
            InfColor::Light => true,
        }
    }
}

fn get_color_from_code(enh:& Vec<bool>, code: usize) -> bool {
    enh[code]
}

fn enh(v:& Vec<Vec<bool>>,  enh: &Vec<bool>, c: InfColor) -> Vec<Vec<bool>> {
    let vh = v.len();
    let vl = v[0].len();
    let mut ret : Vec<Vec<bool>> = vec![ vec![false;vl+2]; vh+2];
    
    for i in -1..( (vh+1) as isize) {
        for j in -1..((vl+1) as isize) {
            let mut code = 0;
            for a in -1..=1 {
                for b in -1..=1 {
                    code = code << 1;
                    let bit = get_color(v, i+a, j+b, c);
                    code += if bit {1} else {0};

                }
            }
            let iret = (i+1) as usize;
            let jret = (j+1) as usize;
            ret[iret][jret] = get_color_from_code(&enh, code);

        }
    }
    ret
}

fn main() ->Result<(),Box<dyn Error>>{
    let enh_l = read_enh_alg_line().ok_or("error on enh alg line")?;
    skip_inp_line();
    let v = read_matrix().ok_or("error on matrix")?;

    let mut v_prev = v;
    let mut inf_color = InfColor::Dark;
    for _i in 0..50 {
        let v_next = enh(&v_prev, &enh_l, inf_color);
        v_prev = v_next;
        inf_color = match inf_color {
            InfColor::Dark => InfColor::Light,
            InfColor::Light => InfColor::Dark,
        }
    }


    let mut count = 0;
    for it in v_prev {
        for jt in it {
            if jt { count +=1; print!("#"); }
            else { print!("."); }
        }
        println!("")
    }
    println!("{}", count);
    Ok(())
}


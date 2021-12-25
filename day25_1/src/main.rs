use std::io;

fn read_matrix() -> Option<Vec<Vec<char>>> {
    let mut v = Vec::new();
    loop {
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).ok()?;
        let s = inp.trim();
        if s.len() == 0 { break; }
        let m_line: Vec<char> = s.chars().collect();
        v.push(m_line);
    }
    Some(v)
}


fn make_step_right( m: Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
    let mut ret = vec![vec!['_'; m[0].len()]; m.len() ];
    let mut retf = false;

    let mlen = m.len() ;
    let m0len = m[0].len() ;


    for i in 0..mlen {
        for j in 0..m0len {
            let jprev = if j == 0 { m0len-1 }  else { j-1 };
            if m[i][j]=='.' && m[i][jprev]=='>' {
                ret[i][j] = '>';
                ret[i][jprev] = '.';
                retf = true;
            } else {
                if ret[i][j] == '_' { 
                    ret[i][j] = m[i][j]; 
                }
            }
        }
    }
    (retf, ret)
}

fn make_step_down( m: Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
    let mut ret = vec![vec!['_'; m[0].len()]; m.len() ];
    let mut retf = false;

    let mlen = m.len() ;
    let m0len = m[0].len() ;

    for i in 0..mlen {
        for j in 0..m0len {
            let iprev = if i == 0 { mlen-1 }  else { i-1 };
            if m[i][j]=='.' && m[iprev][j]=='v' {
                ret[i][j] = 'v';
                ret[iprev][j] = '.';
                retf = true;
            } else {
                if ret[i][j] == '_' { 
                    ret[i][j] = m[i][j]; 
                }
            }
        }
    }
    (retf, ret)
}




fn main() {
    let mut m = read_matrix().expect("bad output");

    let mut move_down = true;
    let mut move_right = true;
    let mut steps = 0;
    while (move_down || move_right) {
        let (f, mtmp) = make_step_right(m);
        move_right = f;
        m = mtmp;

        let (f,mtmp) = make_step_down(m);
        move_down = f;
        m = mtmp;

        steps+=1;
    }

    let mlen = m.len() ;
    let m0len = m[0].len() ;

    for i in 0..mlen {
        for j in 0..m0len {
            print!("{}",m[i][j]);
        }
        println!("");
    }

    println!("{}", steps);
    

    println!("Hello, world!");
}

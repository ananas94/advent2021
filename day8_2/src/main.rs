use std::io;
use std::error::Error;
use std::convert::TryInto;

fn str_to_vec(st :String) -> Vec<String> {
    let some :Vec<String>= st.split(" ").map( |d|->String {
                                                       let mut d_split_sorted: Vec<char> = d.trim().chars().collect();
                                                       d_split_sorted.sort();
                                                       d_split_sorted.into_iter().collect()
                                                   }).filter(|x| x.len()>0).collect();
    some
}

fn read_line() -> Option< ([String;10],[String;4]) > {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok()?;
    let mut it = inp.split("|");

    let digits = it.next()?.to_string();
    let input = it.next()?.to_string();

    let digits_sorted:[String;10]  = str_to_vec(digits).try_into().ok()?;
    let input_sorted:[String;4] = str_to_vec(input).try_into().ok()?;

    Some((digits_sorted, input_sorted))
}

fn main() ->Result<(), Box<dyn Error> >{
    let mut total = 0;
    loop {
        let (dig, inp) = match read_line() {
            Some((a,b)) => (a,b),
            None => break,
        };
        let mut length:[usize;10] =[0;10];
        let mut length1:[usize;10] = [0;10];
        let mut length4:[usize;10] = [0;10];
        let mut index : [usize;10]= [0;10];
        
        for i in 0..10 {
            length[i] = dig[i].len();
        }

        let mut i1=0;
        let mut i4=0;
        for i in 0..10 {
            match length[i] {
                2 =>    { index[i] = 1; i1=i;},
                4 =>    { index[i] = 4; i4=i;},
                3 =>    index[i] = 7,
                7 =>    index[i] = 8,
                _ => {}
            }
        }

        for i in 0..10 {
            if index[i] != 0  { continue; }
            let mut len = 0;
            for a in dig[i].chars() {
                if dig[i1].find(a) == None { len+=1; }
            }
            length1[i] = len;
            match len {
                3 => index[i] = 3,
                5 => index[i] = 6,
                _ => {}
            }
        }
        for i in 0..10 {
            let mut len = 0;
            for a in dig[i].chars() {
                if dig[i4].find(a) == None { len+=1; }
            }
            length4[i] = len;
        }
        for i in 0..10 {
            if index[i] != 0  { continue; }
            if length[i] == 6  && length4[i] ==3 { index[i] = 0; }
            if length[i] == 5  && length4[i] ==3 { index[i] = 2; }
            if length[i] == 5  && length4[i] ==2 { index[i] = 5; }
            if length[i] == 6  && length4[i] ==2 { index[i] = 9; }
        }

        let mut num = 0;
        for s in inp{
            num *=10;
            for i in 0..10 {
                if s == dig[i] { num += index[i]; }
            }
        }
        total += num;
    }
    println!("{}", total);

    Ok(())
}

use std::io;
use std::error::Error;
use std::collections::HashMap;

fn read_polymer_teplate() -> Result<String, Box<dyn Error>> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp)?;

    Ok(inp.trim().to_string())
}

fn read_rule() -> Option<(String, [String;2])> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok()?;

    let mut it = inp.split("->");
    let from = it.next()?.trim().to_string();
    let add = it.next()?.trim();

    let mut from_it = from.split("").filter(|x| x.len()>0);

    let to1:String = from_it.next()?.to_string() + &add.to_string();
    let to2:String = add.to_string() + &from_it.next()?.to_string();
 
    Some((from, [to1,to2]))
}

fn read_rules() -> Result<HashMap<String,[String;2]>, Box<dyn Error>> {
    let  mut ret = HashMap::new();
    while let Some((from,to)) = read_rule() {    
       
        ret.insert(from, to);
    }
    Ok(ret)   
    
}


fn main() -> Result<(), Box<dyn Error> > {
    let template = read_polymer_teplate()?;

    let mut inp = String::new();
    io::stdin().read_line(&mut inp)?;

    let rules = read_rules()?;


    let mut polymer: HashMap<String, u64> = HashMap::new();

    for i in 0..template.len()-1 {
        let s = &template[i..i+2];
        polymer.entry(s.to_string()).and_modify(|x| { *x+=1; }).or_insert(1);
    }

    for i in 0..10 {
        let mut next_polymer: HashMap<String, u64> = HashMap::new();
        for (p,pn) in polymer {
            let to = rules.get(&p).unwrap();
            for t in to {
                next_polymer.entry(t.to_string()).and_modify(|x| { *x+=pn;}).or_insert(pn);
                //println!("insert from {} to {} {}",p, t, pn); 
            }
        }
        polymer = next_polymer;
    }

    for (p, pn) in &polymer {
        println!("{} : {}", p, pn);
    }

    let mut letters: HashMap<char, u64> = HashMap::new();
    

    let first = template.chars().nth(0).unwrap();
    let last = template.chars().last().unwrap();
    
    letters.insert(first,1);
    letters.insert(last,1);

    for (p,pn) in &polymer {
        for c in p.chars() {
            letters.entry(c).and_modify(|x| { *x+=pn; } ).or_insert(*pn);
        }
    }

    let (_, min) = letters.iter().min_by( |(_,x), (_,y) | x.cmp(y)).unwrap()  ;
    let (_, max) = letters.iter().max_by( |(_,x), (_,y) | x.cmp(y)).unwrap()  ;
    println!("{} - {} = {}", max/2, min/2, (max -min)/2);

    Ok(())
}

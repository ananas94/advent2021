
struct Cube //cuboid really
{
    x1: i64,
    y1: i64,
    z1: i64,
    x2: i64,
    y2: i64,
    z2: i64
}

#[derive(std::cmp::PartialEq, Copy,Clone,Debug)]
enum CrossMode
{
    On,
    Off            //off only if all 3d coordinates are off
}


fn itersection1d( a1:i64, a2:i64, b1:i64,b2:i64 ) -> Vec<(CrossMode,i64,i64)>  {
    if a2 < b1 { return vec![ (CrossMode::On,a1,a2) ]; }   //don't intersect at all
    if a1 > b2 { return vec![ (CrossMode::On,a1,a2) ];  } //don't intersect at all

    let mut ret = Vec::new();
    if a1<b1 { 
        ret.push( (CrossMode::On,a1,b1-1) );
        if b2 < a2 {
            ret.push( (CrossMode::Off,b1,b2) );
            ret.push( (CrossMode::On,b2+1, a2));
        }
        else {
            ret.push( (CrossMode::Off,b1, a2));
        }
    } else {

        if b2<a2 {
            ret.push((CrossMode::Off,a1,b2) );
            ret.push((CrossMode::On,b2+1,a2) );
        }else {
            ret.push((CrossMode::Off,a1,a2));
        }
    }

    ret
}

fn intersection3d(on:& Cube, off:& Cube) -> Vec<Cube> {
    let mut ret = Vec::with_capacity(27);

    let x_inter = itersection1d(on.x1,on.x2, off.x1, off.x2);
    let y_inter = itersection1d(on.y1,on.y2, off.y1, off.y2);
    let z_inter = itersection1d(on.z1,on.z2, off.z1, off.z2);

    for x in &x_inter{
        for y in &y_inter {
            for z in &z_inter {
                let (xf,x1,x2) = *x;
                let (yf,y1,y2) = *y;
                let (zf,z1,z2) = *z;
                let cube = Cube{x1,y1,z1,x2,y2,z2};
                if xf != CrossMode::Off || yf != CrossMode::Off || zf != CrossMode::Off {
  //                  print!("({};{};{} , {} {} {})", cube.x1,cube.y1,cube.z1, cube.x2,cube.y2,cube.z2);
    //                println!(" xf {:?} yf {:?} zf {:?} ", xf, yf, zf);
                    ret.push(cube);
                }
            }
        }
    }
    ret
}

fn volume( c : &Cube) -> i64 {
    let v = (c.x2-c.x1+1) * (c.y2-c.y1+1) *(c.z2-c.z1+1);
//    println!("v {}", v);
    v
}

fn all_cubes_intersections(cubes: Vec<Cube>, off: Cube) -> Vec<Cube> {
    let mut ret = Vec::with_capacity(cubes.len() + 10 );
    for c in cubes {
        let mut inter = intersection3d(&c, &off);
        ret.append(&mut inter);
    }
    ret
}


fn parse_coord(str: &str) -> (i64,i64) {
    let mut type_split = str.split("=");
    type_split.next();
    let numbers = type_split.next().unwrap();
    let mut num_iter = numbers.split("..");

    let first_s = num_iter.next().unwrap();
    let first = first_s.parse().unwrap();
    let second = num_iter.next().unwrap().parse().unwrap();

    (first, second)

}

fn read_cube() -> Option<(CrossMode, Cube)> {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).ok()?;
    if inp.trim().len() == 0 { return None; }

    let mut onoff_split = inp.trim().split_whitespace();

    let onoff = onoff_split.next().unwrap();

    let cube = onoff_split.next().unwrap();

    let mut coord_split = cube.split(",");

    let x_str = coord_split.next().unwrap();
    let y_str = coord_split.next().unwrap();
    let z_str = coord_split.next().unwrap();

    let (x1,x2) = parse_coord(x_str);
    let (y1,y2) = parse_coord(y_str);
    let (z1,z2) = parse_coord(z_str);

    let mode = match onoff {
        "on" => CrossMode::On,
        "off" => CrossMode::Off,
        _ =>    unreachable!()
    };

    Some((mode, Cube{x1,y1,z1,x2,y2,z2}))
}

fn main() {

    let mut cubes = Vec::new();

    while let Some((onoff,c)) = read_cube() {
        match onoff {
            CrossMode::On  => { cubes.push(c); },
            CrossMode::Off => { cubes = all_cubes_intersections(cubes, c); }
        }
    }

//    let mut inter_cubes = intersection3d(a,b);

    println!("cube number: {} start counting volumes", cubes.len());

    let global_len = cubes.len();
    
    let mut iter: u64 = 0;

    let mut counter = 0;
    loop {
        let current_len = cubes.len();
        if current_len == 0 { break; }
        
        if iter%1000 == 0 {
            println!("current length is {}, start length is {}", current_len, global_len);
        }
        iter+=1;


        let cube = cubes.pop().unwrap();
        
        counter+=volume(&cube);

        cubes = all_cubes_intersections(cubes, cube);
            
    }

    println!("{}", counter);



}

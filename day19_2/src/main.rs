use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

type Point = (i32,i32,i32);
type Direction = (i32,i32,i32);

fn skip_scanner_line() -> Option<()> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok()?;
    if inp.trim().len() == 0 { return None; }
    Some(())
}

fn get_xyz() -> Option<Point> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).ok()?;
    let c: Vec<i32> = inp.trim().split(",").map(|x| x.trim().parse()).take_while(|x| x.is_ok()).map(|x| x.unwrap()).collect();
    if c.len() != 3 { return None; }
    Some( (c[0],c[1],c[2]) )
}

fn rotate(p: &Point, rotation_id: u8) -> Point {
    let (x,y,z) = *p;
    let (x1,y1,z1) = match rotation_id/4 {  
        0 =>  ( x, y, z),              // z is up     
        1 =>  (-x, y,-z),              //-z is up
        2 =>  ( z, y,-x),              // x is up
        3 =>  (-z, y, x),              //-x is up
        4 =>  ( x, z,-y),              //y is up
        5 =>  ( x,-z, y),              //-y is up
        _ => unreachable!()
    };
    let (x2,y2,z2) = match rotation_id%4 {   
        0 => ( x1, y1, z1),
        1 => ( y1,-x1, z1),        //90 degrees around 0z
        2 => (-x1,-y1, z1),        //180
        3 => (-y1, x1, z1),        //270
        _ => unreachable!()
    };
    (x2,y2,z2)
}

fn distance(p1:& Point, p2:& Point) -> Direction {
    let (x1,y1,z1) = *p1;
    let (x2,y2,z2) = *p2;
    (x1-x2,y1-y2,z1-z2)
}

type PointsRelations = Vec<HashMap<Direction,(Point,Point)>> ;

fn calculate_distances(v:&Vec<Point> ) -> PointsRelations {
    let mut ret = Vec::new();
    for i in v.into_iter() {
        let p1 = *i;
        let mut map = HashMap::new();
        for j in v.into_iter() {
            if i==j { continue; }
            let p2 = *j;
            let dist =distance(&p1,&p2);
            map.insert(dist,(p1,p2));
        }
        ret.push(map);
    }
    ret
}

//YEP IT SHOULD BE MACROS OR GENERIC... I FAILED TO WRITE IT
fn calculate_distances_from_hashset(v:&HashSet<Point> ) -> PointsRelations {
    let mut ret = Vec::new();
    for i in v.into_iter() {
        let p1 = *i;
        let mut map = HashMap::new();
        for j in v.into_iter() {
            if i==j { continue; }
            let p2 = *j;
            let dist =distance(&p1,&p2);
            map.insert(dist,(p1,p2));
        }
        ret.push(map);
    }
    ret
}



fn findSamePoint( pr1:&PointsRelations, pr2:&PointsRelations ) -> Option< (Point,Point) > {
    for v1 in pr1 {
        for v2 in pr2 {
            let mut counter = 0;
            for (d1, (i1,_)) in v1 {
                if v2.contains_key(d1) {
                    counter+=1; 
                    let (i2,_) = v2.get(d1).unwrap(); 
                    if counter >=11 {
                        return Some((*i1,*i2))
                    }
                }
            }
        }
    }
    None
}

type Beacons = HashSet<Point>;


fn add(p: Point, d: Direction) -> Point {
    let (x,y,z) = p;
    let (dx,dy,dz) = d;
    (x+dx, y+dy, z+dz)
}

fn add_to_beacons(b: &mut Beacons, v: Vec<Point>, d: Direction) {
    for p in v {
        b.insert( add(p,d) );
    }
}

fn main() {
    let mut beacons:Beacons = HashSet::new();
    let mut scanners_data = Vec::new();
    let mut first_scaner = true;
    while let Some(()) = skip_scanner_line() {
        let mut scanner_data = Vec::new();
        while let Some(p) = get_xyz() {
            scanner_data.push(p);
            if first_scaner {
                beacons.insert(p);
            }
        }
        first_scaner =false;
        scanners_data.push(scanner_data);
    }
    scanners_data.remove(0);

    let mut scanners_number = scanners_data.len();
    let mut vDist = Vec::new();
    while scanners_number != 0 {
        scanners_number = scanners_data.len();
        let mut i = 0;
        while i<scanners_data.len() {
            let distances = calculate_distances_from_hashset(&beacons); //each time - we add new points each time
            let mut found = false;
'rotations:
            for rotation in 0..24 {
                let mut new_data = Vec::new();
                for point in &scanners_data[i] {
                    let p = rotate(&point, rotation);
                    new_data.push(p);
                }
                let new_dist = calculate_distances(&new_data);
                if let Some(p) = findSamePoint(&distances, &new_dist) {
                    println!("Same point found!");
                    println!("rotation {}", rotation);
                    let (i1,i2) = p;
                    let point_dist = distance(&i1,&i2);
                    vDist.push(point_dist);
                    add_to_beacons(&mut beacons, new_data, point_dist);
        println!("beacons len {}", beacons.len());
                    found = true;
                    scanners_data.remove(i);
                    break 'rotations;
                }
            }
            i+=1;
        }
    }

    println!("beacons len {}", beacons.len());

    let v_dist_len = vDist.len(); 
    let mut max = 0;
    for i in 0..v_dist_len { // I belive, there is better way to write it
        for j in 0..v_dist_len {
            let (x1,y1,z1) = vDist[i];
            let (x2,y2,z2) = vDist[j];
            let manhattan_distance =  (x1-x2).abs() + (y1-y2).abs() + (z1-z2).abs();
            if manhattan_distance > max { max = manhattan_distance; };
        }
    }
    println!("manhattan dist {}",max);

    println!("Hello, world!");
}

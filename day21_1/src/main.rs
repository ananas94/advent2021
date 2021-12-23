

fn main() {
    let first_dice  = [6,4,2,0,8];
    let second_dice = [5,3,1,9,7];
    
    let mut get_first  = first_dice. iter().cycle();
    let mut get_second = second_dice.iter().cycle();

    let mut points_first = 0;
    let mut points_second = 0;

    let mut position_first = 8;
    let mut position_second = 5;

    let mut it = 0;
    loop {
        it+=1;
        position_first =  (*get_first.next().unwrap() + position_first) % 10;
        if position_first == 0 { position_first = 10; }
        points_first += position_first;
        if points_first >=1000 { break;}

        it+=1;
        position_second =  (*get_second.next().unwrap() + position_second) % 10;
        if position_second == 0 { position_second = 10; }
        points_second += position_second;
        if points_second >=1000 { break;}

    }

    println!("game finished:");
    println!("iteration number: {}", it);
    println!("first has {} points", points_first);
    println!("second has {} points", points_second);
    println!("");
    println!("solution {}", 3*it * points_first.min(points_second));


}

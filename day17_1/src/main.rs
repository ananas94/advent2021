


/*
const x_input_min: i32 = 20;
const x_input_max: i32 = 30;
const y_input_min: i32 = -10;
const y_input_max: i32 = -5;
*/

const x_input_min: i32 = 244;
const x_input_max: i32 = 303;
const y_input_min: i32 = -91;
const y_input_max: i32 = -54;
fn main() {
    let mut sum = 0;
    let mut vertical_fall_down = false;
    for i in 0..x_input_max  {
        sum += i;
        if sum>= x_input_min && sum <= x_input_max { println!("BANG {} : {}",i, sum); vertical_fall_down = true; }
    }
    if vertical_fall_down == false { panic!("it's another case"); }
    else {
        let y_speed = (y_input_min +1).abs();
        let max = y_speed * (y_speed+1) / 2;
        println!("{}", max);
    }


    println!("Hello, world!");
}

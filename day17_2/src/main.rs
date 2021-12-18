


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
        let mut counter = 0;
        for xi in 0..x_input_max+1 {
            for yi in y_input_min..(y_input_min+1).abs() +1 {
                let mut x = 0;
                let mut y = 0;
                let mut xs = xi;
                let mut ys = yi;
                for _i in 0..1000 {
                    x += xs;
                    y += ys;
                    if x_input_min <= x && x <= x_input_max && y_input_min <= y && y <= y_input_max {
                        counter+=1; break; }
                    if (xs==0 &&  x < x_input_min ) || x > x_input_max {break;}
                    if y<y_input_min {break;}

                    if xs >0 { xs -= 1; }
                    ys -=1;
                }
           }
        }
        println!("{}", counter);

    }

    println!("Hello, world!");
}

use std::collections::HashMap;

type Cache =  HashMap<((u8,u8),(u8,u8)), (u64,u64)> ;


fn roll_a(a_store: &mut Cache, b_store: &mut Cache, (points_a, position_a): (u8,u8), (points_b, position_b): (u8,u8) ) -> (u64,u64) {
    let mut first_win =0;
    let mut second_win =0;
    for r1 in [1,2,3] {
        for r2 in [1,2,3] {
            for r3 in [1,2,3] {
                let roll = r1+r2+r3;
                let mut new_position = (position_a + roll) %10;
                if new_position == 0 { new_position = 10; }
                let new_points = points_a + new_position;
                if new_points >= 21 { first_win+=1; }
                else {
                    if let Some((f,s)) = a_store.get( &((new_points, new_position), (points_b, position_b)) ) {
                        first_win += f;
                        second_win +=s;
                    } else {
                        let (f,s) = roll_b(a_store, b_store, (new_points, new_position), (points_b,position_b)) ;
                        a_store.insert( ((new_points, new_position), (points_b, position_b)), (f,s) );
                        first_win += f;
                        second_win +=s;
                    }
                }
            }
        }
    }
    (first_win,second_win)
}

fn roll_b(a_store: &mut Cache, b_store: &mut Cache, (points_a, position_a): (u8,u8), (points_b, position_b): (u8,u8)) -> (u64,u64) {
    let mut first_win =0;
    let mut second_win =0;
    for r1 in [1,2,3] {
        for r2 in [1,2,3] {
            for r3 in [1,2,3] {
                let roll = r1+r2+r3;
 
                let mut new_position = (position_b + roll) %10;
                if new_position == 0 { new_position = 10; }
                let new_points = points_b + new_position;
                if new_points >= 21 { second_win +=1; }
                else {
                    if let Some((f,s)) = b_store.get( &((points_a,position_a) , (new_points, new_position))) {
                        first_win += f;
                        second_win +=s;
                    } else {
                        let (f,s) = roll_a(a_store, b_store, (points_a,position_a) , (new_points, new_position));
                        b_store.insert( ((points_a,position_a) , (new_points, new_position)), (f,s));
                        first_win += f;
                        second_win +=s;
                    }
                }
            }
        }
    }
    (first_win,second_win)
}

fn main() {
    let mut a_cache: Cache = HashMap::new();
    let mut b_cache: Cache  = HashMap::new();
    let (first,second) = roll_a(&mut a_cache, &mut b_cache, (0,8), (0,5)  );

    println!("{},{}", first, second);
    println!("{}", first.max(second));
}

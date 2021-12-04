use std::error::Error;
use std::io;
use std::convert::TryInto;

fn get_line() -> Result< Vec<u32>, Box<dyn Error> > {
    let mut inp = String::new();
    let lineStr: Vec<&str> = match io::stdin().read_line(&mut inp) {
        Ok(_) => inp.trim().split_whitespace().collect(),
        Err(_) => return Err("AAA")?,
    };

    let line: Vec<u32> = lineStr.iter().map(|str| str.parse()).take_while(|x| x.is_ok()).map(|x| x.unwrap()).collect();
    if line.len() != lineStr.len() || line.len() != 5 { return Err("BBB")? }

    Ok(line)
}


fn get_input() -> Result< Vec<u32>, Box<dyn Error> > {
    let mut inp = String::new();
    let lineStr: Vec<&str> = match io::stdin().read_line(&mut inp) {
        Ok(_) => inp.trim().split(",").collect(),
        Err(_) => return Err("AAA")?,
    };

    let line: Vec<u32> = lineStr.iter().map(|str| str.parse()).take_while(|x| x.is_ok()).map(|x| x.unwrap()).collect();
    if line.len() != lineStr.len() { return Err("BBB")? }

    Ok(line)
}


struct BingoRow {
    board_number: u32,
    row: [u32;5],
    count: u32
}

fn main() ->Result<(),Box<dyn Error> > {
    let bingo_input:Vec<u32> = get_input()?;
    let mut board_data: Vec<BingoRow> = Vec::new();
    let mut board_number = 0;
    'l: loop {
        let mut board: Vec<Vec<u32>> = Vec::new();


        let mut inp = String::new();
        match io::stdin().read_line(&mut inp) {
            Ok(_) => {},
            Err(_) => break 'l,
        };

        for _i in 0..5 {
            let l  = match get_line(){
                Ok(x) => x,
                Err(_) => break 'l,
            };
            board.push(l);
        }
        
        for i in 0..5 {
            let row_data =BingoRow{ 
                board_number: board_number,
                row: board[i].to_vec().try_into().unwrap(),
                count:      0
            };
            board_data.push(row_data);
        }
        for i in 0..5 {
           let mut row: [u32;5] = [0;5];
           for j in 0..5 {
              row[j]=board[j][i];  
            }
            let row_data = BingoRow{
                board_number: board_number,
                row: row,
                count:      0
            };
 
            board_data.push(row_data);
        }
        board_number+=1;
    }
    let mut winner: Option<u32> = None;
    let mut fin: Option<u32> = None;
    for n in bingo_input {
        for row in &mut board_data {
            match row.row.iter().position(|&x| x==n) {
                Some(v) => { 
                    println!("find {} in board {} in {} position", n, row.board_number, v);
                    row.count+=1;
                    row.row[v] = 100;
                },
                None => {}
            }
            if row.count == 5 { println!("WIN: row.board_number {}", row.board_number); winner = Some(row.board_number); fin = Some(n); }
        }
        if winner != None { break; }
    }
    let mut sum = 0;
    if winner != None {
        for row in board_data {
            for i in row.row {
                if row.board_number == winner.unwrap() {sum += i%100;}
            }
        }
        println!("{} {}", sum/2, fin.unwrap());
        println!("{} ", sum/2 * fin.unwrap());
    }
    
    

    Ok(())
}

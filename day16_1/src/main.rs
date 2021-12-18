use std::io;
use std::error::Error;


struct BitStream{
    bytes   : Vec<u8>,
    num     : usize,
    pointer : usize
}

impl Iterator for BitStream {
    type Item  = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer >= self.num { return None; }

        let byte_pos = self.pointer/8 as usize;
        let bit_pos = self.pointer%8;

        self.pointer+=1;

        let byte = self.bytes[byte_pos];
        
        let bit = (byte << bit_pos) & (1<<7) == (1<<7);
        Some(bit)
    }
}


fn str_to_bytes(s: String) -> BitStream {
    let mut v = Vec::new();
    let mut chars = s.trim().chars();
    let mut byte = 0;
    let mut first4 = true;
    while let Some(c) = chars.next() {
        let d = c.to_digit(16).unwrap() as u8;   // yep it's bad practice 
        if first4 {
            byte = d << 4;
            first4 = false;
        }
        else {
            byte += d;
            first4 = true;
            v.push(byte);
        }
    }
    if first4 == false { v.push(byte); } //break happens after full byte
    BitStream{bytes: v, num: (s.len()-1)*8, pointer: 0}
}


macro_rules! create_n_bit_function {
    ($func_name:ident, $func_type: ty) => {
        fn $func_name<const N: usize> (stream: &mut BitStream) -> $func_type {
            let mut val =0;
            for _i in 0..N {
                val = val << 1;
                let b = stream.next();
                match b {
                    Some(true)  => {  val=val+1}, 
                    Some(false) => { },
                    None => panic!(""), // oops i did it again
                };
            }
            val
        }
    }
}


create_n_bit_function!( get_n_bit8, u8);
create_n_bit_function!( get_n_bit32, u32);

const STRING_LITERAL: u8 = 4;

fn read_string_literal(stream: &mut BitStream) {
    loop { 
        let l = get_n_bit8::<5>(stream);
        if l & 0b10000 == 0 { break;}
    }
}

fn read_operator_packet_versions(stream: &mut BitStream) -> u64 {
    let p_length_type = get_n_bit8::<1>(stream);
    let mut v = 0;
    if p_length_type == 0 {
        let total_length = get_n_bit32::<15>(stream);
        let current_pointer = stream.pointer;
        while current_pointer + (total_length as usize) != stream.pointer {
            v+= read_packet_versions(stream);
        }
    } else {
        let number_of_subpackets = get_n_bit32::<11>(stream);
        for i in 0..number_of_subpackets{
            v+=read_packet_versions(stream);
        }
    }
    v
}

fn read_packet_versions(stream: &mut BitStream) -> u64 {
    let p_version = get_n_bit8::<3>(stream);
    let p_type = get_n_bit8::<3>(stream);
    let mut v = p_version as u64;

    match p_type {
        STRING_LITERAL => {
            read_string_literal(stream);
        },   
        _ => { v+=read_operator_packet_versions(stream); } 
    };
    v
}

fn main() ->Result<(),Box<dyn Error>>{
    let mut inp = String::new();
    io::stdin().read_line(&mut inp)?; 
    let mut stream = str_to_bytes(inp);


    let version_sum = read_packet_versions(& mut stream);
    println!("{}", version_sum);
    

    Ok(())
}

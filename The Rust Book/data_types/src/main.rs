use std::io;

fn main() {
    // Scalar Types
    let decimal_number = 98_222;
    println!("{decimal_number}");
    let hex_number = 0xff;
    println!("{hex_number}");
    // Compund Types
    let tup:(i32, u32, f64, u8) = (500,500,11.1,1);
    println!("{tup:?}"); // pretty print | You cannot directly print a tuple
    let (x,y,z,w) = tup;
    println!("{x},{y},{z},{w}");
    let (x,y,z,w) = (tup.0,tup.1,tup.2,tup.3);
    println!("{x},{y},{z},{w}");
    let a = [1,2,3,4,5];
    println!("{a:?}");
    let a: [i32;5] = [7,34563,2234,23423,12];
    println!("{a:?}");
    println!("Enter Index");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index not a number");
    let element = a[index];
    println!("{element}");
}   
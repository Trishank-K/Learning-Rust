fn main() {
    let x: i32 = 10;
    let y: u32 = 1000;
    let z: f32 = 1000.001; 
    println!("Hello, world!");
    println!("x:{}", x);
    println!("y:{}", y);
    println!("z:{}", z);

    let is_awesome: bool = true;
    if is_awesome{
        println!("Awesome")
    }
    else{
        println!("Still Awesome")
    }

    let greeting: String = String::from("hello world");
    println!("{}", greeting);

    /* 

    let char1 = greeting.chars().nth(100);

    match char1{
        Some(c) => println!("{}",c),
        None=>println!("No character at index"),
    } 
    
    */

}

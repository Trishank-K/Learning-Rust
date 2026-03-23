fn main() {
    let num = 3;
    if num < 5 {
        println!("Trueee");
    } else {
        println!("Falseee");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    // let number = if condition { 5 } else { "six" }; Wrong; must be of same type

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 8 {
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining-=1;
        }
        count+=1;
    }
    println!("End Count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let arr = [1,2,3,4,5,6];
    for element in arr {
        print!("{element} ");
    }

    for number in (1..4).rev() {
        print!("{number} ");
    }

}

pub fn conditionals() {
    let x = 99;
    let is_even = x % 2 == 0;

    // if-else statement
    if is_even {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }
}

pub fn loops(){
    // for-loop
    for i in 0..10 {
        print!("{i} ")
    }
}
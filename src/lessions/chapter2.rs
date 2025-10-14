pub mod control_flow {
    pub fn conditionals() {
        let x = 99;
        let is_even = x % 2 == 0;

        // if-else statement
        if is_even {
            println!("{} is even", x);
        } else {
            println!("{} is odd", x);
        }

        let number = if is_even {100} else {0};
        println!("if in let statement number is {}", number);
    }

    pub fn loops(){
        // for-loop
        for i in 0..10 {
            print!("{i} ")
        }

        // // loop
        // loop { // used for infinite loops
        //     println!("Hello Harshit!")
        // }

        let mut counter = 0;

        let mut result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 10; // ! adding value with break returns it
            }
        };

        println!("counter: {}", result);

        // while (conditional loops)
        while result < 100 {
            result += 10
        }
    }
}


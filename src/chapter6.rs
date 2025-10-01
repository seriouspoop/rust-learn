pub mod enums {

    #[derive(Debug)]
    enum IpAddr {
        IPv4(u8, u8, u8, u8),
        IPv6(String),
    }

    impl IpAddr {
        fn call(&self) {
            match self {
                IpAddr::IPv4(a, b, c, d) => {
                    println!("This is an IPv4 address ({}.{}.{}.{})", a, b, c, d);
                }
                IpAddr::IPv6(addr) => {
                    println!("This is an IPv6 address: {}", addr);
                }
            }
        }
    }

    pub fn ip_addr_example() {
        let home = IpAddr::IPv4(127, 0, 0, 1);
        let loopback = IpAddr::IPv6(String::from("::1"));

        println!("{home:?}, {loopback:?}");

        home.call();
        loopback.call();
    }

    pub fn option_enum() {
        let some_number = Some(5);
        let some_char = Some('e');
        let none_number: Option<i32> = None;

        println!("{some_number:?}, {some_char:?}, {none_number:?}");
    }
}

pub mod matching {
    use rand::Rng;

    #[allow(dead_code)]
    enum Currency {
        INR(i32),
        USD(i32),
        EURO(i32),
        YEN(i32),
    }

    impl Currency {
        /// Here `&self` indicates that this method is callable only on reference to the currency enum variants.
        /// and the type of self is:
        /// ```rust
        /// self: &Currency
        /// ```
        /// inside the match block the pattern should match with the reference to the variant
        /// in the form of `&Enum::Variant(x)`, if it matches reference to reference then the inner value
        /// is de-structured as an owned type. So `x` is binded to the value as owned.
        ///
        /// If the pattern inside match is of type `Enum::Variant(x)`, this matches to the reference of the
        /// match value, hence it is converted to a reference and `x` value is binded as a reference to the value.
        ///
        /// Explanation:
        /// Assume self is `&Currency::INR(10)` and pattern is `Currency::INR(x)`, rust after stripping off
        /// sees `x = &10` so it binds `x` to reference of value 10. This is rust default binding mode.
        /// when an irrefutable pattern is applied to a reference, all variables in the pattern are by default bound as references.
        ///
        /// If self is `&Currency::INR(10)` and pattern is also `&Currency::INR(x)`, then rust after stripping off
        /// sees `&x = &10` so deferencing both gives `x = 10` and hence `x` binds to 10.
        fn is_rich(&self, base_amount: i32) -> bool {
            println!("base amount {}", base_amount);
            match self {
                Currency::INR(x) | Currency::EURO(x) | Currency::USD(x) | Currency::YEN(x) => {
                    *x >= base_amount
                }
            }
        }
    }

    /// Pattern matching with enums is exhaustive and must cover all possible values.
    /// Matching can also be nested. To exhaust all remaining patterns use a variable like `others`
    /// or an unused variable `_others`.
    pub fn enum_patterns() {
        let curr = Some(Currency::USD(30));
        let amount = match curr {
            // this x is bind to the value of the enum on match
            Some(Currency::INR(x)) => {
                print!("Indian Rupee ");
                x
            }
            Some(currency) => match currency {
                Currency::EURO(x) | Currency::USD(x) => {
                    print!("USD or EURO ");
                    x
                }
                _others => {
                    println!("Some other currency");
                    0
                }
            },
            None => 0,
        };

        println!("amount is {amount}");
    }

    /// if-let is used to match against only one value, ignoring the rest.
    pub fn if_let() {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("the max value is {max}")
        }

        let curr = Currency::INR(10);
        if let Currency::INR(x) = curr {
            println!("This is Indian Rupee valued {x}")
        }

        let curr_list = [Currency::INR(12), Currency::USD(11), Currency::EURO(12)];
        let mut inr_counter = 0;

        for currency in curr_list {
            if let Currency::INR(amount) = currency {
                println!("INR amount is equal to {amount}");
            } else {
                inr_counter += 1;
            }

            // // match version
            // match currency {
            //     Currency::INR(x) => println!("INR amount in match block is {}", x),
            //     _ => inr_counter += 1,
            // }
        }

        println!("Non Indian currencies are {inr_counter}");
    }

    pub fn let_else() {
        let curr = Currency::INR(1000);
        let is_rich = find_if_rich(&curr);

        match is_rich {
            Some(e) => println!("{}", e),
            None => println!("Not valid currency to find rich"),
        }
    }

    fn find_if_rich(curr: &Currency) -> Option<String> {
        // rand number generator
        let mut rnd = rand::thread_rng();
        let n: f64 = rnd.r#gen();

        let &(Currency::INR(amount) | Currency::USD(amount) | Currency::EURO(amount)) = curr else {
            return None;
        };

        println!("amount is {amount}");

        if curr.is_rich(((amount as f64) * n) as i32 + 500) {
            Some(format!("This person is rich"))
        } else {
            Some(format!("This person is not rich"))
        }
    }
}

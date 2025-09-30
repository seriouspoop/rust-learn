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

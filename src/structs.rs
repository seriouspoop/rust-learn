
/// We used owned type `String` instead of `&str` because, 
/// we want each instance of this struct to own all of its data 
/// and for that data to be valid for as long as the entire struct is valid.
struct User {
    name: String,
    active: bool,
    email: String,
    age: u16
}

pub fn sample_struct() {
    let mut user1 = User {
        name: String::from("harshit"),
        active: true,
        email: String::from("email@harshit.com"),
        age: 21,
    };

    println!("Name of the user is {}", user1.name);

    user1.email = String::from("harshit@email.com");

    let user2 = build_user(
        String::from("Harshit"),
        String::from("harshit2@email.com"),
        22);

    println!("{}{}{}{}", user2.name, user2.active, user2.age, user2.email);
}

/// It uses field init shorthand,
/// to use this shorthand the variable and the field name must have same names.
fn build_user(name: String, email: String, age: u16) -> User {
    User {
        name,
        email,
        active: true,
        age,
    }
}

/// Struct update syntax lets you create new instances from other instances.
/// add the fields that are updated and spread the old instance.
/// 
/// IMPORTANT! -> now `user1` is no longer valid as the `String` values from `name` and `email` field
/// have been <i><b>moved</b></i> to `user2`
/// 
/// If no heap allocated values were borrowed from `user1` then it would still be valid.
/// Not moved values can still be used.
pub fn update_syntax() {
    let user1 = User {
        name: String::from("harshit"),
        active: true,
        email: String::from("email@harshit.com"),
        age: 21,
    };

    let user2 = User {
        email: String::from("user2@email.com"),
        ..user1
    };

    // println!("{}{} {}", user2.email, user2.age, user1.name); // ! borrow of moved value
    println!("{}{} {}", user2.email, user2.age, user1.email); // * unborrowed values are still valid
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Struct type is needed to destructuref
    let Color(r, g, b) = black;
    let Point(x, y, z) = origin;

    println!("R:{}, G:{}, B:{}", r, g, b);
    println!("X:{}, Y:{}, Z:{}", x, y, z);

}

#[derive(Debug)]
struct AlwaysEqual;

/// Unit type struct behave like unit value `()`, use them if you want to have some trait on the type
/// but doesn't have any fields for the type.
pub fn unit_structs() {
    let num = AlwaysEqual; 
    println!("{num:?}");
    dbg!(&num);
}

struct Rect {
    width: u32,
    heigth: u32,
}

/// Within an impl block, the type Self is an alias for the type that the impl block is for
impl Rect {
    fn area(&self) -> u32 {
        self.width*self.heigth
    }
}

pub fn methods(){
    let rect1 = Rect {
        width: 115,
        heigth: 230,
    };

    println!("area is {}", rect1.area())
}
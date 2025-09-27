use std::vec;

/// Primitives are basic types like `int`
pub fn primitives() {
    //* Data Types
    let a = 1;
    let mut x: i8= 10; // by default all variables in rust are immutable
    //* types :- i8..i128, f16..f128, u8..u128, char
    
    // overflows i8 limit -128..127
    // for _i in 0..1000{
        //     _x += 100
        // }
        
    println!("Hello, world! {x} today's sum is {a}");
    x = 12;
    println!("New x = {x}");

    // shadowing
    let s = 3;
    let s = s + 2;
    {
        let s = s + 10;
        println!("inscope s = {}", s);
    }
    println!("outscope x = {}", s);
}

pub fn strings() {
    //* Strings
    let name = "Harshit";
    let greeting = String::from("Good morning");

    let char1 = greeting.chars().nth(0);

    println!("{greeting}, {name}");

    // ! This doesn't work as rust analyzes that there might be a character or migth not be like nth(1000).
    //println!("{greeting}, {name}, 0th char is {char1}");

    // pattern matching can be used to resolve options
    match char1 {
        Some(c) => {println!("Char1 is {c}");}
        None => println!("Char1 not found")
    }

    let mut str = String::from("Harshit");
    println!("Capacity: {}, Length: {}, Pointer: {:p}", str.capacity(), str.len(), str.as_ptr());
    str.push_str(" is learning Rust");
    println!("Capacity: {}, Length: {}, Pointer: {:p}", str.capacity(), str.len(), str.as_ptr());
}


pub fn arrays() {
    /*
    *   Arrays:
        1. array with element type and static size
        2. 2d array with element type and static size
        3. mutable array with element type and static size
        4. array with element type dynamic size at initialization -> slices
        5. array with same elements and static size
    */

    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2: [[i32; 6]; 2] = [
        [1, 2, 3, 4, 5, 6],
        [9, 8, 7, 6, 5, 4]
    ];
    let mut arr3: [i8; 3] = [1, 2, 3];
    arr3[0] = 10;

    let slice4: &[i32] = &arr1[0..2]; // references sections of array
    let arr5 = [4; 5];
    println!("{arr1:?}\n{arr2:?}\n{arr3:?}\n{slice4:?}\n{arr5:?}")
}

pub fn tuples(){
    let tup: (i8, char, String) = (8, 'i', String::from("value"));

    let (sno, ch, str) = tup;

    println!("{}, {}, {}", sno, ch, str);

    let serial_no = tup.0;
    let character = tup.1;
    // ! reference moved to str at line 63
    // let string_value = tup.2;

    println!("{serial_no}, {character}");

    let arr_tup: [(i32, &str, bool); 2] = [(1, "harshit", true), (2, "harshit2", true)];
    println!("{}", arr_tup[0].1);
}

pub fn vectors() {
    /*   
        A vector is a similar collection type provided by the standard library
        that is allowed to grow or shrink in size
        because its contents live on the heap.
    */
    let mut vec = vec![1, 2, 3, 4, 5, 6];

    println!("Old vector = {vec:?}");
    vec.push(29); // ! due to heap allocation it is dynamically resizable
    println!("New Vector = {vec:?}");
}
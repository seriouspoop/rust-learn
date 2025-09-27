/// Using x before y is not allowed now.
/// According to `XOR mutability` rule
/// - Many immutable references to the same data, OR
/// - Exactly one mutable reference to that data
/// 
/// Here at `println!("{x}")` we are trying to use immutable reference of x,
/// whilst y already has an mutable reference.
/// 
/// This feature of Rust, makes it safe againt race conditions. And solve problems like
/// - read after write
/// - write after read
pub fn borrowing() {
    let mut x = 5;
    let y = &mut x; // ! y has borrowed mutable reference of x

    *y = 10;

    // println!("{x}"); 
    println!("{y}");
    println!("{x}"); // * now since y is out of scope and is being dropped, we can use x
}

/// Move is a operation/jargon that is performed on the values that needs to be allocated
/// For example: `String`
///     - These string types are stored in heap
///     - A stack frame is created for {length, capacity, pointer}
///     - Pointer points to the address assigned by the memory allocator in the heap
/// 
/// With code like:
/// ```
/// let s = String::from("Hello");
/// let y = s;
/// ```
/// another stack frame for `y` is copied from `s`
/// resulting in, same pointer reference to the memory in heap.
/// This for of copying is known as `shallow copy`.
/// Rust on copying invalidates the first variable `s` hence, the term `shallow copy`
/// be replaced by `move`.
///     1. As `s` becomes invalid, Rust won't call drop on both the variables when they go out of scope.
///     1. This is essentially transfering the onwership of the memory in heap (pointer) from one variable to another.
/// This is how Rust solves the `double free` problem.
pub fn moving() {
    let s = String::from("Hello");
    let y = s;

    // drop(y);
    println!("{y}");
    // println!("{s}"); // ! to solve this ownership/move issue we can borrow the value from s or clone it or tranfer back the ownership after use.

    // 1. cloning
    let c = y.clone();
    println!("{c}, {y}");

    // 2. borrowing
    let z = &y;
    println!("{z}, {y}");

    // 3. transfer back
    println!("{y}");
    let s = y;
    println!("{s}")
}

/// The action of creating a reference is called borrowing.
/// References are immutable by default.
/// In Rust, by contrast, the compiler guarantees that references will never be dangling references
pub fn references() {
    let s = String::from("value");
    let s1 = get_length(&s);

    println!("{s1} is the size of the referenced/borrowed string")
}

fn get_length(s: &String) -> usize {
    s.len()
}
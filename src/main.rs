fn main() {
    let a = 1;
    println!("Hello, world! {} today's sum is {}", a, sum(1, 12));
}

fn sum (a: i32, b: i32) -> i32 {
    return a+b
}
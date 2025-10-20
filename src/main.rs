



fn main() {
    let s1 = String::from("abc");
    let s2 = &s1;           // borrow
    let s3 = s1.clone();    // new copy

    println!("*s2 = {}", *s2); // deref reference to String
    println!("s3 = {}", s3);
}
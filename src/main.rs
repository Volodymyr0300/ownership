fn main() {

    /*
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
     */

    /* 
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];
    println!("slice: {}", slice);
    */

    /* 
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    println!("len: {}, slice: {}", len, slice);
    */

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];
    println!("len: {}, slice: {}", len, slice);
}

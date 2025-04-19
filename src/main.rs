fn main() { 
    /*
                            // s is not valid here; it's not yet declared
    let s = "hello";  // s is valid from this point forward
                            // do stuff with s
     */

    /*
    let _s1 = "hello"; // &str
    let mut s2 = String::from("hello"); // String

    // s1.push_str(" world"); // Error: `&str` is immutable
    s2.push_str(" world"); // Works: `String` is mutable
    println!("{}", s2); // Output: "hello world"
     */

    /*
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // this will print `hello, world!`
    */

    /*
    {
        let _t = String::from("hello"); // t is valid from this point forward
         // this scope is now over, and t is no
        // longer valid
    }
    */

    /*
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y); // Output: x: 5, y: 5

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{} {}", s1, s2); // Error: s1 is no longer valid
     */

    
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Clone s1 into s2
    println!("s1 = {}, s2 = {}", s1, s2);
    
}

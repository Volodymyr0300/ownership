// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
    
//     fn calculate_length(s: &String) -> usize { // s is a reference to a String
//         s.len()
//     }
//     // Here, s goes out of scope. But because it does not have ownership of
//     // what it refers to, nothing happens.
// }


// fn main() {
//     let s = String::from("hello");
//     change(&s);
    
//     fn change(some_string: &String) {
//     some_string.push_str(", world");
//     }
// }

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    /* 
    let mut s = String::from("hello");
    {
    let r1 = &mut s;
    }
   
     // r1 goes out of scope here, so we can make a new reference with no
    // problems.
    let r2 = &mut s;
    println!("{}", r2);
    */

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    println!("{}, {}, and {}", r1, r2, r3);
}

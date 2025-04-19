fn main() {
    // let x = String::from("hello world");
    // first_word(&x);
    // println!("{}", first_word(&x));

    let mut s = String::from("hello world");
    
    let word = first_word(&s); // word will get the value 5
    
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally
    // invalid!
}
    
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // println!("{:?}", bytes);
    // println!("{}", s.as_bytes().len());
    
    for (i, &item) in bytes.iter().enumerate() {
        // println!("Index: {}, Value: {}", i, item);
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}
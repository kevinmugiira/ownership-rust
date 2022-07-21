pub fn owner() {

    //creating a String from a string literal
    let mut s = String::from("Wajackoyah"); 

    //string literal
    let ss = "is here.";

    //pushing to the String
    s.push_str(" the fifth");

    //printing out the String
    println!("Our String: {}", s);
    println!("Combining the String with the string literal: '{} {}'",s , ss);

    let s1 = String::from("hello");

    //utilizing clone() to make a deep copy, ie: of the heap
    let s2 = s1.clone();

    println!("value for s2 is: {}", s1);

    let s3 = gives_ownership();

    let s4 = String::from("wembe");

    let s5 = takes_and_gives_back(s4);

    println!("s5: {}", s5);

    let ab_string = String::from("watu wa chamaa");
    let (ss2, len) = calculate_length(ab_string);
    println!("The length of '{}', is: {}", ss2, len);

}

pub fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

pub fn makes_copy(some_int: u8) {
    println!("{}", some_int);
}

pub fn gives_ownership() -> String {
    let a_string = String::from("kamba");
    a_string
}

pub fn takes_and_gives_back(b_string: String) -> String {
    b_string
}

pub fn calculate_length(s: String) -> (String, usize){
    let length = s.len();

    (s, length)
}
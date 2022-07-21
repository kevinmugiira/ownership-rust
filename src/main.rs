
mod ownership;


fn main() {

    ownership::owner();

    let my_string = String::from("a good word");
    ownership::takes_ownership(my_string);

    let x = 5;
    ownership::makes_copy(x);
   
    
}

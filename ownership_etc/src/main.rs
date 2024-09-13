fn main(){

    let mut s1 = String::from("deepanshu");
    s1 = do_something(s2:&s1); // s1 is borrowed by do_something
    println!("The number is : {}",s1);
}

fn do_something(s2:&String){
    println!("{}",s2); // s2 owns the value
}

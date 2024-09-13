

fn main(){
    let bigger  = largest(1,2);
    let bigger_char = largest('a','b');

    println!("The bigger number is : {}",bigger);
    println!("The bigger char is : {}",bigger_char);
}

fn largest<T: std::cmp::PartialOrd>(a:T,b:T)->T{
    if a>b{
        return a;
    }else{
        return b;
    }
}


// Generics- > A way to write code that works with multiple types without duplicating code

// generics allow us to define functions, structs, enums, and methods that work with any type


/*
Generics in Rust are similar to templates in C++ and generics in Java

Generics are useful when we want to write a function that can work with multiple types

For example, we want to write a function that takes two arguments and returns the bigger of the two arguments

<T: std::cmp::PartialOrd> is a trait bound that specifies that the type T must implement the PartialOrd trait

PartialOrd trait is implemented by types that can be compared for ordering

*/

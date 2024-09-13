

fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(6);

    // println!("{}",vec); // throw an error as we can't print a vector directly because it doesn't implement the Display trait

    // To print the vector we can use the debug format specifier - > {:?}

    println!("{:?}",vec); // [1,2,3]

    //testing the function

    println!("{:?}",even_vector(vec));

    let nums = vec![1,2,3]; // type will be inferred as Vec<i32> automatically


}

//write a function that takes a vector as an input and returns a vector with even elements


fn even_vector(vec:Vec<i32>)->Vec<i32>{
    let mut new_vec = Vec::new();

    for i in vec{
        if i%2==0{
            new_vec.push(i);
        }
    }

    return new_vec;
}

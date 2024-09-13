use::std::sync::mpsc;
use::std::thread:: {self,spawn};

fn main(){

    let (tx,rx) = mpsc::channel();

    for i in  0..10{
        let producer = tx.clone();
        spawn(move || {
            let mut sum:u64 = 0;
            for j in i*1000000000 .. (i+1*1000000000) -1 {
                sum+=j;
            }
            producer.send(sum)
        });
    }

    drop(tx);


    let mut final_sum:u64 = 0;
    for val in rx{
        println!("Received value from thread");
        final_sum+=val;
    }

    println!("{}",final_sum);
}


// In the above code, we are creating a channel using mpsc::channel() and then we are creating 10 threads and each thread is calculating the sum of numbers from i*1000000000 to (i+1)*1000000000 -1. We are then sending the sum to the main thread using the producer.send(sum) method. We are then dropping the producer as we don't need it anymore. We are then receiving the sum from the threads and adding it to the final_sum variable. Finally, we are printing the final_sum.

//Macros in Rust

/*
Macros in Rust are a way to generate code at compile time. Macros are defined using the macro_rules! keyword. Macros can take any number of arguments and can generate any number of lines of code. Macros are used to reduce code duplication and to make the code more readable.

For example, we can define a macro that takes two arguments and generates a function that adds the two arguments. The macro definition would look like this:

macro_rules! add {
    ($a:expr,$b:expr) => {
        fn add(a:i32,b:i32)->i32{
            return a+b;
        }
    }
}

in simple words macros are the way to write code that writes code for us

*/

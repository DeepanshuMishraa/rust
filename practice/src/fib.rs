// Rust program to find the nth Fibonacci number using recursion

fn main(){
    let n:i32 = 10;
    let result:i32 = fib(n);
    println!("The {}th Fibonacci number is {}", n, result); 
}


fn fib(n:i32)-> i32{
    if n== 0 || n==1{
        return n;
    }
    fib(n-1) + fib(n-2)
}

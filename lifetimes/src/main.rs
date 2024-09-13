

fn main(){

    let a = String::from("Deepanshu");
    let b = String::from("Deepanshu Sharma");

    let result = longest(&a,&b);

    println!("{}",result);

    //error as the lifetime of the reference is not known

    //why actually error because the b is not in the scope of the reference returned by the longest function so it will throw an error as the reference is not valid anymore as the value it was pointing to has been deallocated  

}

fn longest(a:&str,b:&str)->&str{
    if a.len() > b.len(){
        return a;
    }else{
        return b;
    }
}

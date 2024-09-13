// options and result enum -> enum Option<T> { Some(T), None } and enum Result<T, E> { Ok(T), Err(E) }

// we can use Option<T> to represent a value that may or may not be present
// we can use Result<T, E> to represent a value that may be present or an error

fn main(){

    let index = find_first_a(String::from("harkirat"));

    match index{
        Some(value )=>{
            println!("Index of first a is : {}",value);
        }
        None=>{
            println!("No a found");
        }
    }
}

enum customOption{
    Some(i32),
    None
}


fn find_first_a(s:String) -> Option<i32>{
    for(index,char) in s.chars().enumerate(){
        if char == 'a'{
            return Some(index as i32);
        }
    }
    return None;
}

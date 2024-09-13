struct User{
    first_name:String,
    last_name:String,
    age:i32
}

//structs are similar to classes in other languages

// structs are used to create custom data types

// implementing a struct

struct Rect{
    width:i32,
    height:i32
}

impl Rect{

    fn area(&self)->i32{
        self.width*self.height
    }

    fn perimeter(&self)->i32{
        2*(self.width+self.height)
    }

    fn debug() -> i32{
        return 1;
    } // this is a static method-> similar to class methods in other languages basically its a method that can be called without creating an instance of the struct
}



fn main() {
    let user = User{
        first_name:String::from("harkirat"),
        last_name:String::from("Singh"),
        age:22
    };

    let rect = Rect{
        width:10,
        height:20
    };

    println!("Area of rectangle is : {}",rect.area());
    println!("Perimeter of rectangle is : {}",rect.perimeter());
    println!("Debug of rectangle is : {}",Rect::debug());

    println!("{}",user.first_name);


}

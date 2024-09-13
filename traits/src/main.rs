trait Summary{
    fn summarize(&self)->String{
        return String::from("Hi there");
    }
}


struct User{
    name:String,
    age:u32
}


impl Summary for User{
    fn summarize(&self)->String{
        return format!("Name : {} , Age : {}",self.name,self.age);
    }
}



fn main(){

    let user = User{
        name:String::from("Deepanshu"),
        age:21
    };

    println!("{}",user.summarize());


}

fn notify(u:impl Summary){
    println!("{}",u.summarize());
}

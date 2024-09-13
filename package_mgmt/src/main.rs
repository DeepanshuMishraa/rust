use chrono::Local;

fn main() {
    let now = Local::now();
    println!("Local time is : {}", now);
}

// cargo add chrono -> crates.io 

use clap::Parser;

#[derive(Parser,Debug)]
#[command(version,about,long_about = None)]

struct Args{
    #[arg(short,long)]
    operation:String,

    #[arg(short,long)]
    first:i32,
    #[arg(short,long)]
    second:i32,
}

fn main(){
    let args = Args::parse();


    let operation = args.operation;
    let first = args.first;
    let second = args.second;

  match operation.as_str() {
        "+" => println!("{} + {} = {}",first,second,first+second),
        "-" => println!("{} - {} = {}",first,second,first-second),
        "*" => println!("{} * {} = {}",first,second,first*second),
        "/" => println!("{} / {} = {}",first,second,first/second),
        _ => println!("Invalid operation"),
    }

}

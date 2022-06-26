use std::env::{args,Args};

fn main() {
    println!("Hello, world!"); 
    //  Getting the Command Line Arguments
    let mut arguments: Args=args();
    //Extracting Arguments as String 
    let first=arguments.nth(1).unwrap();
    let op=arguments.nth(0).unwrap().chars().next().unwrap();
    let second=arguments.nth(0).unwrap();
     println!("{} {} {} ",first,op,second);
}

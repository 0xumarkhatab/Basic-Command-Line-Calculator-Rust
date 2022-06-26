use std::env::{args,Args};

fn main() {
    println!("Hello, world!"); 
    //  Getting the Command Line Arguments
    let mut arguments: Args=args();
    //Extracting Arguments as String 
    let first=arguments.nth(1).unwrap();
    let op=arguments.nth(0).unwrap().chars().next().unwrap();
    let second=arguments.nth(0).unwrap();
     println!("Accepted Arguments = {} {} {} ",first,op,second);
     
    // Converting Arguments to Flot32
  let firstNumber  =first.parse::<f32>().unwrap();
  let secondNumber =second.parse::<f32>().unwrap();
    
}
// Performing Operations
fn operate(a:f32,b:f32,op:char)->f32{
match op {
  '+'=>a+b,
  '-'=>a-b,
  '/'=>a/b,
  'x'=>a*b,
  _=>0.0
  
}

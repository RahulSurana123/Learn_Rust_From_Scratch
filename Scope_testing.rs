const EXAMPLE_VARIABLE_WITH_GLOBAL_SCOPE: f64 = 9.9;  // giving type of variable is compulsory here

fn main() {
    let x = 9;
    {
        let x = 55;
        println!("x in the block : {}",x);
    }
    println!("x of block : {}, global variable value : {}",x,EXAMPLE_VARIABLE_WITH_GLOBAL_SCOPE);
    
    // Example of how mut vaiable can become imutable which traken care at assembly level by rust
    let mut a = 5;
    println!("a (mutable) : {}",a);
    a = 15;
    println!("a (to be converted to imutable) : {}",a);
    let a = a;
    // a = 30; // this statement will error out 
    
    // Shadow Variable example 
    
    let b: i32 = 5;
    println!("b (int) : {}",b);

    let b: f32 = 7.4;
    println!("b (float) : {}",b);
    
}

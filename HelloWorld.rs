
/*

Trying Out Different ways of Printing Hello World

*/

use std::fmt;

fn main() {
    
    // Hello World with new line 
    println!("Hello, world!");
    
    // Hello World without new line
    print!("Hello, world!");
    print!("\n");
    
    // Hello World as Arguments
    println!("{}","Hello World!");
    println!("{0} {1}","Hello","World!");   
    println!("{h} {w}",h="Hello",w="World!");
    
    // Alignment and fills
    println!("{:>17}","Hello World!");
    println!("{0:<7} {1:>9}","Hello","World!");
    println!("{h:-<8} {w:->10}",h="Hello",w="World!");

    // String in a Struct making formatter for it
    struct Structure(String);
    impl fmt::Display for Structure {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            
            write!(f, "{} ", self.0)
            // Uncomment to see warning and new type of output
            // write with new line just after printing variable
            // writeln!(f, "{}", self.0)

        }
    }
    println!("{} using a Structure",Structure(String::from("Hello World!")));

    // printing Some Number with precision
    println!("Number : {0} , Binary({0}) : {0:b} , Oct({0}) : {0:o}, Hex({0}) : {0:x}, HEX({0}) : {0:X} ", 43);
    println!("Number : {0:.7} ", 20.843);

}       
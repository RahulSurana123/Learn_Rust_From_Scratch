
/*

Making a "Guess" a game to know how many try it takes for you to guess the number

Motive behind this code is to know Inputs and how &mut(Mutable Data type) works 
also some conditional statements

*/

use std::io;

fn main() {
    
     let mut number = String::new();
     let mut guess = String::new();
     let mut count:i32 = 0;
     let mut num:i32;
     let mut num2:i32;
    // Hello World with new line 
    println!("Player 1 Please Enter a number(1 - 100) to be guessed: ");
    io::stdin().read_line(&mut number).expect("Failed to read line");
    number.pop();
    num = number.parse().expect("Reason");
    println!(" Number you choose is : {}  ",num);
    
    while num < 0 || num > 100 {
        number = String::new();
        println!("Please Enter a Valid Number in between (1 -100) inclusive :");
        io::stdin().read_line(&mut number).expect("Failed to read line");
        number.pop();
        num = number.parse().expect("Reason");
        println!(" Number you choose is : {}  ",num);
    }
    
    println!(" Player 2 Please  Make your guess :");
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    guess.pop();
    num2 = guess.parse().expect("Reason");
    println!(" Your Guess was : {}  ",num2);
    count += 1;
    while num2 != num {
        guess = String::new();
        print!(" Try Another Guess ");
        if num2 > num  {
            println!( " (Last Number was Too High) : ");
        }
        else{
            println!( " (Last Number was Too Low) : ");
        }
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess.pop();
        num2 = guess.parse().expect("Reason");
        println!(" Your Guess was : {}  ",num2);
        count+=1;
    }
    
    println!(" Yayhh ! you guess it correct number was : {} ", num);
    println!("Total Tries u took till now is : {}", count);
    
    
}       
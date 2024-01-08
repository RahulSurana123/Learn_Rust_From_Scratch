fn main(){

	// normal Addition / Substraction  
	// Note : 
	//      If data type of number not mention then its taken automatically
	//      If data type is mentioned as unsigned then operator overflow needs to be taken care by user
	//			i.e 1u32 - 2 will result in overflow as answer will be 
	//              -1u32 ( Not possible as unsigned data type cant be negative)
	println!(" 1 + 2 = {}", 1i32 + 2);
	println!(" 1 - 2 = {}", 1i32 - 2);

	// Normal Multiplication and Division
	// Note : 
	//       Scientific Notation is considered float in nature in Rust
	println!(" 1e4 * 29 = {}", 1e4 * 29f32);
	println!(" 1 / 2 = {}", 1 / 2);

	// Normal Logical Operator
	// Note : 
	//       Scientific Notation is considered float in nature in Rust
	//       Anything which is not a bool givens error when used within logical operators
	println!(" true && false = {}", true && false);
	println!(" true || false = {}", true || false);
	println!(" true  ^ false = {}", true ^ false);
	println!(" true  ^ true = {}", true ^ true);
	
	// Bitwise Operators
	println!(" 10 & 2 = {}", 10 & 2);
	println!(" 14 | 1 = {}", 14 | 1);
	println!(" 24  ^ 13 = {}", 24 ^ 13);
	println!(" 1 << 4 = {}", 1 << 4);
	println!(" 5 >> 1 = {}", 5 >> 1);
	println!(" 43 << 2 = {}", 43 << 2);
	println!(" 3 >> 7 = {}", 3 >> 7);

}
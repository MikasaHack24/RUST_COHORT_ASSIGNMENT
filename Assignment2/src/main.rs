/* This print_name function takes in one argument
name and returns its name value  */
fn print_name(name:&str){
    println!("My name is {}", name);
}

/*This print_matno function takes in one argument
 mat_no and returns the value */

fn print_matno(mat_no:&str){
    println!("My MatNO is {}", mat_no);
}


/*This print emoji function takes in an argument emoji 
and returns the value */
fn print_my_emoji(emoji:char){
    println!("A smiley face emoji{}", emoji);
}


/*This div_num function takes in two arguments with parameters
num1 and num2  and rurns their quotient */
fn div_num(num1:f64, num2:f64)->f64{
    let divide_num:f64= num1/num2;
    return divide_num;
}



/*This is the main function. It does the following:
*It calls the print_name function and prints the result to the 
console 

*It calls the print_matno function and prints the result to the 
console 

*It calls the print_my_emoj function and prints the result to the 
console

* It calls the div_num function
with arguments(5.0 and 2.0) and prints results to the console
 */
fn main(){
    let name = "Igba Gift Emike";
print_name(name);
let mat_no = "ENG2102771";
   print_matno(mat_no);

   let my_emoj= '\u{1F600}';
   print_my_emoji(my_emoj);

   let num1:f64 = 5.0;
   let num2:f64 = 2.0;
   let solution = div_num(num1, num2);
   println!("The division of {} and {} is {}", num1, num2, solution);
   
    
}
    
    


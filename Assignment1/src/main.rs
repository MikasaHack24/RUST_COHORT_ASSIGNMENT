/*This program prints out my name
*Mat NO
*My Favourite Emoji
*and the division of two numbers to 
give a fractional value  */
fn main(){
    let my_name:&str = "Igba Gift Emike";
    println!("My name is {}", my_name);

    let my_matno:&str = "ENG2102771";
    println!("My mat no is {}", my_matno);

    let fav_emoji:&str = "\u{1f600}";
    println!("My favourite emoji {}", fav_emoji);

    let num1:f64 = 5.0;
    let num2:f64 = 2.0;
    let div_num:f64 = num1/num2;
    println!("The division of {} and {} is {}", num1, num2, div_num);
}
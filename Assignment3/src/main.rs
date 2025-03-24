/*This program does the following
*prints the sum of the scores
*prints the average of the scores
*prints the highest score
*prints the scores of the student that failed
*prints the scores of the students that passed
 */
fn main(){
    let scores:[i32;10] = [23, 54, 45, 76, 89, 99, 34, 67, 33, 43];
    let total_sum:i32 = scores.iter().sum();
    println!("The sum of the scores is {}", total_sum);

    let total_avg:i32 = scores.len().try_into().unwrap();
    println!("The average score is {}", total_avg);

    let max_num = scores.iter().max().unwrap();
    println!("The score of the highest scoring student is {}", max_num);

    println!("the scores of students that failed are listed below:");
    for numbers in scores.iter(){
        if *numbers<50{
            println!("{}", numbers);
            
        }
        
    }

    println!("The scores of students who passed are:");
    for number in scores.iter(){
        if *number > 50{
            println!("{}", number);
        }
    }
}

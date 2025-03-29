/*A simple rust code that implements the function of a calculator
and also calculates the cgpa of a student */


use std::io;

fn main() {
    loop {
        println!("Choose an operation:");
        println!("1. Basic Calculator");
        println!("2. CGPA Calculator");
        println!("3. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: i32 = choice.trim().parse().expect("Invalid choice");
        match choice {
            1 => basic_calculator(),
            2 => cgpa_calculator(),
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn basic_calculator() {
    println!("Choose an operation:");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: i32 = choice.trim().parse().expect("Invalid choice");
    println!("Enter the number of operands:");
    let mut num_operands = String::new();
    io::stdin().read_line(&mut num_operands).expect("Failed to read line");
    let num_operands: i32 = num_operands.trim().parse().expect("Invalid number of operands");
    let mut operands: Vec<f64> = Vec::new();

    let num_operands_usize = num_operands as usize;
    for _i in 0..num_operands_usize {
        println!("Enter an operand:");
        let mut operand = String::new();
        io::stdin().read_line(&mut operand).expect("Failed to read line");
        let operand: f64 = operand.trim().parse().expect("Invalid operand");
        operands.push(operand);
    }

    match choice {
        1 => println!("Result: {}", operands.iter().sum::<f64>()),
        2 => {
            let mut result = operands[0];
            for &operand in &operands[1..] {
                result -= operand;
            }
            println!("Result: {}", result);
        }
        3 => {
            let mut result = 1.0;
            for &operand in &operands {
                result *= operand;
            }
            println!("Result: {}", result);
        }
        4 => {
            let mut result = operands[0];
            for &operand in &operands[1..] {
                if operand == 0.0 {
                    println!("Error: Division by zero!");
                    return;
                }
                result /= operand;
            }
            println!("Result: {}", result);
        }
        _ => println!("Invalid choice"),
    }
}

fn cgpa_calculator() {
    println!("Enter the number of courses:");
    let mut num_courses = String::new();
    io::stdin().read_line(&mut num_courses).expect("Failed to read line");
    let num_courses: i32 = num_courses.trim().parse().expect("Invalid number of courses");
    let mut total_credits = 0.0;
    let mut total_points = 0.0;

    let num_courses_usize = num_courses as usize;
    for _i in 0..num_courses_usize {
        println!("Enter grade (A, B, C, D, F) and credits for a course:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let grade = parts.next().unwrap();
        let credits: f64 = parts.next().unwrap().parse().expect("Invalid credits");
        let points = match grade.to_uppercase().as_str() {
            "A" => 4.0 * credits,
            "B" => 3.0 * credits,
            "C" => 2.0 * credits,
            "D" => 1.0 * credits,
            "F" => 0.0 * credits,
            _ => {
                println!("Invalid grade");
                continue;
            }
        };
        total_credits += credits;
        total_points += points;
    }

    if total_credits == 0.0 {
        println!("No courses entered");
    } else {
        let cgpa = total_points / total_credits;
        println!("CGPA: {:.2}", cgpa);
    }
}
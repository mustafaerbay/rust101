use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the number of variables to sum:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please enter a valid integer");

    let mut sum = 0;
    for i in 1..=n {
        let mut variable_input = String::new();
        println!("Enter variable {}: ", i);
        io::stdin().read_line(&mut variable_input).expect("Failed to read line");
        let variable: i32 = variable_input.trim().parse().expect("Please enter a valid integer");
        sum += variable;
    }

    println!("The sum of the variables is: {}", sum);
}

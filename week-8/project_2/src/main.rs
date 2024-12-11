use std::io;
fn main() {
    println!("WELCOME TO ERNEST AND YOUNG GLOBAL LIMITED\nWELCOME TO  YOUR ONLINE INTERVIEW\n");
    println!("What is your name?");

    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Not valid");
   
    println!("How many years of work experience do you have?");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Not valid");
    let years:i32 = input2.trim().parse().expect("Invalid");

    if years > 10 {
        println!("Congrats {} due to your {} years of experience you have been chosen for the job",input1,years);
    }
    else {
        println!("Sorry to inform you {} but you need at least 10 years of work experience for this job",input1);
    }
}

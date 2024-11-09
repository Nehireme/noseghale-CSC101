use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("How old are you?");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:i32 = input1.trim().parse().expect("Not valid number");

    println!("You are an experienced worker:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience:bool = input2.trim().parse().expect("Not a alid nummber");


    if age >40 && experience == true{
        println!("Incentive is 1,560,000");
    } else if age <30 && age<40 && experience == true {
        println!("Incentive is 1,480,000");
    } else if age<28 && experience == true {
        println!("Incentive is 1,300,000");
    } else if experience == false {
        println!("Incentive is 100,000");
    } 

}
use std::io;

fn main() {
    println!("\nStudent Information Management System");

    //input name
    println!("\nPlease enter your name.");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("DOUMA GONNA GET YOU");
    println!("Your name is {}", name);

    //input age
    println!("\nEnter your age.");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("MUZAN IS ON THE WAY");
    let age:i32 = age.trim().parse().expect("This is not an integer");
    println!("Your age is {}", age);
}

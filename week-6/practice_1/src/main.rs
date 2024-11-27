fn main() {
    let name = "Nehi Oseghale";
    let uni:&str = "Pan Atlantic Uiversity";
    let addr:&str = "Km 52 Lekk-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}", uni, addr);

    let department:&'static str = "Software Engineering";
    let school:&'static str = "School of Science and Technology";
    println!("Department; {}, \nSchool: {}", department, school);
}

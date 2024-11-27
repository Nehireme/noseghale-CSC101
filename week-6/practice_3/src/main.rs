fn main() {
    let name1 = "Nehireme Oseghale";
    println!("My name is {}", name1);

    let name2 = name1.replace("Nehireme","Nicole");
    println!("You can also call me {}",name2);
    let faculty = "Faculty of Science and Technology";

    let school = faculty.replace("Faculty","School");
    println!("I am a student of the {}", school);
}

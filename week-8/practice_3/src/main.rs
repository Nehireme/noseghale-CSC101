fn value(n:Option<&char>)
{
    println!("Element of vector {:?}",n);
}
fn main() {
    let v = vec!['A','N','N','A','B','E','T','H'];

    let mut input1 = String::new();
    println!("\nEnter an index value between (0-7)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read");
    let index:usize = input1.trim().parse().expect("Invalid input");

    let ch:Option<&char> = v.get(index);
    value(ch);
}
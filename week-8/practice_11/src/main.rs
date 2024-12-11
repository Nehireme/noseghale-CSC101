fn main() {
    let numbers = [1,2,3,4,5];
    println!("Orignal array = {:?}",numbers);

    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}",slice1);

    let slice2 = &numbers[..3];
    println!("Index 0 to index 3 sliced = {:?}",slice2);

    let slice3 = &numbers[2..];
    println!("Index 2 to index 5 sliced = {:?}",slice3);

    let slice4 = &numbers[..];
    println!("Index 0 to index 5 sliced = {:?}",slice4);
}
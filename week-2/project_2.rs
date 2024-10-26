fn main() {
	let z:f64 = 2.00;
	let y:f64 = 1.00;
	let x:f64 = 3.00;
	let w:f64 = 3.00;
	let v:f64 = 1.00;
	let a:f64 = 450000.00;
	let b:f64 = 1500000.00;
	let c:f64 = 750000.00;
	let d:f64 = 2850000.00;
	let e:f64 = 250000.00;

	//sum 
	let sum = (z*a)+(y*b)+(x*c)+(w*d)+(v*e);
	println!("Sum of records is {}", sum);
	//average 
	let average = sum/10.0;
	println!("Average of records is {}", average)
}
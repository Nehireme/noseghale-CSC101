fn main() {
	let p:f64 = 510000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

//complex interest
let a = p * (1.0-(r/100.0)) .powf(n);
let dep = p-a;
println!("Depreciation is {}", dep);
}
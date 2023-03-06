fn gcd(mut a: u64, mut b: u64) -> u64 {
	while a != 0 && b != 0 {
		(a, b) = (b, a % b);
	}
	if a == 0 { b } 
	else { a }
}
fn lcm(a: u64, b: u64) -> u64 {
	return (a*b)/gcd(a,b);
}
fn main() {
	// 1 is trivial
	let mut res: u64 = 1;
	for i in 2..=20 {
		res = lcm(res, i);
	}
	println!("{res}");
}

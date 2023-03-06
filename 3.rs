fn main() {
	let mut maxfactor: u64 = 1; // 1 so that rustc doesn't complain
	let mut n: u64 = 600851475143;
	// Precision errors with large numbers, but that probably won't apply here
	for i in 2..=(n as f64).sqrt() as u64 {
		if n % i == 0 {
			maxfactor = i;
			while n % i == 0{
				n /= i;
			}
		}
	}
	if n > 1 {
		maxfactor = n;
	}
	println!("{maxfactor}");
}

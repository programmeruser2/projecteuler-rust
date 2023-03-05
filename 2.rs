fn main() {
	let mut sum: u64 = 0;
	let mut a1: u64 = 1;
	let mut a2: u64 = 2;
	while a2 < 4_000_000 {
		(a1, a2) = (a2, a1 + a2);
		if a1 % 2 == 0 {
			sum += a1;
		}
	}
	println!("{sum}");
}

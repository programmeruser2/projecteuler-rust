fn main() {
	let mut sum: u32 = 0;
	for i in 1..1000 {
		if i % 3 == 0 || i % 5 == 0 {
			sum += i;
		}
	}
	println!("{sum}");
}

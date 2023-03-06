fn main() {
	// By the prime number theorem, the 10001th prime is probably less than ~117000
	const BOUND: usize = 117000;
	let mut nth: u64 = 0; // nth prime
	// set to 1 so that rustc stops complaining about possibly being uninitialized
	let mut nth_prime: u64 = 1; 
	// +1 so that we can assign to array without subtracting number by 1
	let mut sieve: [bool; BOUND+1] = [false; BOUND+1 as usize];
	// Cross out all composites
	let sqrt_bound: usize = (BOUND as f64).sqrt() as usize;
	for i in 2..=sqrt_bound {
		if sieve[i] {
			continue;
		}
		let mut j: usize = i*2;
		while j < BOUND {
			sieve[j] = true;
			j += i;
		}
		nth += 1;
		nth_prime = i as u64;
	}
	// Keep iterating until we find the 10,001th prime
	let mut i: usize = sqrt_bound;
	while nth < 10_001 && i < BOUND {
		if !sieve[i] {
			nth += 1;
			nth_prime = i as u64;
		}
		i += 1;
	}
	if nth < 10_001 {
		println!("couldn't get to 10,001th prime, only got to {nth} prime");
	} else {
		println!("{nth_prime}");
	}
}

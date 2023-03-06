fn is_palindrome(n: u32) -> bool {
	// Only contains 0-9 so we can just use the bytes
	let s: String = n.to_string(); // so that it isn't a temporary variable
	// and rustc stops complaining
	let b: &[u8] = s.as_bytes();
	let middle: usize = b.len() / 2;
	for i in 0..middle { 
		if b[b.len()-1-i] != b[i] {
			return false;
		}
	}
	true
}
fn main() {
	// 101 * 101 = 10201
	let mut max: u32 = 101 * 101; 
	for i in 100..=999 {
		for j in 100..=999 {
			let prod: u32 = i*j;
			if prod > max && is_palindrome(prod) {
				max = prod;
			}
		}
	}
	println!("{max}");
}

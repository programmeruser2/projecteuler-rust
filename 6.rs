fn main() {
	const SUM_SQUARES: u32 = (100*(100+1)*(2*100+1))/6;
	const SUM: u32 = (100*(100+1))/2;
	const SQUARE_SUM: u32 = SUM*SUM;
	// subtracting the other way around will make rustc complain
	const DIFF: u32 = SQUARE_SUM - SUM_SQUARES; 
	println!("{DIFF}");
}

%: %.rs
	rustc $< -o bin/$@

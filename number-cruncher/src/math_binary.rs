pub fn decimal_to_binary(a: i32) {
    let mut binary = [0; 32];
	let mut i = 0;
    let mut n = a;
	while n > 0 {
		binary[i] = n % 2;
		n /= 2;
		i += 1;
	}

    print!("BINARY: ");
	for j in (0..i).rev() {
		print!("{}", binary[j]);
		if j % 4 == 0 && j != 0 {
			print!(" ")
		}
	}
}
